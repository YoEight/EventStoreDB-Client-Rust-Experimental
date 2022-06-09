use crate::event_store::client::shared::Empty;
use crate::event_store::client::streams::append_req::options::ExpectedStreamRevision;
use crate::options::CommonOperationOptions;
use crate::private::Sealed;
use crate::{impl_options_trait, EventData, ExpectedRevision};

#[derive(Clone)]
/// Options of the append to stream command.
pub struct AppendToStreamOptions {
    pub(crate) version: ExpectedStreamRevision,
    pub(crate) common_operation_options: CommonOperationOptions,
}

impl Default for AppendToStreamOptions {
    fn default() -> Self {
        Self {
            version: ExpectedStreamRevision::Any(Empty {}),
            common_operation_options: Default::default(),
        }
    }
}

impl_options_trait!(AppendToStreamOptions);

impl AppendToStreamOptions {
    /// Asks the server to check that the stream receiving the event is at
    /// the given expected version. Default: `ExpectedVersion::Any`.
    pub fn expected_revision(self, version: ExpectedRevision) -> Self {
        let version = match version {
            ExpectedRevision::Any => ExpectedStreamRevision::Any(Empty {}),
            ExpectedRevision::StreamExists => ExpectedStreamRevision::StreamExists(Empty {}),
            ExpectedRevision::NoStream => ExpectedStreamRevision::NoStream(Empty {}),
            ExpectedRevision::Exact(version) => ExpectedStreamRevision::Revision(version),
        };

        Self { version, ..self }
    }
}

pub struct Streaming<I>(pub I);

pub trait ToEvents: Sealed {
    type Events: Iterator<Item = EventData> + Send + 'static;
    fn into_events(self) -> Self::Events;
}

impl ToEvents for EventData {
    type Events = std::option::IntoIter<EventData>;

    fn into_events(self) -> Self::Events {
        Some(self).into_iter()
    }
}

impl ToEvents for Vec<EventData> {
    type Events = std::vec::IntoIter<EventData>;

    fn into_events(self) -> Self::Events {
        self.into_iter()
    }
}

impl<I> ToEvents for Streaming<I>
where
    I: Iterator<Item = EventData> + Send + 'static,
{
    type Events = I;

    fn into_events(self) -> Self::Events {
        self.0
    }
}
