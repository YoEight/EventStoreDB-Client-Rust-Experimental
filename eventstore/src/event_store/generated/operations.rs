#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartScavengeReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<start_scavenge_req::Options>,
}
/// Nested message and enum types in `StartScavengeReq`.
pub mod start_scavenge_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(int32, tag = "1")]
        pub thread_count: i32,
        #[prost(int32, tag = "2")]
        pub start_from_chunk: i32,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopScavengeReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<stop_scavenge_req::Options>,
}
/// Nested message and enum types in `StopScavengeReq`.
pub mod stop_scavenge_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "1")]
        pub scavenge_id: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScavengeResp {
    #[prost(string, tag = "1")]
    pub scavenge_id: ::prost::alloc::string::String,
    #[prost(enumeration = "scavenge_resp::ScavengeResult", tag = "2")]
    pub scavenge_result: i32,
}
/// Nested message and enum types in `ScavengeResp`.
pub mod scavenge_resp {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ScavengeResult {
        Started = 0,
        InProgress = 1,
        Stopped = 2,
    }
    impl ScavengeResult {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ScavengeResult::Started => "Started",
                ScavengeResult::InProgress => "InProgress",
                ScavengeResult::Stopped => "Stopped",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Started" => Some(Self::Started),
                "InProgress" => Some(Self::InProgress),
                "Stopped" => Some(Self::Stopped),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNodePriorityReq {
    #[prost(int32, tag = "1")]
    pub priority: i32,
}
/// Generated client implementations.
pub mod operations_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OperationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OperationsClient<tonic::transport::Channel> {
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
    impl<T> OperationsClient<T>
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
        ) -> OperationsClient<InterceptedService<T, F>>
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
            OperationsClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn start_scavenge(
            &mut self,
            request: impl tonic::IntoRequest<super::StartScavengeReq>,
        ) -> std::result::Result<tonic::Response<super::ScavengeResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.operations.Operations/StartScavenge",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.operations.Operations",
                "StartScavenge",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop_scavenge(
            &mut self,
            request: impl tonic::IntoRequest<super::StopScavengeReq>,
        ) -> std::result::Result<tonic::Response<super::ScavengeResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.operations.Operations/StopScavenge",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.operations.Operations",
                "StopScavenge",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn shutdown(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.operations.Operations/Shutdown",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.operations.Operations",
                "Shutdown",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn merge_indexes(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.operations.Operations/MergeIndexes",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.operations.Operations",
                "MergeIndexes",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn resign_node(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.operations.Operations/ResignNode",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.operations.Operations",
                "ResignNode",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_node_priority(
            &mut self,
            request: impl tonic::IntoRequest<super::SetNodePriorityReq>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.operations.Operations/SetNodePriority",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.operations.Operations",
                "SetNodePriority",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn restart_persistent_subscriptions(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.operations.Operations/RestartPersistentSubscriptions",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.operations.Operations",
                "RestartPersistentSubscriptions",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
