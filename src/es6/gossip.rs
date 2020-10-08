use crate::es6::grpc::event_store::client::gossip as wire;
use crate::es6::grpc::event_store::client::shared::{self, Empty};
use crate::es6::types::Endpoint;
use tonic::transport::Channel;
use tonic::{Request, Status};
use uuid::Uuid;

fn uuid_from_structured(most: u64, least: u64) -> Uuid {
    let repr = (most as u128) << 64 | least as u128;

    Uuid::from_u128(repr)
}

pub struct Gossip {
    inner: wire::gossip_client::GossipClient<Channel>,
}

impl Gossip {
    pub fn create(channel: Channel) -> Self {
        let inner = wire::gossip_client::GossipClient::new(channel);

        Gossip { inner }
    }

    pub async fn read(&self) -> Result<Vec<MemberInfo>, Status> {
        debug!("Before reading gossip");
        let wire_members = self
            .inner
            .clone()
            .read(Request::new(Empty {}))
            .await?
            .into_inner()
            .members;
        debug!("After receiving gossip");

        let mut members = Vec::with_capacity(wire_members.capacity());
        for wire_member in wire_members {
            let state = VNodeState::from_i32(wire_member.state)?;

            let instance_id =
                if let Some(wire_uuid) = wire_member.instance_id.and_then(|uuid| uuid.value) {
                    match wire_uuid {
                        shared::uuid::Value::Structured(repr) => uuid_from_structured(
                            repr.most_significant_bits as u64,
                            repr.least_significant_bits as u64,
                        ),

                        shared::uuid::Value::String(str) => Uuid::parse_str(str.as_str()).unwrap(),
                    }
                } else {
                    Uuid::nil()
                };

            let http_end_point = if let Some(endpoint) = wire_member.http_end_point {
                let endpoint = Endpoint {
                    host: endpoint.address,
                    port: endpoint.port,
                };

                Ok(endpoint)
            } else {
                Err(Status::failed_precondition(
                    "MemberInfo endpoint must be defined",
                ))
            }?;

            let member = MemberInfo {
                instance_id,
                state,
                is_alive: wire_member.is_alive,
                time_stamp: wire_member.time_stamp,
                http_end_point,
            };
            members.push(member);
        }

        Ok(members)
    }
}

#[derive(Debug)]
pub struct MemberInfo {
    pub instance_id: Uuid,
    pub time_stamp: i64,
    pub state: VNodeState,
    pub is_alive: bool,
    pub http_end_point: Endpoint,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VNodeState {
    Initializing,
    DiscoverLeader,
    Unknown,
    PreReplica,
    CatchingUp,
    Clone,
    Follower,
    PreLeader,
    Leader,
    Manager,
    ShuttingDown,
    Shutdown,
    ReadOnlyLeaderLess,
    PreReadOnlyReplica,
    ReadOnlyReplica,
    ResigningLeader,
}

impl VNodeState {
    pub fn from_i32(value: i32) -> Result<Self, Status> {
        match value {
            0 => Ok(VNodeState::Initializing),
            1 => Ok(VNodeState::DiscoverLeader),
            2 => Ok(VNodeState::Unknown),
            3 => Ok(VNodeState::PreReplica),
            4 => Ok(VNodeState::CatchingUp),
            5 => Ok(VNodeState::Clone),
            6 => Ok(VNodeState::Follower),
            7 => Ok(VNodeState::PreLeader),
            8 => Ok(VNodeState::Leader),
            9 => Ok(VNodeState::Manager),
            10 => Ok(VNodeState::ShuttingDown),
            11 => Ok(VNodeState::Shutdown),
            12 => Ok(VNodeState::ReadOnlyLeaderLess),
            13 => Ok(VNodeState::PreReadOnlyReplica),
            14 => Ok(VNodeState::ReadOnlyReplica),
            15 => Ok(VNodeState::ResigningLeader),
            _ => Err(Status::out_of_range(format!(
                "Unknown VNodeState value: {}",
                value
            ))),
        }
    }
}
