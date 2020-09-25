use crate::es6::gossip::{Gossip, MemberInfo, VNodeState};
use crate::es6::types::Endpoint;
use crate::{ClusterSettings, Credentials, Either, NodePreference};
use async_trait::async_trait;
use futures::channel::oneshot;
use futures::stream::StreamExt;
use futures::{Future, SinkExt};
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{RngCore, SeedableRng};
use std::cmp::Ordering;
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

#[derive(Clone, Default)]
pub struct ConnectionSettings {
    pub disable_certs_validation: bool,
    pub default_user_name: Option<Credentials>,
}

async fn create_channel(
    setts: &ConnectionSettings,
    endpoint: &Endpoint,
) -> Result<Channel, tonic::transport::Error> {
    // TODO - Allow to use insecure connection.
    let uri = format!("https://{}:{}", endpoint.host, endpoint.port)
        .parse()
        .unwrap();
    let mut channel = Channel::builder(uri);

    if setts.disable_certs_validation {
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

    Ok(channel)
}

#[async_trait]
pub trait EventStoreDBConnection: Clone + Send + Sync {
    async fn execute<F, Fut, A>(&self, action: F) -> Result<A, Status>
    where
        F: FnOnce(Channel) -> Fut + Send,
        Fut: Future<Output = Result<A, Status>> + Send,
        A: Send;
}

#[derive(Clone)]
pub struct StaticEventStoreDBConnection {
    handle: Handle,
}

#[async_trait]
impl EventStoreDBConnection for StaticEventStoreDBConnection {
    async fn execute<F, Fut, A>(&self, action: F) -> Result<A, Status>
    where
        F: FnOnce(Channel) -> Fut + Send,
        Fut: Future<Output = Result<A, Status>> + Send,
        A: Send,
    {
        action(self.handle.channel.clone()).await
    }
}

impl StaticEventStoreDBConnection {
    pub async fn create(
        setts: ConnectionSettings,
        endpoint: Endpoint,
    ) -> Result<Self, tonic::transport::Error> {
        let channel = create_channel(&setts, &endpoint).await?;
        let handle = Handle {
            channel,
            id: Uuid::new_v4(),
        };

        Ok(StaticEventStoreDBConnection { handle })
    }
}

#[derive(Clone)]
pub struct ClusterEventStoreDBConnection {
    preference: NodePreference,
    sender: futures::channel::mpsc::UnboundedSender<Msg>,
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

impl ClusterEventStoreDBConnection {
    pub fn create(conn_setts: ConnectionSettings, setts: ClusterSettings) -> Self {
        let (sender, mut consumer) = futures::channel::mpsc::unbounded::<Msg>();
        let preference = setts.preference;

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
                                    &setts,
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

                                        tokio::time::delay_for(Duration::from_millis(
                                            node_selection_delay_on_error_in_ms,
                                        ))
                                        .await;
                                        work_queue.push(Msg::CreateChannel(id, seed_opt))
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

        ClusterEventStoreDBConnection { preference, sender }
    }
}

#[async_trait]
impl EventStoreDBConnection for ClusterEventStoreDBConnection {
    async fn execute<F, Fut, A>(&self, action: F) -> Result<A, Status>
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
    setts: &ClusterSettings,
    failed_endpoint: &Option<Endpoint>,
    rng: &mut SmallRng,
    previous_candidates: &mut Option<Vec<Member>>,
) -> Option<Endpoint> {
    let candidates = match previous_candidates.take() {
        Some(old_candidates) => candidates_from_old_gossip(&failed_endpoint, old_candidates),

        None => match candidates_from_dns(rng, &setts).await {
            Ok(seeds) => seeds,
            Err(e) => {
                error!("Error when performing DNS resolution: {}", e);
                Vec::new()
            }
        },
    };

    for candidate in candidates {
        match create_channel(conn_setts, &candidate).await {
            Ok(channel) => {
                let gossip_client = Gossip::create(channel);

                match gossip_client.read().await {
                    Ok(members_info) => {
                        let selected_node =
                            determine_best_node(rng, setts.preference, members_info.as_slice());

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
    rng: &mut SmallRng,
    settings: &ClusterSettings,
) -> Result<Vec<Endpoint>, trust_dns_resolver::error::ResolveError> {
    let mut src = match settings.kind.as_ref() {
        Either::Left(seeds) => {
            let endpoints = seeds
                .iter()
                .map(|s| Endpoint {
                    host: s.endpoint.addr.ip().to_string(),
                    port: s.endpoint.addr.port() as u32,
                })
                .collect();

            Ok::<Vec<Endpoint>, trust_dns_resolver::error::ResolveError>(endpoints)
        }
        Either::Right(dns) => {
            let lookup = dns.resolver.srv_lookup(dns.domain_name.clone()).await?;
            let mut endpoints = Vec::new();

            for ip in lookup.ip_iter() {
                let endpoint = Endpoint {
                    host: ip.to_string(),
                    port: settings.gossip_port as u32,
                };
                //GossipSeed::from_socket_addr(SocketAddr::new(ip, settings.gossip_port));
                endpoints.push(endpoint);
            }

            Ok(endpoints)
        }
    }?;

    src.shuffle(rng);
    Ok(src)
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
        match state {
            VNodeState::Manager | VNodeState::ShuttingDown | VNodeState::Shutdown => false,
            _ => true,
        }
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
