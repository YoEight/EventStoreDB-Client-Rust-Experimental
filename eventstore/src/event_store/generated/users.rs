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
        #[prost(string, tag = "1")]
        pub login_name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub password: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub full_name: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "4")]
        pub groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
        pub login_name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub password: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub full_name: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "4")]
        pub groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
        pub login_name: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResp {}
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
        pub login_name: ::prost::alloc::string::String,
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
        pub login_name: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableResp {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetailsReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<details_req::Options>,
}
/// Nested message and enum types in `DetailsReq`.
pub mod details_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "1")]
        pub login_name: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetailsResp {
    #[prost(message, optional, tag = "1")]
    pub user_details: ::core::option::Option<details_resp::UserDetails>,
}
/// Nested message and enum types in `DetailsResp`.
pub mod details_resp {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserDetails {
        #[prost(string, tag = "1")]
        pub login_name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub full_name: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "3")]
        pub groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, optional, tag = "4")]
        pub last_updated: ::core::option::Option<user_details::DateTime>,
        #[prost(bool, tag = "5")]
        pub disabled: bool,
    }
    /// Nested message and enum types in `UserDetails`.
    pub mod user_details {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DateTime {
            #[prost(int64, tag = "1")]
            pub ticks_since_epoch: i64,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePasswordReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<change_password_req::Options>,
}
/// Nested message and enum types in `ChangePasswordReq`.
pub mod change_password_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "1")]
        pub login_name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub current_password: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub new_password: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePasswordResp {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPasswordReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<reset_password_req::Options>,
}
/// Nested message and enum types in `ResetPasswordReq`.
pub mod reset_password_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "1")]
        pub login_name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub new_password: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPasswordResp {}
/// Generated client implementations.
pub mod users_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct UsersClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UsersClient<tonic::transport::Channel> {
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
    impl<T> UsersClient<T>
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
        ) -> UsersClient<InterceptedService<T, F>>
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
            UsersClient::new(InterceptedService::new(inner, interceptor))
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
            let path =
                http::uri::PathAndQuery::from_static("/event_store.client.users.Users/Create");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("event_store.client.users.Users", "Create"));
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
            let path =
                http::uri::PathAndQuery::from_static("/event_store.client.users.Users/Update");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("event_store.client.users.Users", "Update"));
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
            let path =
                http::uri::PathAndQuery::from_static("/event_store.client.users.Users/Delete");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("event_store.client.users.Users", "Delete"));
            self.inner.unary(req, path, codec).await
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
            let path =
                http::uri::PathAndQuery::from_static("/event_store.client.users.Users/Disable");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("event_store.client.users.Users", "Disable"));
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
            let path =
                http::uri::PathAndQuery::from_static("/event_store.client.users.Users/Enable");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("event_store.client.users.Users", "Enable"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn details(
            &mut self,
            request: impl tonic::IntoRequest<super::DetailsReq>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::DetailsResp>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/event_store.client.users.Users/Details");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("event_store.client.users.Users", "Details"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn change_password(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangePasswordReq>,
        ) -> std::result::Result<tonic::Response<super::ChangePasswordResp>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.users.Users/ChangePassword",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.users.Users",
                "ChangePassword",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_password(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetPasswordReq>,
        ) -> std::result::Result<tonic::Response<super::ResetPasswordResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.users.Users/ResetPassword",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.users.Users",
                "ResetPassword",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
