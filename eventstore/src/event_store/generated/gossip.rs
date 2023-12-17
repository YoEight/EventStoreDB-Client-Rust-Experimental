#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterInfo {
    #[prost(message, repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<MemberInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndPoint {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub port: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemberInfo {
    #[prost(message, optional, tag = "1")]
    pub instance_id: ::core::option::Option<crate::event_store::generated::common::Uuid>,
    #[prost(int64, tag = "2")]
    pub time_stamp: i64,
    #[prost(enumeration = "member_info::VNodeState", tag = "3")]
    pub state: i32,
    #[prost(bool, tag = "4")]
    pub is_alive: bool,
    #[prost(message, optional, tag = "5")]
    pub http_end_point: ::core::option::Option<EndPoint>,
}
/// Nested message and enum types in `MemberInfo`.
pub mod member_info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VNodeState {
        Initializing = 0,
        DiscoverLeader = 1,
        Unknown = 2,
        PreReplica = 3,
        CatchingUp = 4,
        Clone = 5,
        Follower = 6,
        PreLeader = 7,
        Leader = 8,
        Manager = 9,
        ShuttingDown = 10,
        Shutdown = 11,
        ReadOnlyLeaderless = 12,
        PreReadOnlyReplica = 13,
        ReadOnlyReplica = 14,
        ResigningLeader = 15,
    }
    impl VNodeState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                VNodeState::Initializing => "Initializing",
                VNodeState::DiscoverLeader => "DiscoverLeader",
                VNodeState::Unknown => "Unknown",
                VNodeState::PreReplica => "PreReplica",
                VNodeState::CatchingUp => "CatchingUp",
                VNodeState::Clone => "Clone",
                VNodeState::Follower => "Follower",
                VNodeState::PreLeader => "PreLeader",
                VNodeState::Leader => "Leader",
                VNodeState::Manager => "Manager",
                VNodeState::ShuttingDown => "ShuttingDown",
                VNodeState::Shutdown => "Shutdown",
                VNodeState::ReadOnlyLeaderless => "ReadOnlyLeaderless",
                VNodeState::PreReadOnlyReplica => "PreReadOnlyReplica",
                VNodeState::ReadOnlyReplica => "ReadOnlyReplica",
                VNodeState::ResigningLeader => "ResigningLeader",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Initializing" => Some(Self::Initializing),
                "DiscoverLeader" => Some(Self::DiscoverLeader),
                "Unknown" => Some(Self::Unknown),
                "PreReplica" => Some(Self::PreReplica),
                "CatchingUp" => Some(Self::CatchingUp),
                "Clone" => Some(Self::Clone),
                "Follower" => Some(Self::Follower),
                "PreLeader" => Some(Self::PreLeader),
                "Leader" => Some(Self::Leader),
                "Manager" => Some(Self::Manager),
                "ShuttingDown" => Some(Self::ShuttingDown),
                "Shutdown" => Some(Self::Shutdown),
                "ReadOnlyLeaderless" => Some(Self::ReadOnlyLeaderless),
                "PreReadOnlyReplica" => Some(Self::PreReadOnlyReplica),
                "ReadOnlyReplica" => Some(Self::ReadOnlyReplica),
                "ResigningLeader" => Some(Self::ResigningLeader),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod gossip_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct GossipClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GossipClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GossipClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GossipClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            GossipClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn read(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::ClusterInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/event_store.client.gossip.Gossip/Read");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("event_store.client.gossip.Gossip", "Read"));
            self.inner.unary(req, path, codec).await
        }
    }
}
