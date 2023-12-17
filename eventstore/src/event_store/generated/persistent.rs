#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadReq {
    #[prost(oneof = "read_req::Content", tags = "1, 2, 3")]
    pub content: ::core::option::Option<read_req::Content>,
}
/// Nested message and enum types in `ReadReq`.
pub mod read_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "2")]
        pub group_name: ::prost::alloc::string::String,
        #[prost(int32, tag = "3")]
        pub buffer_size: i32,
        #[prost(message, optional, tag = "4")]
        pub uuid_option: ::core::option::Option<options::UuidOption>,
        #[prost(oneof = "options::StreamOption", tags = "1, 5")]
        pub stream_option: ::core::option::Option<options::StreamOption>,
    }
    /// Nested message and enum types in `Options`.
    pub mod options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct UuidOption {
            #[prost(oneof = "uuid_option::Content", tags = "1, 2")]
            pub content: ::core::option::Option<uuid_option::Content>,
        }
        /// Nested message and enum types in `UUIDOption`.
        pub mod uuid_option {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Content {
                #[prost(message, tag = "1")]
                Structured(()),
                #[prost(message, tag = "2")]
                String(()),
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum StreamOption {
            #[prost(message, tag = "1")]
            StreamIdentifier(crate::event_store::generated::common::StreamIdentifier),
            #[prost(message, tag = "5")]
            All(()),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Ack {
        #[prost(bytes = "vec", tag = "1")]
        pub id: ::prost::alloc::vec::Vec<u8>,
        #[prost(message, repeated, tag = "2")]
        pub ids: ::prost::alloc::vec::Vec<crate::event_store::generated::common::Uuid>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Nack {
        #[prost(bytes = "vec", tag = "1")]
        pub id: ::prost::alloc::vec::Vec<u8>,
        #[prost(message, repeated, tag = "2")]
        pub ids: ::prost::alloc::vec::Vec<crate::event_store::generated::common::Uuid>,
        #[prost(enumeration = "nack::Action", tag = "3")]
        pub action: i32,
        #[prost(string, tag = "4")]
        pub reason: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Nack`.
    pub mod nack {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Action {
            Unknown = 0,
            Park = 1,
            Retry = 2,
            Skip = 3,
            Stop = 4,
        }
        impl Action {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Action::Unknown => "Unknown",
                    Action::Park => "Park",
                    Action::Retry => "Retry",
                    Action::Skip => "Skip",
                    Action::Stop => "Stop",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "Unknown" => Some(Self::Unknown),
                    "Park" => Some(Self::Park),
                    "Retry" => Some(Self::Retry),
                    "Skip" => Some(Self::Skip),
                    "Stop" => Some(Self::Stop),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "1")]
        Options(Options),
        #[prost(message, tag = "2")]
        Ack(Ack),
        #[prost(message, tag = "3")]
        Nack(Nack),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResp {
    #[prost(oneof = "read_resp::Content", tags = "1, 2")]
    pub content: ::core::option::Option<read_resp::Content>,
}
/// Nested message and enum types in `ReadResp`.
pub mod read_resp {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReadEvent {
        #[prost(message, optional, tag = "1")]
        pub event: ::core::option::Option<read_event::RecordedEvent>,
        #[prost(message, optional, tag = "2")]
        pub link: ::core::option::Option<read_event::RecordedEvent>,
        #[prost(oneof = "read_event::Position", tags = "3, 4")]
        pub position: ::core::option::Option<read_event::Position>,
        #[prost(oneof = "read_event::Count", tags = "5, 6")]
        pub count: ::core::option::Option<read_event::Count>,
    }
    /// Nested message and enum types in `ReadEvent`.
    pub mod read_event {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RecordedEvent {
            #[prost(message, optional, tag = "1")]
            pub id: ::core::option::Option<crate::event_store::generated::common::Uuid>,
            #[prost(message, optional, tag = "2")]
            pub stream_identifier:
                ::core::option::Option<crate::event_store::generated::common::StreamIdentifier>,
            #[prost(uint64, tag = "3")]
            pub stream_revision: u64,
            #[prost(uint64, tag = "4")]
            pub prepare_position: u64,
            #[prost(uint64, tag = "5")]
            pub commit_position: u64,
            #[prost(map = "string, string", tag = "6")]
            pub metadata: ::std::collections::HashMap<
                ::prost::alloc::string::String,
                ::prost::alloc::string::String,
            >,
            #[prost(bytes = "bytes", tag = "7")]
            pub custom_metadata: ::prost::bytes::Bytes,
            #[prost(bytes = "bytes", tag = "8")]
            pub data: ::prost::bytes::Bytes,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Position {
            #[prost(uint64, tag = "3")]
            CommitPosition(u64),
            #[prost(message, tag = "4")]
            NoPosition(()),
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Count {
            #[prost(int32, tag = "5")]
            RetryCount(i32),
            #[prost(message, tag = "6")]
            NoRetryCount(()),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubscriptionConfirmation {
        #[prost(string, tag = "1")]
        pub subscription_id: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "1")]
        Event(ReadEvent),
        #[prost(message, tag = "2")]
        SubscriptionConfirmation(SubscriptionConfirmation),
    }
}
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
        #[deprecated]
        #[prost(message, optional, tag = "1")]
        pub stream_identifier:
            ::core::option::Option<crate::event_store::generated::common::StreamIdentifier>,
        #[prost(string, tag = "2")]
        pub group_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "3")]
        pub settings: ::core::option::Option<Settings>,
        #[prost(oneof = "options::StreamOption", tags = "4, 5")]
        pub stream_option: ::core::option::Option<options::StreamOption>,
    }
    /// Nested message and enum types in `Options`.
    pub mod options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum StreamOption {
            #[prost(message, tag = "4")]
            Stream(super::StreamOptions),
            #[prost(message, tag = "5")]
            All(super::AllOptions),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StreamOptions {
        #[prost(message, optional, tag = "1")]
        pub stream_identifier:
            ::core::option::Option<crate::event_store::generated::common::StreamIdentifier>,
        #[prost(oneof = "stream_options::RevisionOption", tags = "2, 3, 4")]
        pub revision_option: ::core::option::Option<stream_options::RevisionOption>,
    }
    /// Nested message and enum types in `StreamOptions`.
    pub mod stream_options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum RevisionOption {
            #[prost(uint64, tag = "2")]
            Revision(u64),
            #[prost(message, tag = "3")]
            Start(()),
            #[prost(message, tag = "4")]
            End(()),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AllOptions {
        #[prost(oneof = "all_options::AllOption", tags = "1, 2, 3")]
        pub all_option: ::core::option::Option<all_options::AllOption>,
        #[prost(oneof = "all_options::FilterOption", tags = "4, 5")]
        pub filter_option: ::core::option::Option<all_options::FilterOption>,
    }
    /// Nested message and enum types in `AllOptions`.
    pub mod all_options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FilterOptions {
            #[prost(uint32, tag = "5")]
            pub checkpoint_interval_multiplier: u32,
            #[prost(oneof = "filter_options::Filter", tags = "1, 2")]
            pub filter: ::core::option::Option<filter_options::Filter>,
            #[prost(oneof = "filter_options::Window", tags = "3, 4")]
            pub window: ::core::option::Option<filter_options::Window>,
        }
        /// Nested message and enum types in `FilterOptions`.
        pub mod filter_options {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Expression {
                #[prost(string, tag = "1")]
                pub regex: ::prost::alloc::string::String,
                #[prost(string, repeated, tag = "2")]
                pub prefix: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Filter {
                #[prost(message, tag = "1")]
                StreamIdentifier(Expression),
                #[prost(message, tag = "2")]
                EventType(Expression),
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Window {
                #[prost(uint32, tag = "3")]
                Max(u32),
                #[prost(message, tag = "4")]
                Count(()),
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum AllOption {
            #[prost(message, tag = "1")]
            Position(super::Position),
            #[prost(message, tag = "2")]
            Start(()),
            #[prost(message, tag = "3")]
            End(()),
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum FilterOption {
            #[prost(message, tag = "4")]
            Filter(FilterOptions),
            #[prost(message, tag = "5")]
            NoFilter(()),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Position {
        #[prost(uint64, tag = "1")]
        pub commit_position: u64,
        #[prost(uint64, tag = "2")]
        pub prepare_position: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Settings {
        #[prost(bool, tag = "1")]
        pub resolve_links: bool,
        #[deprecated]
        #[prost(uint64, tag = "2")]
        pub revision: u64,
        #[prost(bool, tag = "3")]
        pub extra_statistics: bool,
        #[prost(int32, tag = "5")]
        pub max_retry_count: i32,
        #[prost(int32, tag = "7")]
        pub min_checkpoint_count: i32,
        #[prost(int32, tag = "8")]
        pub max_checkpoint_count: i32,
        #[prost(int32, tag = "9")]
        pub max_subscriber_count: i32,
        #[prost(int32, tag = "10")]
        pub live_buffer_size: i32,
        #[prost(int32, tag = "11")]
        pub read_batch_size: i32,
        #[prost(int32, tag = "12")]
        pub history_buffer_size: i32,
        #[deprecated]
        #[prost(enumeration = "ConsumerStrategy", tag = "13")]
        pub named_consumer_strategy: i32,
        #[prost(string, tag = "16")]
        pub consumer_strategy: ::prost::alloc::string::String,
        #[prost(oneof = "settings::MessageTimeout", tags = "4, 14")]
        pub message_timeout: ::core::option::Option<settings::MessageTimeout>,
        #[prost(oneof = "settings::CheckpointAfter", tags = "6, 15")]
        pub checkpoint_after: ::core::option::Option<settings::CheckpointAfter>,
    }
    /// Nested message and enum types in `Settings`.
    pub mod settings {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum MessageTimeout {
            #[prost(int64, tag = "4")]
            MessageTimeoutTicks(i64),
            #[prost(int32, tag = "14")]
            MessageTimeoutMs(i32),
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum CheckpointAfter {
            #[prost(int64, tag = "6")]
            CheckpointAfterTicks(i64),
            #[prost(int32, tag = "15")]
            CheckpointAfterMs(i32),
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConsumerStrategy {
        DispatchToSingle = 0,
        RoundRobin = 1,
        Pinned = 2,
    }
    impl ConsumerStrategy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConsumerStrategy::DispatchToSingle => "DispatchToSingle",
                ConsumerStrategy::RoundRobin => "RoundRobin",
                ConsumerStrategy::Pinned => "Pinned",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DispatchToSingle" => Some(Self::DispatchToSingle),
                "RoundRobin" => Some(Self::RoundRobin),
                "Pinned" => Some(Self::Pinned),
                _ => None,
            }
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
        #[deprecated]
        #[prost(message, optional, tag = "1")]
        pub stream_identifier:
            ::core::option::Option<crate::event_store::generated::common::StreamIdentifier>,
        #[prost(string, tag = "2")]
        pub group_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "3")]
        pub settings: ::core::option::Option<Settings>,
        #[prost(oneof = "options::StreamOption", tags = "4, 5")]
        pub stream_option: ::core::option::Option<options::StreamOption>,
    }
    /// Nested message and enum types in `Options`.
    pub mod options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum StreamOption {
            #[prost(message, tag = "4")]
            Stream(super::StreamOptions),
            #[prost(message, tag = "5")]
            All(super::AllOptions),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StreamOptions {
        #[prost(message, optional, tag = "1")]
        pub stream_identifier:
            ::core::option::Option<crate::event_store::generated::common::StreamIdentifier>,
        #[prost(oneof = "stream_options::RevisionOption", tags = "2, 3, 4")]
        pub revision_option: ::core::option::Option<stream_options::RevisionOption>,
    }
    /// Nested message and enum types in `StreamOptions`.
    pub mod stream_options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum RevisionOption {
            #[prost(uint64, tag = "2")]
            Revision(u64),
            #[prost(message, tag = "3")]
            Start(()),
            #[prost(message, tag = "4")]
            End(()),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AllOptions {
        #[prost(oneof = "all_options::AllOption", tags = "1, 2, 3")]
        pub all_option: ::core::option::Option<all_options::AllOption>,
    }
    /// Nested message and enum types in `AllOptions`.
    pub mod all_options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum AllOption {
            #[prost(message, tag = "1")]
            Position(super::Position),
            #[prost(message, tag = "2")]
            Start(()),
            #[prost(message, tag = "3")]
            End(()),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Position {
        #[prost(uint64, tag = "1")]
        pub commit_position: u64,
        #[prost(uint64, tag = "2")]
        pub prepare_position: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Settings {
        #[prost(bool, tag = "1")]
        pub resolve_links: bool,
        #[deprecated]
        #[prost(uint64, tag = "2")]
        pub revision: u64,
        #[prost(bool, tag = "3")]
        pub extra_statistics: bool,
        #[prost(int32, tag = "5")]
        pub max_retry_count: i32,
        #[prost(int32, tag = "7")]
        pub min_checkpoint_count: i32,
        #[prost(int32, tag = "8")]
        pub max_checkpoint_count: i32,
        #[prost(int32, tag = "9")]
        pub max_subscriber_count: i32,
        #[prost(int32, tag = "10")]
        pub live_buffer_size: i32,
        #[prost(int32, tag = "11")]
        pub read_batch_size: i32,
        #[prost(int32, tag = "12")]
        pub history_buffer_size: i32,
        #[prost(enumeration = "ConsumerStrategy", tag = "13")]
        pub named_consumer_strategy: i32,
        #[prost(oneof = "settings::MessageTimeout", tags = "4, 14")]
        pub message_timeout: ::core::option::Option<settings::MessageTimeout>,
        #[prost(oneof = "settings::CheckpointAfter", tags = "6, 15")]
        pub checkpoint_after: ::core::option::Option<settings::CheckpointAfter>,
    }
    /// Nested message and enum types in `Settings`.
    pub mod settings {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum MessageTimeout {
            #[prost(int64, tag = "4")]
            MessageTimeoutTicks(i64),
            #[prost(int32, tag = "14")]
            MessageTimeoutMs(i32),
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum CheckpointAfter {
            #[prost(int64, tag = "6")]
            CheckpointAfterTicks(i64),
            #[prost(int32, tag = "15")]
            CheckpointAfterMs(i32),
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConsumerStrategy {
        DispatchToSingle = 0,
        RoundRobin = 1,
        Pinned = 2,
    }
    impl ConsumerStrategy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConsumerStrategy::DispatchToSingle => "DispatchToSingle",
                ConsumerStrategy::RoundRobin => "RoundRobin",
                ConsumerStrategy::Pinned => "Pinned",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DispatchToSingle" => Some(Self::DispatchToSingle),
                "RoundRobin" => Some(Self::RoundRobin),
                "Pinned" => Some(Self::Pinned),
                _ => None,
            }
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
        #[prost(string, tag = "2")]
        pub group_name: ::prost::alloc::string::String,
        #[prost(oneof = "options::StreamOption", tags = "1, 3")]
        pub stream_option: ::core::option::Option<options::StreamOption>,
    }
    /// Nested message and enum types in `Options`.
    pub mod options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum StreamOption {
            #[prost(message, tag = "1")]
            StreamIdentifier(crate::event_store::generated::common::StreamIdentifier),
            #[prost(message, tag = "3")]
            All(()),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResp {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<get_info_req::Options>,
}
/// Nested message and enum types in `GetInfoReq`.
pub mod get_info_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "3")]
        pub group_name: ::prost::alloc::string::String,
        #[prost(oneof = "options::StreamOption", tags = "1, 2")]
        pub stream_option: ::core::option::Option<options::StreamOption>,
    }
    /// Nested message and enum types in `Options`.
    pub mod options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum StreamOption {
            #[prost(message, tag = "1")]
            StreamIdentifier(crate::event_store::generated::common::StreamIdentifier),
            #[prost(message, tag = "2")]
            All(()),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoResp {
    #[prost(message, optional, tag = "1")]
    pub subscription_info: ::core::option::Option<SubscriptionInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionInfo {
    #[prost(string, tag = "1")]
    pub event_source: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub connections: ::prost::alloc::vec::Vec<subscription_info::ConnectionInfo>,
    #[prost(int32, tag = "5")]
    pub average_per_second: i32,
    #[prost(int64, tag = "6")]
    pub total_items: i64,
    #[prost(int64, tag = "7")]
    pub count_since_last_measurement: i64,
    #[prost(string, tag = "8")]
    pub last_checkpointed_event_position: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub last_known_event_position: ::prost::alloc::string::String,
    #[prost(bool, tag = "10")]
    pub resolve_link_tos: bool,
    #[prost(string, tag = "11")]
    pub start_from: ::prost::alloc::string::String,
    #[prost(int32, tag = "12")]
    pub message_timeout_milliseconds: i32,
    #[prost(bool, tag = "13")]
    pub extra_statistics: bool,
    #[prost(int32, tag = "14")]
    pub max_retry_count: i32,
    #[prost(int32, tag = "15")]
    pub live_buffer_size: i32,
    #[prost(int32, tag = "16")]
    pub buffer_size: i32,
    #[prost(int32, tag = "17")]
    pub read_batch_size: i32,
    #[prost(int32, tag = "18")]
    pub check_point_after_milliseconds: i32,
    #[prost(int32, tag = "19")]
    pub min_check_point_count: i32,
    #[prost(int32, tag = "20")]
    pub max_check_point_count: i32,
    #[prost(int32, tag = "21")]
    pub read_buffer_count: i32,
    #[prost(int64, tag = "22")]
    pub live_buffer_count: i64,
    #[prost(int32, tag = "23")]
    pub retry_buffer_count: i32,
    #[prost(int32, tag = "24")]
    pub total_in_flight_messages: i32,
    #[prost(int32, tag = "25")]
    pub outstanding_messages_count: i32,
    #[prost(string, tag = "26")]
    pub named_consumer_strategy: ::prost::alloc::string::String,
    #[prost(int32, tag = "27")]
    pub max_subscriber_count: i32,
    #[prost(int64, tag = "28")]
    pub parked_message_count: i64,
}
/// Nested message and enum types in `SubscriptionInfo`.
pub mod subscription_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConnectionInfo {
        #[prost(string, tag = "1")]
        pub from: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub username: ::prost::alloc::string::String,
        #[prost(int32, tag = "3")]
        pub average_items_per_second: i32,
        #[prost(int64, tag = "4")]
        pub total_items: i64,
        #[prost(int64, tag = "5")]
        pub count_since_last_measurement: i64,
        #[prost(message, repeated, tag = "6")]
        pub observed_measurements: ::prost::alloc::vec::Vec<Measurement>,
        #[prost(int32, tag = "7")]
        pub available_slots: i32,
        #[prost(int32, tag = "8")]
        pub in_flight_messages: i32,
        #[prost(string, tag = "9")]
        pub connection_name: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Measurement {
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplayParkedReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<replay_parked_req::Options>,
}
/// Nested message and enum types in `ReplayParkedReq`.
pub mod replay_parked_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(string, tag = "1")]
        pub group_name: ::prost::alloc::string::String,
        #[prost(oneof = "options::StreamOption", tags = "2, 3")]
        pub stream_option: ::core::option::Option<options::StreamOption>,
        #[prost(oneof = "options::StopAtOption", tags = "4, 5")]
        pub stop_at_option: ::core::option::Option<options::StopAtOption>,
    }
    /// Nested message and enum types in `Options`.
    pub mod options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum StreamOption {
            #[prost(message, tag = "2")]
            StreamIdentifier(crate::event_store::generated::common::StreamIdentifier),
            #[prost(message, tag = "3")]
            All(()),
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum StopAtOption {
            #[prost(int64, tag = "4")]
            StopAt(i64),
            #[prost(message, tag = "5")]
            NoLimit(()),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplayParkedResp {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<list_req::Options>,
}
/// Nested message and enum types in `ListReq`.
pub mod list_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(oneof = "options::ListOption", tags = "1, 2")]
        pub list_option: ::core::option::Option<options::ListOption>,
    }
    /// Nested message and enum types in `Options`.
    pub mod options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ListOption {
            #[prost(message, tag = "1")]
            ListAllSubscriptions(()),
            #[prost(message, tag = "2")]
            ListForStream(super::StreamOption),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StreamOption {
        #[prost(oneof = "stream_option::StreamOption", tags = "1, 2")]
        pub stream_option: ::core::option::Option<stream_option::StreamOption>,
    }
    /// Nested message and enum types in `StreamOption`.
    pub mod stream_option {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum StreamOption {
            #[prost(message, tag = "1")]
            Stream(crate::event_store::generated::common::StreamIdentifier),
            #[prost(message, tag = "2")]
            All(()),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResp {
    #[prost(message, repeated, tag = "1")]
    pub subscriptions: ::prost::alloc::vec::Vec<SubscriptionInfo>,
}
/// Generated client implementations.
pub mod persistent_subscriptions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct PersistentSubscriptionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PersistentSubscriptionsClient<tonic::transport::Channel> {
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
    impl<T> PersistentSubscriptionsClient<T>
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
        ) -> PersistentSubscriptionsClient<InterceptedService<T, F>>
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
            PersistentSubscriptionsClient::new(InterceptedService::new(inner, interceptor))
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
                "/event_store.client.persistent_subscriptions.PersistentSubscriptions/Create",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.persistent_subscriptions.PersistentSubscriptions",
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
                "/event_store.client.persistent_subscriptions.PersistentSubscriptions/Update",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.persistent_subscriptions.PersistentSubscriptions",
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
                "/event_store.client.persistent_subscriptions.PersistentSubscriptions/Delete",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.persistent_subscriptions.PersistentSubscriptions",
                "Delete",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ReadReq>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ReadResp>>,
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
                "/event_store.client.persistent_subscriptions.PersistentSubscriptions/Read",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.persistent_subscriptions.PersistentSubscriptions",
                "Read",
            ));
            self.inner.streaming(req, path, codec).await
        }
        pub async fn get_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInfoReq>,
        ) -> std::result::Result<tonic::Response<super::GetInfoResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.persistent_subscriptions.PersistentSubscriptions/GetInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.persistent_subscriptions.PersistentSubscriptions",
                "GetInfo",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn replay_parked(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplayParkedReq>,
        ) -> std::result::Result<tonic::Response<super::ReplayParkedResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.persistent_subscriptions.PersistentSubscriptions/ReplayParked",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.persistent_subscriptions.PersistentSubscriptions",
                "ReplayParked",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReq>,
        ) -> std::result::Result<tonic::Response<super::ListResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.persistent_subscriptions.PersistentSubscriptions/List",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.persistent_subscriptions.PersistentSubscriptions",
                "List",
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
                "/event_store.client.persistent_subscriptions.PersistentSubscriptions/RestartSubsystem",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.persistent_subscriptions.PersistentSubscriptions",
                "RestartSubsystem",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
