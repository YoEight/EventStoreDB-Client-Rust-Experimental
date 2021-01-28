#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uuid {
    #[prost(oneof = "uuid::Value", tags = "1, 2")]
    pub value: ::core::option::Option<uuid::Value>,
}
/// Nested message and enum types in `UUID`.
pub mod uuid {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Structured {
        #[prost(int64, tag = "1")]
        pub most_significant_bits: i64,
        #[prost(int64, tag = "2")]
        pub least_significant_bits: i64,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "1")]
        Structured(Structured),
        #[prost(string, tag = "2")]
        String(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamIdentifier {
    #[prost(bytes = "vec", tag = "3")]
    pub stream_name: ::prost::alloc::vec::Vec<u8>,
}
