use std::time::Duration;

use crate::event_store::client::shared::Empty;
use crate::event_store::client::streams::append_req::options::ExpectedStreamRevision;
use crate::private::Sealed;
use crate::{impl_options_trait, Credentials, EventData, ExpectedRevision};
use futures::future::Ready;
use futures::stream::{Iter, Once};
use futures::Stream;

#[derive(Clone)]
/// Options of the append to stream command.
pub struct AppendToStreamOptions {
    pub(crate) version: ExpectedStreamRevision,
    pub(crate) credentials: Option<Credentials>,
    pub(crate) require_leader: bool,
    pub(crate) deadline: Option<Duration>,
}

impl Default for AppendToStreamOptions {
    fn default() -> Self {
        Self {
            version: ExpectedStreamRevision::Any(Empty {}),
            credentials: None,
            require_leader: false,
            deadline: None,
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

pub struct Streaming<S>(S);

pub trait ToEvents: Sealed {
    type Events: Stream<Item = EventData> + Send + Sync;
    fn into_events(self) -> Self::Events;
}

impl ToEvents for EventData {
    type Events = Once<Ready<EventData>>;

    fn into_events(self) -> Self::Events {
        futures::stream::once(futures::future::ready(self))
    }
}

impl ToEvents for Vec<EventData> {
    type Events = Iter<std::vec::IntoIter<EventData>>;

    fn into_events(self) -> Self::Events {
        futures::stream::iter(self)
    }
}

impl<S> ToEvents for Streaming<S>
where
    S: Stream<Item = EventData> + Send + Sync,
{
    type Events = S;

    fn into_events(self) -> Self::Events {
        self.0
    }
}
