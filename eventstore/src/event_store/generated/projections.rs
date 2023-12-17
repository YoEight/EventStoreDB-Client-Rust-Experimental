#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<create_req::Options>,
}
/// Nested message and enum types in `CreateReq`.
pub mod create_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "4")]
        pub query: ::prost::alloc::string::String,
        #[prost(oneof = "options::Mode", tags = "1, 2, 3")]
        pub mode: ::core::option::Option<options::Mode>,
    }
    /// Nested message and enum types in `Options`.
    pub mod options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Transient {
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Continuous {
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            #[prost(bool, tag = "2")]
            pub track_emitted_streams: bool,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Mode {
            #[prost(message, tag = "1")]
            OneTime(()),
            #[prost(message, tag = "2")]
            Transient(Transient),
            #[prost(message, tag = "3")]
            Continuous(Continuous),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateResp {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<update_req::Options>,
}
/// Nested message and enum types in `UpdateReq`.
pub mod update_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub query: ::prost::alloc::string::String,
        #[prost(oneof = "options::EmitOption", tags = "3, 4")]
        pub emit_option: ::core::option::Option<options::EmitOption>,
    }
    /// Nested message and enum types in `Options`.
    pub mod options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum EmitOption {
            #[prost(bool, tag = "3")]
            EmitEnabled(bool),
            #[prost(message, tag = "4")]
            NoEmitOptions(()),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResp {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<delete_req::Options>,
}
/// Nested message and enum types in `DeleteReq`.
pub mod delete_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(bool, tag = "2")]
        pub delete_emitted_streams: bool,
        #[prost(bool, tag = "3")]
        pub delete_state_stream: bool,
        #[prost(bool, tag = "4")]
        pub delete_checkpoint_stream: bool,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResp {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatisticsReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<statistics_req::Options>,
}
/// Nested message and enum types in `StatisticsReq`.
pub mod statistics_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(oneof = "options::Mode", tags = "1, 2, 3, 4, 5")]
        pub mode: ::core::option::Option<options::Mode>,
    }
    /// Nested message and enum types in `Options`.
    pub mod options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Mode {
            #[prost(string, tag = "1")]
            Name(::prost::alloc::string::String),
            #[prost(message, tag = "2")]
            All(()),
            #[prost(message, tag = "3")]
            Transient(()),
            #[prost(message, tag = "4")]
            Continuous(()),
            #[prost(message, tag = "5")]
            OneTime(()),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatisticsResp {
    #[prost(message, optional, tag = "1")]
    pub details: ::core::option::Option<statistics_resp::Details>,
}
/// Nested message and enum types in `StatisticsResp`.
pub mod statistics_resp {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Details {
        #[prost(int64, tag = "1")]
        pub core_processing_time: i64,
        #[prost(int64, tag = "2")]
        pub version: i64,
        #[prost(int64, tag = "3")]
        pub epoch: i64,
        #[prost(string, tag = "4")]
        pub effective_name: ::prost::alloc::string::String,
        #[prost(int32, tag = "5")]
        pub writes_in_progress: i32,
        #[prost(int32, tag = "6")]
        pub reads_in_progress: i32,
        #[prost(int32, tag = "7")]
        pub partitions_cached: i32,
        #[prost(string, tag = "8")]
        pub status: ::prost::alloc::string::String,
        #[prost(string, tag = "9")]
        pub state_reason: ::prost::alloc::string::String,
        #[prost(string, tag = "10")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag = "11")]
        pub mode: ::prost::alloc::string::String,
        #[prost(string, tag = "12")]
        pub position: ::prost::alloc::string::String,
        #[prost(float, tag = "13")]
        pub progress: f32,
        #[prost(string, tag = "14")]
        pub last_checkpoint: ::prost::alloc::string::String,
        #[prost(int64, tag = "15")]
        pub events_processed_after_restart: i64,
        #[prost(string, tag = "16")]
        pub checkpoint_status: ::prost::alloc::string::String,
        #[prost(int64, tag = "17")]
        pub buffered_events: i64,
        #[prost(int32, tag = "18")]
        pub write_pending_events_before_checkpoint: i32,
        #[prost(int32, tag = "19")]
        pub write_pending_events_after_checkpoint: i32,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<state_req::Options>,
}
/// Nested message and enum types in `StateReq`.
pub mod state_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub partition: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateResp {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<::prost_types::Value>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResultReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<result_req::Options>,
}
/// Nested message and enum types in `ResultReq`.
pub mod result_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub partition: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResultResp {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<::prost_types::Value>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<reset_req::Options>,
}
/// Nested message and enum types in `ResetReq`.
pub mod reset_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(bool, tag = "2")]
        pub write_checkpoint: bool,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetResp {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<enable_req::Options>,
}
/// Nested message and enum types in `EnableReq`.
pub mod enable_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableResp {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<disable_req::Options>,
}
/// Nested message and enum types in `DisableReq`.
pub mod disable_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(bool, tag = "2")]
        pub write_checkpoint: bool,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableResp {}
/// Generated client implementations.
pub mod projections_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ProjectionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProjectionsClient<tonic::transport::Channel> {
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
    impl<T> ProjectionsClient<T>
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
        ) -> ProjectionsClient<InterceptedService<T, F>>
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
            ProjectionsClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReq>,
        ) -> std::result::Result<tonic::Response<super::CreateResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.projections.Projections/Create",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.projections.Projections",
                "Create",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateReq>,
        ) -> std::result::Result<tonic::Response<super::UpdateResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.projections.Projections/Update",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.projections.Projections",
                "Update",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteReq>,
        ) -> std::result::Result<tonic::Response<super::DeleteResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.projections.Projections/Delete",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.projections.Projections",
                "Delete",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn statistics(
            &mut self,
            request: impl tonic::IntoRequest<super::StatisticsReq>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StatisticsResp>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.projections.Projections/Statistics",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.projections.Projections",
                "Statistics",
            ));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn disable(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableReq>,
        ) -> std::result::Result<tonic::Response<super::DisableResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.projections.Projections/Disable",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.projections.Projections",
                "Disable",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn enable(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableReq>,
        ) -> std::result::Result<tonic::Response<super::EnableResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.projections.Projections/Enable",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.projections.Projections",
                "Enable",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetReq>,
        ) -> std::result::Result<tonic::Response<super::ResetResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.projections.Projections/Reset",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.projections.Projections",
                "Reset",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn state(
            &mut self,
            request: impl tonic::IntoRequest<super::StateReq>,
        ) -> std::result::Result<tonic::Response<super::StateResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.projections.Projections/State",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.projections.Projections",
                "State",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn result(
            &mut self,
            request: impl tonic::IntoRequest<super::ResultReq>,
        ) -> std::result::Result<tonic::Response<super::ResultResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.projections.Projections/Result",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.projections.Projections",
                "Result",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn restart_subsystem(
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
                "/event_store.client.projections.Projections/RestartSubsystem",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.projections.Projections",
                "RestartSubsystem",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
