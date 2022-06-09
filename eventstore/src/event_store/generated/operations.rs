#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartScavengeReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<start_scavenge_req::Options>,
}
/// Nested message and enum types in `StartScavengeReq`.
pub mod start_scavenge_req {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(int32, tag = "1")]
        pub thread_count: i32,
        #[prost(int32, tag = "2")]
        pub start_from_chunk: i32,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopScavengeReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<stop_scavenge_req::Options>,
}
/// Nested message and enum types in `StopScavengeReq`.
pub mod stop_scavenge_req {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "1")]
        pub scavenge_id: ::prost::alloc::string::String,
    }
}
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
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNodePriorityReq {
    #[prost(int32, tag = "1")]
    pub priority: i32,
}
#[doc = r" Generated client implementations."]
pub mod operations_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OperationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OperationsClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OperationsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OperationsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
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
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn start_scavenge(
            &mut self,
            request: impl tonic::IntoRequest<super::StartScavengeReq>,
        ) -> Result<tonic::Response<super::ScavengeResp>, tonic::Status> {
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn stop_scavenge(
            &mut self,
            request: impl tonic::IntoRequest<super::StopScavengeReq>,
        ) -> Result<tonic::Response<super::ScavengeResp>, tonic::Status> {
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn shutdown(
            &mut self,
            request: impl tonic::IntoRequest<super::super::Empty>,
        ) -> Result<tonic::Response<super::super::Empty>, tonic::Status> {
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn merge_indexes(
            &mut self,
            request: impl tonic::IntoRequest<super::super::Empty>,
        ) -> Result<tonic::Response<super::super::Empty>, tonic::Status> {
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn resign_node(
            &mut self,
            request: impl tonic::IntoRequest<super::super::Empty>,
        ) -> Result<tonic::Response<super::super::Empty>, tonic::Status> {
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_node_priority(
            &mut self,
            request: impl tonic::IntoRequest<super::SetNodePriorityReq>,
        ) -> Result<tonic::Response<super::super::Empty>, tonic::Status> {
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn restart_persistent_subscriptions(
            &mut self,
            request: impl tonic::IntoRequest<super::super::Empty>,
        ) -> Result<tonic::Response<super::super::Empty>, tonic::Status> {
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
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
