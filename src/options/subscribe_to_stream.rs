use std::time::Duration;

use crate::options::retry::RetryOptions;
use crate::{impl_options_trait, Credentials, StreamPosition};

#[derive(Clone)]
pub struct SubscribeToStreamOptions {
    pub(crate) credentials: Option<Credentials>,
    pub(crate) position: StreamPosition<u64>,
    pub(crate) resolve_link_tos: bool,
    pub(crate) retry: Option<RetryOptions>,
    pub(crate) require_leader: bool,
    pub(crate) deadline: Option<Duration>,
}

impl Default for SubscribeToStreamOptions {
    fn default() -> Self {
        Self {
            credentials: None,
            position: StreamPosition::End,
            resolve_link_tos: false,
            retry: None,
            require_leader: false,
            deadline: None,
        }
    }
}

impl_options_trait!(SubscribeToStreamOptions);

impl SubscribeToStreamOptions {
    /// For example, if a starting point of 50 is specified when a stream has
    /// 100 events in it, the subscriber can expect to see events 51 through
    /// 100, and then any events subsequently written events until such time
    /// as the subscription is dropped or closed.
    ///
    /// By default, it's `StreamPosition::End`
    pub fn start_from(self, position: StreamPosition<u64>) -> Self {
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

    /// When a disconnection happens, automatically resubscribe to stream changes. When enabled,
    /// The client will keep track of the current subscription offset.
    pub fn retry_options(self, options: RetryOptions) -> Self {
        Self {
            retry: Some(options),
            ..self
        }
    }
}
