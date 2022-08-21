use crate::event_store::generated::server_features::server_features_client::ServerFeaturesClient;
use crate::event_store::generated::Empty;
use crate::grpc::HyperClient;
use bitflags::bitflags;
use tonic::{Code, Request, Status};

bitflags! {
    pub struct Features: u32 {
        const NOTHING = 0;
        const BATCH_APPEND = 1;
        const PERSISTENT_SUBSCRIPTION_LIST = 2;
        const PERSISTENT_SUBSCRIPTION_REPLAY = 4;
        const PERSISTENT_SUBSCRIPTION_RESTART_SUBSYSTEM = 8;
        const PERSISTENT_SUBSCRIPTION_GET_INFO = 16;
        const PERSISTENT_SUBSCRIPTION_MANAGEMENT = Self::PERSISTENT_SUBSCRIPTION_LIST.bits | Self::PERSISTENT_SUBSCRIPTION_REPLAY.bits | Self::PERSISTENT_SUBSCRIPTION_RESTART_SUBSYSTEM .bits | Self::PERSISTENT_SUBSCRIPTION_GET_INFO.bits;
        const PERSISTENT_SUBSCRIPITON_TO_ALL = 32;
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct ServerVersion {
    pub(crate) major: usize,
    pub(crate) minor: usize,
    pub(crate) patch: usize,
}

impl ServerVersion {
    pub fn major(&self) -> usize {
        self.major
    }

    pub fn minor(&self) -> usize {
        self.minor
    }

    pub fn patch(&self) -> usize {
        self.patch
    }
}

#[allow(unused)]
#[derive(Clone, Copy, Debug)]
pub struct ServerInfo {
    pub(crate) version: ServerVersion,
    pub(crate) features: Features,
}

impl ServerInfo {
    pub fn version(&self) -> ServerVersion {
        self.version
    }

    pub fn features(&self) -> Features {
        self.features
    }
}

pub(crate) async fn supported_methods(
    client: &HyperClient,
    uri: hyper::Uri,
) -> Result<ServerInfo, Status> {
    let mut client = ServerFeaturesClient::with_origin(client, uri);
    let methods = client
        .get_supported_methods(Request::new(Empty {}))
        .await?
        .into_inner();

    let mut version = ServerVersion::default();

    // Server version uses sem versioning.
    for (idx, ver) in methods.event_store_server_version.split('.').enumerate() {
        if idx > 2 {
            break;
        }

        match ver.parse::<usize>() {
            Err(_) => return Err(Status::new(Code::InvalidArgument, "invalid semver format")),
            Ok(v) => match idx {
                0 => version.major = v,
                1 => version.minor = v,
                2 => version.patch = v,
                _ => unreachable!(),
            },
        }
    }

    let mut features = Features::NOTHING;

    for method in methods.methods {
        let feats = method.features;
        match (method.method_name.as_str(), method.service_name.as_str()) {
            ("batchappend", "event_store.client.streams.streams") => {
                features |= Features::BATCH_APPEND
            }

            (method, "event_store.client.persistent_subscriptions.persistentsubscriptions") => {
                match method {
                    "create" => {
                        for feat in feats {
                            if feat == "all" {
                                features |= Features::PERSISTENT_SUBSCRIPITON_TO_ALL;
                            }
                        }
                    }
                    "getinfo" => features |= Features::PERSISTENT_SUBSCRIPTION_GET_INFO,
                    "replayparked" => features |= Features::PERSISTENT_SUBSCRIPTION_REPLAY,
                    "list" => features |= Features::PERSISTENT_SUBSCRIPTION_LIST,
                    "restartsubsystem" => {
                        features |= Features::PERSISTENT_SUBSCRIPTION_RESTART_SUBSYSTEM
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(ServerInfo { version, features })
}
