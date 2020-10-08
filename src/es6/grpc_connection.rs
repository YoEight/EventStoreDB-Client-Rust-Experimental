use crate::es6::gossip::{Gossip, MemberInfo, VNodeState};
use crate::es6::types::Endpoint;
use crate::{Credentials, DnsClusterSettings, Either, NodePreference};
use futures::channel::mpsc::UnboundedSender;
use futures::channel::oneshot;
use futures::stream::StreamExt;
use futures::{Future, SinkExt};
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::combinator::{all_consuming, complete, opt};
use nom::error::ErrorKind;
use nom::lib::std::fmt::Formatter;
use nom::{bytes::complete::tag, IResult};
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{RngCore, SeedableRng};
use serde::de::Visitor;
use serde::{Deserializer, Serializer};
use std::cmp::Ordering;
use std::str::FromStr;
use std::time::Duration;
use tonic::transport::Channel;
use tonic::Status;
use uuid::Uuid;

struct NoVerification;

impl rustls::ServerCertVerifier for NoVerification {
    fn verify_server_cert(
        &self,
        _roots: &rustls::RootCertStore,
        _presented_certs: &[rustls::Certificate],
        _dns_name: webpki::DNSNameRef,
        _ocsp_response: &[u8],
    ) -> Result<rustls::ServerCertVerified, rustls::TLSError> {
        Ok(rustls::ServerCertVerified::assertion())
    }
}

#[test]
fn test_connection_string() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Mockup {
        string: String,
        expected: ConnectionSettings,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Mockups {
        mockups: Vec<Mockup>,
    }

    let mockups = std::fs::read("tests/fixtures/connection_string/mockups.toml").unwrap();
    let fixtures: Mockups = toml::from_slice(mockups.as_slice()).unwrap();

    for mockup in fixtures.mockups {
        match mockup.string.as_str().parse::<ConnectionSettings>() {
            Ok(current) => assert_eq!(
                current, mockup.expected,
                "Failed parsing [{}]",
                mockup.string
            ),

            Err(e) => panic!(format!("Failed parsing [{}]: {:?}", mockup.string, e)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConnectionSettingsParseError {
    input: String,
}

impl std::fmt::Display for ConnectionSettingsParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionSettings parsing error: {}", self.input)
    }
}

impl std::error::Error for ConnectionSettingsParseError {}

struct DurationVisitor;

impl<'de> Visitor<'de> for DurationVisitor {
    type Value = Duration;

    fn expecting(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "duration in milliseconds")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Duration::from_millis(v as u64))
    }
}

fn serialize_duration<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u64(value.as_millis() as u64)
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DurationVisitor)
}

fn default_max_discover_attempts() -> usize {
    ConnectionSettings::default().max_discover_attempts
}

fn default_discovery_interval() -> Duration {
    ConnectionSettings::default().discovery_interval
}

fn default_gossip_timeout() -> Duration {
    ConnectionSettings::default().gossip_timeout
}

fn default_preference() -> NodePreference {
    ConnectionSettings::default().preference
}

fn default_secure() -> bool {
    ConnectionSettings::default().secure
}

fn default_tls_verify_cert() -> bool {
    ConnectionSettings::default().tls_verify_cert
}

fn default_throw_on_append_failure() -> bool {
    ConnectionSettings::default().throw_on_append_failure
}

/// Gathers all the settings related to a gRPC connection with an EventStoreDB database.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectionSettings {
    #[serde(default)]
    pub(crate) dns_discover: bool,
    #[serde(default)]
    pub(crate) hosts: Vec<Endpoint>,
    #[serde(default = "default_max_discover_attempts")]
    pub(crate) max_discover_attempts: usize,
    #[serde(
        default = "default_discovery_interval",
        serialize_with = "serialize_duration",
        deserialize_with = "deserialize_duration"
    )]
    pub(crate) discovery_interval: Duration,
    #[serde(
        default = "default_gossip_timeout",
        serialize_with = "serialize_duration",
        deserialize_with = "deserialize_duration"
    )]
    pub(crate) gossip_timeout: Duration,
    #[serde(default = "default_preference")]
    pub(crate) preference: NodePreference,
    #[serde(default = "default_secure")]
    pub(crate) secure: bool,
    #[serde(default = "default_tls_verify_cert")]
    pub(crate) tls_verify_cert: bool,
    #[serde(default = "default_throw_on_append_failure")]
    pub(crate) throw_on_append_failure: bool,
    #[serde(default)]
    pub(crate) default_user_name: Option<Credentials>,
}

impl ConnectionSettings {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let mut result: ConnectionSettings = Default::default();
        let mut parsed_authority = false;

        let (initial_input, scheme) = alt((tag("esdb://"), tag("esdb+discover://")))(input)?;

        result.dns_discover = scheme == "esdb+discover://";
        let authority_valid_char = |c: char| c.is_ascii() && c != '@';
        let host_valid_char =
            |c: char| c.is_alphanumeric() || c == '-' || c == '.' || c == ':' || c == ',';
        let (mut input, mut content) = take_while(authority_valid_char)(initial_input)?;
        let at_tagged = opt(tag("@"))(input)?;

        if let (new_input, Some(_)) = at_tagged {
            let authority_parts: Vec<&str> = content.split(':').collect();

            match authority_parts.len() {
                1 => {
                    result.default_user_name = Some(Credentials::new(
                        authority_parts.as_slice()[0].to_string(),
                        "".into(),
                    ));
                }

                2 => {
                    let host = authority_parts.as_slice()[0].to_string();
                    let passw = authority_parts.as_slice()[1].to_string();

                    result.default_user_name = Some(Credentials::new(host, passw));
                }

                _ => {
                    return Err(nom::Err::Failure((input, ErrorKind::ParseTo)));
                }
            }

            let (new_input, new_content) = take_while(host_valid_char)(new_input)?;

            input = new_input;
            content = new_content;
            parsed_authority = true;
        }

        if !parsed_authority {
            let (new_input, new_content) = take_while(host_valid_char)(initial_input)?;
            input = new_input;
            content = new_content;
        }

        let hosts_parts: Vec<&str> = content.split(',').collect();

        for host in hosts_parts {
            let host_parts: Vec<&str> = host.split(':').collect();

            match host_parts.len() {
                1 => {
                    result.hosts.push(Endpoint {
                        host: host.to_string(),
                        port: 2113,
                    });
                }

                2 => {
                    if let Ok(port) = host_parts.as_slice()[1].parse() {
                        result.hosts.push(Endpoint {
                            host: host_parts.as_slice()[0].to_string(),
                            port,
                        });
                    } else {
                        return Err(nom::Err::Failure((input, ErrorKind::ParseTo)));
                    }
                }

                _ => {
                    return Err(nom::Err::Failure((input, ErrorKind::ParseTo)));
                }
            }
        }

        let (input, _) = opt(tag("/"))(input)?;
        let (mut input, has_params) = opt(tag("?"))(input)?;

        if has_params.is_some() {
            let (new_input, params) = all_consuming(take_while(|c: char| c.is_ascii()))(input)?;
            for param in params.split('&') {
                let values: Vec<&str> = param.split('=').collect();

                if values.len() == 2 {
                    match values.as_slice()[0] {
                        "maxDiscoverAttempts" => {
                            let value = values.as_slice()[1];
                            if let Ok(attempt) = value.parse() {
                                result.max_discover_attempts = attempt;
                            } else {
                                return Err(nom::Err::Failure((value, ErrorKind::ParseTo)));
                            }
                        }

                        "discoveryInterval" => {
                            let value = values.as_slice()[1];
                            if let Ok(millis) = value.parse() {
                                result.discovery_interval = Duration::from_millis(millis);
                            } else {
                                return Err(nom::Err::Failure((value, ErrorKind::ParseTo)));
                            }
                        }

                        "gossipTimeout" => {
                            let value = values.as_slice()[1];
                            if let Ok(millis) = value.parse() {
                                result.gossip_timeout = Duration::from_millis(millis);
                            } else {
                                return Err(nom::Err::Failure((value, ErrorKind::ParseTo)));
                            }
                        }

                        "tls" => {
                            let value = values.as_slice()[1];
                            if let Ok(bool) = value.parse() {
                                result.secure = bool;
                            } else {
                                return Err(nom::Err::Failure((value, ErrorKind::ParseTo)));
                            }
                        }

                        "tlsVerifyCert" => {
                            let value = values.as_slice()[1];
                            if let Ok(bool) = value.parse() {
                                result.tls_verify_cert = bool;
                            } else {
                                return Err(nom::Err::Failure((value, ErrorKind::ParseTo)));
                            }
                        }

                        "nodePreference" => match values.as_slice()[1] {
                            "follower" => {
                                result.preference = NodePreference::Follower;
                            }

                            "random" => {
                                result.preference = NodePreference::Random;
                            }

                            "leader" => {
                                result.preference = NodePreference::Leader;
                            }

                            "readOnlyReplica" => {
                                result.preference = NodePreference::ReadOnlyReplica;
                            }

                            wrong => {
                                return Err(nom::Err::Failure((wrong, ErrorKind::ParseTo)));
                            }
                        },

                        "throwOnAppendFailure" => {
                            let value = values.as_slice()[1];
                            if let Ok(bool) = value.parse() {
                                result.throw_on_append_failure = bool;
                            } else {
                                return Err(nom::Err::Failure((value, ErrorKind::ParseTo)));
                            }
                        }

                        _ => {
                            continue;
                        }
                    }
                } else {
                    return Err(nom::Err::Failure((param, ErrorKind::ParseTo)));
                }
            }

            input = new_input;
        }

        Ok((input, result))
    }

    pub fn parse_str(input: &str) -> Result<Self, ConnectionSettingsParseError> {
        match complete(ConnectionSettings::parse)(input) {
            Ok((_, setts)) => Ok(setts),
            Err(err_type) => match err_type {
                nom::Err::Error((input, _)) => Err(ConnectionSettingsParseError {
                    input: input.to_string(),
                }),

                nom::Err::Failure((input, _)) => Err(ConnectionSettingsParseError {
                    input: input.to_string(),
                }),

                nom::Err::Incomplete(_) => Err(ConnectionSettingsParseError {
                    input: "Incomplete connection string".to_string(),
                }),
            },
        }
    }

    pub fn to_uri(&self, endpoint: &Endpoint) -> http::Uri {
        let scheme = if self.secure { "https" } else { "http" };

        format!("{}://{}:{}", scheme, endpoint.host, endpoint.port)
            .parse()
            .unwrap()
    }
}

impl FromStr for ConnectionSettings {
    type Err = ConnectionSettingsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ConnectionSettings::parse_str(s)
    }
}

impl Default for ConnectionSettings {
    fn default() -> Self {
        ConnectionSettings {
            dns_discover: false,
            hosts: Vec::new(),
            max_discover_attempts: 3,
            discovery_interval: Duration::from_millis(500),
            gossip_timeout: Duration::from_secs(3),
            preference: Default::default(),
            secure: true,
            tls_verify_cert: true,
            throw_on_append_failure: true,
            default_user_name: None,
        }
    }
}

async fn cluster_mode_connection(
    conn_setts: ConnectionSettings,
) -> Result<UnboundedSender<Msg>, Box<dyn std::error::Error>> {
    let (sender, mut consumer) = futures::channel::mpsc::unbounded::<Msg>();
    let kind = if conn_setts.dns_discover {
        let endpoint = conn_setts.hosts.as_slice()[0].clone();
        let domain_name = format!("{}.", endpoint.host)
            .as_str()
            .parse::<trust_dns_resolver::Name>()?;

        let resolver = trust_dns_resolver::TokioAsyncResolver::tokio_from_system_conf().await?;

        let dns_settings = DnsClusterSettings {
            resolver,
            domain_name,
            gossip_port: endpoint.port,
        };

        Either::Right(dns_settings)
    } else {
        Either::Left(conn_setts.hosts.clone())
    };

    tokio::spawn(async move {
        let node_selection_delay_on_error_in_ms = 500;
        let mut channel: Option<Channel> = None;
        let mut channel_id = Uuid::new_v4();
        let failed_endpoint: Option<Endpoint> = None;
        let mut previous_candidates: Option<Vec<Member>> = None;
        let mut work_queue = Vec::new();
        let mut rng = SmallRng::from_entropy();

        while let Some(item) = consumer.next().await {
            work_queue.push(item);

            while let Some(msg) = work_queue.pop() {
                match msg {
                    Msg::GetChannel(resp) => {
                        if let Some(channel) = channel.as_ref() {
                            let handle = Handle {
                                id: channel_id,
                                channel: channel.clone(),
                            };

                            let _ = resp.send(Ok(handle));
                        } else {
                            // It means we need to create a new channel.
                            work_queue.push(Msg::GetChannel(resp));
                            work_queue.push(Msg::CreateChannel(channel_id, None));
                        }
                    }

                    Msg::CreateChannel(id, seed_opt) => {
                        if channel_id != id {
                            continue;
                        }

                        let node = if let Some(ref seed) = seed_opt {
                            Some(seed.clone())
                        } else {
                            node_selection(
                                &conn_setts,
                                &kind,
                                &failed_endpoint,
                                &mut rng,
                                &mut previous_candidates,
                            )
                            .await
                        };

                        if let Some(node) = node {
                            match create_channel(&conn_setts, &node).await {
                                Ok(new_channel) => {
                                    channel_id = Uuid::new_v4();
                                    channel = Some(new_channel);
                                }

                                Err(err) => {
                                    error!(
                                        "Error when creating a gRPC channel to {:?}: {}",
                                        node, err
                                    );

                                    tokio::time::delay_for(conn_setts.discovery_interval).await;
                                    work_queue.push(Msg::CreateChannel(id, seed_opt));
                                }
                            }
                        } else {
                            error!("Unable to select a node. Retrying");
                            tokio::time::delay_for(Duration::from_millis(
                                node_selection_delay_on_error_in_ms,
                            ))
                            .await;
                            work_queue.push(Msg::CreateChannel(id, seed_opt));
                        }
                    }
                }
            }
        }
    });

    Ok(sender)
}

fn single_node_mode(conn_setts: ConnectionSettings, endpoint: Endpoint) -> UnboundedSender<Msg> {
    let (sender, mut consumer) = futures::channel::mpsc::unbounded::<Msg>();

    tokio::spawn(async move {
        let mut channel: Option<Channel> = None;
        let mut channel_id = Uuid::new_v4();
        let mut work_queue = Vec::new();

        while let Some(item) = consumer.next().await {
            work_queue.push(item);

            while let Some(msg) = work_queue.pop() {
                match msg {
                    Msg::GetChannel(resp) => {
                        if let Some(channel) = channel.as_ref() {
                            let handle = Handle {
                                id: channel_id,
                                channel: channel.clone(),
                            };

                            let _ = resp.send(Ok(handle));
                        } else {
                            // It means we need to create a new channel.
                            work_queue.push(Msg::GetChannel(resp));
                            work_queue.push(Msg::CreateChannel(channel_id, None));
                        }
                    }

                    Msg::CreateChannel(id, seed_opt) => {
                        if channel_id != id {
                            continue;
                        }

                        let node = if let Some(ref seed) = seed_opt {
                            seed.clone()
                        } else {
                            endpoint.clone()
                        };

                        match create_channel(&conn_setts, &node).await {
                            Ok(new_channel) => {
                                channel_id = Uuid::new_v4();
                                channel = Some(new_channel);
                            }

                            Err(err) => {
                                error!(
                                    "Error when connecting to {}: {}",
                                    conn_setts.to_uri(&endpoint),
                                    err
                                );

                                tokio::time::delay_for(conn_setts.discovery_interval).await;
                                work_queue.push(Msg::CreateChannel(id, seed_opt));
                            }
                        }
                    }
                }
            }
        }
    });

    sender
}

async fn create_channel(
    setts: &ConnectionSettings,
    endpoint: &Endpoint,
) -> Result<Channel, tonic::transport::Error> {
    let uri = setts.to_uri(endpoint);

    debug!("Create gRPC channel for: {}", uri);

    let mut channel = Channel::builder(uri.clone());

    if !setts.tls_verify_cert {
        let mut rustls_config = rustls::ClientConfig::new();
        let protocols = vec![(b"h2".to_vec())];

        rustls_config.set_protocols(protocols.as_slice());

        rustls_config
            .dangerous()
            .set_certificate_verifier(std::sync::Arc::new(NoVerification));

        let client_config =
            tonic::transport::ClientTlsConfig::new().rustls_client_config(rustls_config);

        channel = channel.tls_config(client_config)?;
    }

    let channel = channel.connect().await?;
    debug!("Connected to Node: {}", uri);

    Ok(channel)
}

#[derive(Clone)]
struct Handle {
    id: Uuid,
    channel: Channel,
}

enum Msg {
    GetChannel(oneshot::Sender<Result<Handle, Status>>),
    CreateChannel(Uuid, Option<Endpoint>),
}

impl std::fmt::Debug for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Msg::GetChannel(_) => write!(f, "Msg::GetChannel"),
            Msg::CreateChannel(id, seed_opt) => {
                write!(f, "Msg::CreateChannel({:?}, {:?})", id, seed_opt)
            }
        }
    }
}

#[derive(Clone)]
pub struct GrpcConnection {
    sender: futures::channel::mpsc::UnboundedSender<Msg>,
}

impl GrpcConnection {
    pub async fn create(
        conn_setts: ConnectionSettings,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let sender = if conn_setts.dns_discover || conn_setts.hosts.len() > 1 {
            cluster_mode_connection(conn_setts).await?
        } else {
            let endpoint = conn_setts
                .hosts
                .first()
                .expect("Impossible: hosts can't be empty")
                .clone();

            single_node_mode(conn_setts, endpoint)
        };

        Ok(GrpcConnection { sender })
    }

    pub async fn execute<F, Fut, A>(&self, action: F) -> Result<A, Status>
    where
        F: FnOnce(Channel) -> Fut + Send,
        Fut: Future<Output = Result<A, Status>> + Send,
        A: Send,
    {
        let (sender, consumer) = futures::channel::oneshot::channel();

        if self
            .sender
            .clone()
            .send(Msg::GetChannel(sender))
            .await
            .is_err()
        {
            return Err(Status::aborted("Connection is closed"));
        }

        // FIXME - Introduce a better error when the main connection has been closed.
        let handle = consumer.await.unwrap()?;

        match action(handle.channel).await {
            Err(status) => {
                if status.code() == tonic::Code::Unavailable {
                    let _ = self
                        .sender
                        .clone()
                        .send(Msg::CreateChannel(handle.id, None))
                        .await;
                }

                let leader_endpoint = status
                    .metadata()
                    .get("leader-endpoint-host")
                    .zip(status.metadata().get("leader-endpoint-port"))
                    .and_then(|(host, port)| {
                        let host = host.to_str().ok()?;
                        let port = port.to_str().ok()?;
                        let host = host.to_string();
                        let port = port.parse().ok()?;

                        Some(Endpoint { host, port })
                    });

                if let Some(leader) = leader_endpoint {
                    if self
                        .sender
                        .clone()
                        .send(Msg::CreateChannel(handle.id, Some(leader)))
                        .await
                        .is_err()
                    {
                        return Err(Status::aborted("Connection closed"));
                    }

                    // FIXME - Return a very specific NotLeaderException.
                    // FIXME - Consider returning a meaningful exception compare to tonic::Status.
                    // FIXME - Consider an option so we can replay the command on behalf of the user.
                }

                Err(status)
            }

            Ok(a) => Ok(a),
        }
    }
}

struct Member {
    endpoint: Endpoint,
    state: VNodeState,
}

async fn node_selection(
    conn_setts: &ConnectionSettings,
    kind: &Either<Vec<Endpoint>, DnsClusterSettings>,
    failed_endpoint: &Option<Endpoint>,
    rng: &mut SmallRng,
    previous_candidates: &mut Option<Vec<Member>>,
) -> Option<Endpoint> {
    let candidates = match previous_candidates.take() {
        Some(old_candidates) => {
            let mut new_candidates = candidates_from_old_gossip(&failed_endpoint, old_candidates);

            // Use case: when the cluster is only comprised of a single node and that node
            // previously failed. This can only happen if the user used a fixed set of seeds.
            if new_candidates.is_empty() {
                new_candidates = conn_setts.hosts.clone();
            }

            new_candidates
        }

        None => {
            let mut seeds = match kind.as_ref() {
                Either::Left(seeds) => seeds.clone(),
                Either::Right(dns) => match candidates_from_dns(dns).await {
                    Ok(seeds) => seeds,

                    Err(e) => {
                        error!("Error when performing DNS resolution: {}", e);
                        Vec::new()
                    }
                },
            };

            seeds.shuffle(rng);
            seeds
        }
    };

    debug!("List of candidates: {:?}", candidates);

    for candidate in candidates {
        match create_channel(conn_setts, &candidate).await {
            Ok(channel) => {
                let gossip_client = Gossip::create(channel);

                debug!("Calling gossip endpoint on: {:?}", candidate);
                match gossip_client.read().await {
                    Ok(members_info) => {
                        debug!("Candidate {:?} gossip info: {:?}", candidate, members_info);
                        let selected_node = determine_best_node(
                            rng,
                            conn_setts.preference,
                            members_info.as_slice(),
                        );

                        if let Some(selected_node) = selected_node {
                            return Some(selected_node);
                        }
                    }
                    Err(err) => {
                        debug!(
                            "Failed to retrieve gossip information from candidate {:?}: {}",
                            &candidate, err
                        );
                    }
                }
            }

            Err(err) => debug!(
                "Failed to create gRPC channel for candidate {:?}: {}",
                candidate, err
            ),
        }
    }

    None
}

struct Candidates {
    nodes: Vec<Member>,
    managers: Vec<Member>,
}

impl Candidates {
    fn new() -> Candidates {
        Candidates {
            nodes: vec![],
            managers: vec![],
        }
    }

    fn push(&mut self, member: Member) {
        if let VNodeState::Manager = member.state {
            self.managers.push(member);
        } else {
            self.nodes.push(member);
        }
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();

        self.nodes.shuffle(&mut rng);
        self.managers.shuffle(&mut rng);
    }

    fn endpoints(mut self) -> Vec<Endpoint> {
        self.nodes.extend(self.managers);

        self.nodes.into_iter().map(|m| m.endpoint).collect()
    }
}

async fn candidates_from_dns(
    dns: &DnsClusterSettings,
) -> Result<Vec<Endpoint>, trust_dns_resolver::error::ResolveError> {
    let lookup = dns.resolver.srv_lookup(dns.domain_name.clone()).await?;
    let mut endpoints = Vec::new();

    for ip in lookup.ip_iter() {
        let endpoint = Endpoint {
            host: ip.to_string(),
            port: dns.gossip_port,
        };
        //GossipSeed::from_socket_addr(SocketAddr::new(ip, settings.gossip_port));
        endpoints.push(endpoint);
    }

    Ok(endpoints)
}

fn candidates_from_old_gossip(
    failed_endpoint: &Option<Endpoint>,
    old_candidates: Vec<Member>,
) -> Vec<Endpoint> {
    let candidates = match failed_endpoint {
        Some(endpoint) => old_candidates
            .into_iter()
            .filter(|member| member.endpoint != *endpoint)
            .collect(),

        None => old_candidates,
    };

    arrange_gossip_candidates(candidates)
}

fn arrange_gossip_candidates(candidates: Vec<Member>) -> Vec<Endpoint> {
    let mut arranged_candidates = Candidates::new();

    for member in candidates {
        arranged_candidates.push(member);
    }

    arranged_candidates.shuffle();
    arranged_candidates.endpoints()
}

fn determine_best_node(
    rng: &mut SmallRng,
    preference: NodePreference,
    members: &[MemberInfo],
) -> Option<Endpoint> {
    fn allowed_states(state: VNodeState) -> bool {
        !matches!(
            state,
            VNodeState::Manager | VNodeState::ShuttingDown | VNodeState::Shutdown
        )
    }

    let members = members
        .iter()
        .filter(|member| member.is_alive)
        .filter(|member| allowed_states(member.state));

    let member_opt = match preference {
        NodePreference::Leader => members.min_by(|a, b| {
            if a.state == VNodeState::Leader {
                return Ordering::Less;
            }

            if b.state == VNodeState::Leader {
                return Ordering::Greater;
            }

            Ordering::Equal
        }),

        NodePreference::Follower => members.min_by(|a, b| {
            if a.state == VNodeState::Follower {
                return Ordering::Less;
            }

            if b.state == VNodeState::Follower {
                return Ordering::Greater;
            }

            Ordering::Equal
        }),

        NodePreference::Random => members.min_by(|_, _| {
            if rng.next_u32() % 2 == 0 {
                return Ordering::Greater;
            }

            Ordering::Less
        }),

        NodePreference::ReadOnlyReplica => members.min_by(|a, b| {
            if a.state == VNodeState::ReadOnlyReplica {
                return Ordering::Less;
            }

            if b.state == VNodeState::ReadOnlyReplica {
                return Ordering::Greater;
            }

            Ordering::Equal
        }),
    };

    member_opt.map(|member| {
        info!(
            "Discovering: found best choice {}:{} ({:?})",
            member.http_end_point.host, member.http_end_point.port, member.state
        );

        member.http_end_point.clone()
    })
}
