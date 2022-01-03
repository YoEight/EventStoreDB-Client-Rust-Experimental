use crate::event_store::client::shared::Empty;
use crate::event_store::client::streams::append_req::options::ExpectedStreamRevision;
use crate::private::Sealed;
use crate::{Credentials, EventData, ExpectedRevision};
use futures::future::Ready;
use futures::stream::{Iter, Once};
use futures::Stream;

#[derive(Clone)]
/// Options of the append to stream command.
pub struct AppendToStreamOptions {
    pub(crate) version: ExpectedStreamRevision,
    pub(crate) credentials: Option<Credentials>,
}

impl Default for AppendToStreamOptions {
    fn default() -> Self {
        Self {
            version: ExpectedStreamRevision::Any(Empty {}),
            credentials: None,
        }
    }
}

impl AppendToStreamOptions {
    /// Performs the command with the given credentials.
    pub fn authenticated(self, credentials: Credentials) -> Self {
        Self {
            credentials: Some(credentials),
            ..self
        }
    }

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

pub trait ToEvents {
    type Events: Iterator<Item = EventData> + Send;
    fn into_events(self) -> Self::Events;
}

impl ToEvents for EventData {
    type Events = std::option::IntoIter<EventData>;

    fn into_events(self) -> Self::Events {
        Some(self).into_iter()
    }
}

impl<I> ToEvents for I
where
    I: IntoIterator<Item = EventData>,
    <I as IntoIterator>::IntoIter: Send,
{
    type Events = <I as IntoIterator>::IntoIter;

    fn into_events(self) -> Self::Events {
        self.into_iter()
    }
}
