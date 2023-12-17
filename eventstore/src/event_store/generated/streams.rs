#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<read_req::Options>,
}
/// Nested message and enum types in `ReadReq`.
pub mod read_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(enumeration = "options::ReadDirection", tag = "3")]
        pub read_direction: i32,
        #[prost(bool, tag = "4")]
        pub resolve_links: bool,
        #[prost(message, optional, tag = "9")]
        pub uuid_option: ::core::option::Option<options::UuidOption>,
        #[prost(message, optional, tag = "10")]
        pub control_option: ::core::option::Option<options::ControlOption>,
        #[prost(oneof = "options::StreamOption", tags = "1, 2")]
        pub stream_option: ::core::option::Option<options::StreamOption>,
        #[prost(oneof = "options::CountOption", tags = "5, 6")]
        pub count_option: ::core::option::Option<options::CountOption>,
        #[prost(oneof = "options::FilterOption", tags = "7, 8")]
        pub filter_option: ::core::option::Option<options::FilterOption>,
    }
    /// Nested message and enum types in `Options`.
    pub mod options {
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
        pub struct SubscriptionOptions {}
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
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ControlOption {
            #[prost(uint32, tag = "1")]
            pub compatibility: u32,
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum ReadDirection {
            Forwards = 0,
            Backwards = 1,
        }
        impl ReadDirection {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ReadDirection::Forwards => "Forwards",
                    ReadDirection::Backwards => "Backwards",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "Forwards" => Some(Self::Forwards),
                    "Backwards" => Some(Self::Backwards),
                    _ => None,
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum StreamOption {
            #[prost(message, tag = "1")]
            Stream(StreamOptions),
            #[prost(message, tag = "2")]
            All(AllOptions),
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum CountOption {
            #[prost(uint64, tag = "5")]
            Count(u64),
            #[prost(message, tag = "6")]
            Subscription(SubscriptionOptions),
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum FilterOption {
            #[prost(message, tag = "7")]
            Filter(FilterOptions),
            #[prost(message, tag = "8")]
            NoFilter(()),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResp {
    #[prost(oneof = "read_resp::Content", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9")]
    pub content: ::core::option::Option<read_resp::Content>,
}
/// Nested message and enum types in `ReadResp`.
pub mod read_resp {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CaughtUp {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FellBehind {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReadEvent {
        #[prost(message, optional, tag = "1")]
        pub event: ::core::option::Option<read_event::RecordedEvent>,
        #[prost(message, optional, tag = "2")]
        pub link: ::core::option::Option<read_event::RecordedEvent>,
        #[prost(oneof = "read_event::Position", tags = "3, 4")]
        pub position: ::core::option::Option<read_event::Position>,
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
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubscriptionConfirmation {
        #[prost(string, tag = "1")]
        pub subscription_id: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Checkpoint {
        #[prost(uint64, tag = "1")]
        pub commit_position: u64,
        #[prost(uint64, tag = "2")]
        pub prepare_position: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StreamNotFound {
        #[prost(message, optional, tag = "1")]
        pub stream_identifier:
            ::core::option::Option<crate::event_store::generated::common::StreamIdentifier>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "1")]
        Event(ReadEvent),
        #[prost(message, tag = "2")]
        Confirmation(SubscriptionConfirmation),
        #[prost(message, tag = "3")]
        Checkpoint(Checkpoint),
        #[prost(message, tag = "4")]
        StreamNotFound(StreamNotFound),
        #[prost(uint64, tag = "5")]
        FirstStreamPosition(u64),
        #[prost(uint64, tag = "6")]
        LastStreamPosition(u64),
        #[prost(message, tag = "7")]
        LastAllStreamPosition(crate::event_store::generated::common::AllStreamPosition),
        #[prost(message, tag = "8")]
        CaughtUp(CaughtUp),
        #[prost(message, tag = "9")]
        FellBehind(FellBehind),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendReq {
    #[prost(oneof = "append_req::Content", tags = "1, 2")]
    pub content: ::core::option::Option<append_req::Content>,
}
/// Nested message and enum types in `AppendReq`.
pub mod append_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(message, optional, tag = "1")]
        pub stream_identifier:
            ::core::option::Option<crate::event_store::generated::common::StreamIdentifier>,
        #[prost(oneof = "options::ExpectedStreamRevision", tags = "2, 3, 4, 5")]
        pub expected_stream_revision: ::core::option::Option<options::ExpectedStreamRevision>,
    }
    /// Nested message and enum types in `Options`.
    pub mod options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ExpectedStreamRevision {
            #[prost(uint64, tag = "2")]
            Revision(u64),
            #[prost(message, tag = "3")]
            NoStream(()),
            #[prost(message, tag = "4")]
            Any(()),
            #[prost(message, tag = "5")]
            StreamExists(()),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProposedMessage {
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<crate::event_store::generated::common::Uuid>,
        #[prost(map = "string, string", tag = "2")]
        pub metadata: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        #[prost(bytes = "bytes", tag = "3")]
        pub custom_metadata: ::prost::bytes::Bytes,
        #[prost(bytes = "bytes", tag = "4")]
        pub data: ::prost::bytes::Bytes,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "1")]
        Options(Options),
        #[prost(message, tag = "2")]
        ProposedMessage(ProposedMessage),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendResp {
    #[prost(oneof = "append_resp::Result", tags = "1, 2")]
    pub result: ::core::option::Option<append_resp::Result>,
}
/// Nested message and enum types in `AppendResp`.
pub mod append_resp {
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
    pub struct Success {
        #[prost(oneof = "success::CurrentRevisionOption", tags = "1, 2")]
        pub current_revision_option: ::core::option::Option<success::CurrentRevisionOption>,
        #[prost(oneof = "success::PositionOption", tags = "3, 4")]
        pub position_option: ::core::option::Option<success::PositionOption>,
    }
    /// Nested message and enum types in `Success`.
    pub mod success {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum CurrentRevisionOption {
            #[prost(uint64, tag = "1")]
            CurrentRevision(u64),
            #[prost(message, tag = "2")]
            NoStream(()),
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum PositionOption {
            #[prost(message, tag = "3")]
            Position(super::Position),
            #[prost(message, tag = "4")]
            NoPosition(()),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WrongExpectedVersion {
        #[prost(
            oneof = "wrong_expected_version::CurrentRevisionOption2060",
            tags = "1, 2"
        )]
        pub current_revision_option_20_6_0:
            ::core::option::Option<wrong_expected_version::CurrentRevisionOption2060>,
        #[prost(
            oneof = "wrong_expected_version::ExpectedRevisionOption2060",
            tags = "3, 4, 5"
        )]
        pub expected_revision_option_20_6_0:
            ::core::option::Option<wrong_expected_version::ExpectedRevisionOption2060>,
        #[prost(oneof = "wrong_expected_version::CurrentRevisionOption", tags = "6, 7")]
        pub current_revision_option:
            ::core::option::Option<wrong_expected_version::CurrentRevisionOption>,
        #[prost(
            oneof = "wrong_expected_version::ExpectedRevisionOption",
            tags = "8, 9, 10, 11"
        )]
        pub expected_revision_option:
            ::core::option::Option<wrong_expected_version::ExpectedRevisionOption>,
    }
    /// Nested message and enum types in `WrongExpectedVersion`.
    pub mod wrong_expected_version {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum CurrentRevisionOption2060 {
            #[prost(uint64, tag = "1")]
            CurrentRevision2060(u64),
            #[prost(message, tag = "2")]
            NoStream2060(()),
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ExpectedRevisionOption2060 {
            #[prost(uint64, tag = "3")]
            ExpectedRevision2060(u64),
            #[prost(message, tag = "4")]
            Any2060(()),
            #[prost(message, tag = "5")]
            StreamExists2060(()),
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum CurrentRevisionOption {
            #[prost(uint64, tag = "6")]
            CurrentRevision(u64),
            #[prost(message, tag = "7")]
            CurrentNoStream(()),
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ExpectedRevisionOption {
            #[prost(uint64, tag = "8")]
            ExpectedRevision(u64),
            #[prost(message, tag = "9")]
            ExpectedAny(()),
            #[prost(message, tag = "10")]
            ExpectedStreamExists(()),
            #[prost(message, tag = "11")]
            ExpectedNoStream(()),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag = "1")]
        Success(Success),
        #[prost(message, tag = "2")]
        WrongExpectedVersion(WrongExpectedVersion),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchAppendReq {
    #[prost(message, optional, tag = "1")]
    pub correlation_id: ::core::option::Option<crate::event_store::generated::common::Uuid>,
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<batch_append_req::Options>,
    #[prost(message, repeated, tag = "3")]
    pub proposed_messages: ::prost::alloc::vec::Vec<batch_append_req::ProposedMessage>,
    #[prost(bool, tag = "4")]
    pub is_final: bool,
}
/// Nested message and enum types in `BatchAppendReq`.
pub mod batch_append_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(message, optional, tag = "1")]
        pub stream_identifier:
            ::core::option::Option<crate::event_store::generated::common::StreamIdentifier>,
        #[prost(oneof = "options::ExpectedStreamPosition", tags = "2, 3, 4, 5")]
        pub expected_stream_position: ::core::option::Option<options::ExpectedStreamPosition>,
        #[prost(oneof = "options::DeadlineOption", tags = "6, 7")]
        pub deadline_option: ::core::option::Option<options::DeadlineOption>,
    }
    /// Nested message and enum types in `Options`.
    pub mod options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ExpectedStreamPosition {
            #[prost(uint64, tag = "2")]
            StreamPosition(u64),
            #[prost(message, tag = "3")]
            NoStream(()),
            #[prost(message, tag = "4")]
            Any(()),
            #[prost(message, tag = "5")]
            StreamExists(()),
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum DeadlineOption {
            #[prost(message, tag = "6")]
            Deadline21100(::prost_types::Timestamp),
            #[prost(message, tag = "7")]
            Deadline(::prost_types::Duration),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProposedMessage {
        #[prost(message, optional, tag = "1")]
        pub id: ::core::option::Option<crate::event_store::generated::common::Uuid>,
        #[prost(map = "string, string", tag = "2")]
        pub metadata: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        #[prost(bytes = "bytes", tag = "3")]
        pub custom_metadata: ::prost::bytes::Bytes,
        #[prost(bytes = "bytes", tag = "4")]
        pub data: ::prost::bytes::Bytes,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchAppendResp {
    #[prost(message, optional, tag = "1")]
    pub correlation_id: ::core::option::Option<crate::event_store::generated::common::Uuid>,
    #[prost(message, optional, tag = "4")]
    pub stream_identifier:
        ::core::option::Option<crate::event_store::generated::common::StreamIdentifier>,
    #[prost(oneof = "batch_append_resp::Result", tags = "2, 3")]
    pub result: ::core::option::Option<batch_append_resp::Result>,
    #[prost(
        oneof = "batch_append_resp::ExpectedStreamPosition",
        tags = "5, 6, 7, 8"
    )]
    pub expected_stream_position: ::core::option::Option<batch_append_resp::ExpectedStreamPosition>,
}
/// Nested message and enum types in `BatchAppendResp`.
pub mod batch_append_resp {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Success {
        #[prost(oneof = "success::CurrentRevisionOption", tags = "1, 2")]
        pub current_revision_option: ::core::option::Option<success::CurrentRevisionOption>,
        #[prost(oneof = "success::PositionOption", tags = "3, 4")]
        pub position_option: ::core::option::Option<success::PositionOption>,
    }
    /// Nested message and enum types in `Success`.
    pub mod success {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum CurrentRevisionOption {
            #[prost(uint64, tag = "1")]
            CurrentRevision(u64),
            #[prost(message, tag = "2")]
            NoStream(()),
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum PositionOption {
            #[prost(message, tag = "3")]
            Position(crate::event_store::generated::common::AllStreamPosition),
            #[prost(message, tag = "4")]
            NoPosition(()),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag = "2")]
        Error(super::super::super::super::google::rpc::Status),
        #[prost(message, tag = "3")]
        Success(Success),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ExpectedStreamPosition {
        #[prost(uint64, tag = "5")]
        StreamPosition(u64),
        #[prost(message, tag = "6")]
        NoStream(()),
        #[prost(message, tag = "7")]
        Any(()),
        #[prost(message, tag = "8")]
        StreamExists(()),
    }
}
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
        #[prost(message, optional, tag = "1")]
        pub stream_identifier:
            ::core::option::Option<crate::event_store::generated::common::StreamIdentifier>,
        #[prost(oneof = "options::ExpectedStreamRevision", tags = "2, 3, 4, 5")]
        pub expected_stream_revision: ::core::option::Option<options::ExpectedStreamRevision>,
    }
    /// Nested message and enum types in `Options`.
    pub mod options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ExpectedStreamRevision {
            #[prost(uint64, tag = "2")]
            Revision(u64),
            #[prost(message, tag = "3")]
            NoStream(()),
            #[prost(message, tag = "4")]
            Any(()),
            #[prost(message, tag = "5")]
            StreamExists(()),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResp {
    #[prost(oneof = "delete_resp::PositionOption", tags = "1, 2")]
    pub position_option: ::core::option::Option<delete_resp::PositionOption>,
}
/// Nested message and enum types in `DeleteResp`.
pub mod delete_resp {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Position {
        #[prost(uint64, tag = "1")]
        pub commit_position: u64,
        #[prost(uint64, tag = "2")]
        pub prepare_position: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PositionOption {
        #[prost(message, tag = "1")]
        Position(Position),
        #[prost(message, tag = "2")]
        NoPosition(()),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TombstoneReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<tombstone_req::Options>,
}
/// Nested message and enum types in `TombstoneReq`.
pub mod tombstone_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(message, optional, tag = "1")]
        pub stream_identifier:
            ::core::option::Option<crate::event_store::generated::common::StreamIdentifier>,
        #[prost(oneof = "options::ExpectedStreamRevision", tags = "2, 3, 4, 5")]
        pub expected_stream_revision: ::core::option::Option<options::ExpectedStreamRevision>,
    }
    /// Nested message and enum types in `Options`.
    pub mod options {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ExpectedStreamRevision {
            #[prost(uint64, tag = "2")]
            Revision(u64),
            #[prost(message, tag = "3")]
            NoStream(()),
            #[prost(message, tag = "4")]
            Any(()),
            #[prost(message, tag = "5")]
            StreamExists(()),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TombstoneResp {
    #[prost(oneof = "tombstone_resp::PositionOption", tags = "1, 2")]
    pub position_option: ::core::option::Option<tombstone_resp::PositionOption>,
}
/// Nested message and enum types in `TombstoneResp`.
pub mod tombstone_resp {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Position {
        #[prost(uint64, tag = "1")]
        pub commit_position: u64,
        #[prost(uint64, tag = "2")]
        pub prepare_position: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PositionOption {
        #[prost(message, tag = "1")]
        Position(Position),
        #[prost(message, tag = "2")]
        NoPosition(()),
    }
}
/// Generated client implementations.
pub mod streams_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct StreamsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StreamsClient<tonic::transport::Channel> {
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
    impl<T> StreamsClient<T>
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
        ) -> StreamsClient<InterceptedService<T, F>>
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
            StreamsClient::new(InterceptedService::new(inner, interceptor))
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
            request: impl tonic::IntoRequest<super::ReadReq>,
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
            let path =
                http::uri::PathAndQuery::from_static("/event_store.client.streams.Streams/Read");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.streams.Streams",
                "Read",
            ));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn append(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::AppendReq>,
        ) -> std::result::Result<tonic::Response<super::AppendResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/event_store.client.streams.Streams/Append");
            let mut req = request.into_streaming_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.streams.Streams",
                "Append",
            ));
            self.inner.client_streaming(req, path, codec).await
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
                http::uri::PathAndQuery::from_static("/event_store.client.streams.Streams/Delete");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.streams.Streams",
                "Delete",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn tombstone(
            &mut self,
            request: impl tonic::IntoRequest<super::TombstoneReq>,
        ) -> std::result::Result<tonic::Response<super::TombstoneResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.streams.Streams/Tombstone",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.streams.Streams",
                "Tombstone",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_append(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::BatchAppendReq>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::BatchAppendResp>>,
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
                "/event_store.client.streams.Streams/BatchAppend",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "event_store.client.streams.Streams",
                "BatchAppend",
            ));
            self.inner.streaming(req, path, codec).await
        }
    }
}
