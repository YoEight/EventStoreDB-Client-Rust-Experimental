use crate::{Credentials, StreamPosition};

#[derive(Clone)]
pub struct SubscribeToStreamOptions {
    pub(crate) credentials: Option<Credentials>,
    pub(crate) position: StreamPosition<u64>,
    pub(crate) resolve_link_tos: bool,
}

impl Default for SubscribeToStreamOptions {
    fn default() -> Self {
        Self {
            credentials: None,
            position: StreamPosition::Start,
            resolve_link_tos: false,
        }
    }
}

impl SubscribeToStreamOptions {
    /// Performs the command with the given credentials.
    pub fn authenticated(self, value: Credentials) -> Self {
        Self {
            credentials: Some(value),
            ..self
        }
    }

    /// For example, if a starting point of 50 is specified when a stream has
    /// 100 events in it, the subscriber can expect to see events 51 through
    /// 100, and then any events subsequently written events until such time
    /// as the subscription is dropped or closed.
    ///
    /// By default, it's `Origin::Start`
    pub fn position(self, position: StreamPosition<u64>) -> Self {
        Self { position, ..self }
    }

    /// When using projections, you can have links placed into another stream.
    /// If you set `true`, the server will resolve those links and will return
    /// the event that the link points to. Default: [NoResolution](../types/enum.LinkTos.html).
    pub fn resolve_link_tos(self) -> Self {
        Self {
            resolve_link_tos: true,
            ..self
        }
    }
}
