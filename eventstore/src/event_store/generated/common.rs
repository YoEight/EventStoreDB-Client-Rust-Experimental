#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uuid {
    #[prost(oneof = "uuid::Value", tags = "1, 2")]
    pub value: ::core::option::Option<uuid::Value>,
}
/// Nested message and enum types in `UUID`.
pub mod uuid {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Structured {
        #[prost(int64, tag = "1")]
        pub most_significant_bits: i64,
        #[prost(int64, tag = "2")]
        pub least_significant_bits: i64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "1")]
        Structured(Structured),
        #[prost(string, tag = "2")]
        String(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamIdentifier {
    #[prost(bytes = "bytes", tag = "3")]
    pub stream_name: ::prost::bytes::Bytes,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllStreamPosition {
    #[prost(uint64, tag = "1")]
    pub commit_position: u64,
    #[prost(uint64, tag = "2")]
    pub prepare_position: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WrongExpectedVersion {
    #[prost(
        oneof = "wrong_expected_version::CurrentStreamRevisionOption",
        tags = "1, 2"
    )]
    pub current_stream_revision_option:
        ::core::option::Option<wrong_expected_version::CurrentStreamRevisionOption>,
    #[prost(
        oneof = "wrong_expected_version::ExpectedStreamPositionOption",
        tags = "3, 4, 5, 6"
    )]
    pub expected_stream_position_option:
        ::core::option::Option<wrong_expected_version::ExpectedStreamPositionOption>,
}
/// Nested message and enum types in `WrongExpectedVersion`.
pub mod wrong_expected_version {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CurrentStreamRevisionOption {
        #[prost(uint64, tag = "1")]
        CurrentStreamRevision(u64),
        #[prost(message, tag = "2")]
        CurrentNoStream(()),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ExpectedStreamPositionOption {
        #[prost(uint64, tag = "3")]
        ExpectedStreamPosition(u64),
        #[prost(message, tag = "4")]
        ExpectedAny(()),
        #[prost(message, tag = "5")]
        ExpectedStreamExists(()),
        #[prost(message, tag = "6")]
        ExpectedNoStream(()),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessDenied {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamDeleted {
    #[prost(message, optional, tag = "1")]
    pub stream_identifier: ::core::option::Option<StreamIdentifier>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timeout {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Unknown {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvalidTransaction {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaximumAppendSizeExceeded {
    #[prost(uint32, tag = "1")]
    pub max_append_size: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BadRequest {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
