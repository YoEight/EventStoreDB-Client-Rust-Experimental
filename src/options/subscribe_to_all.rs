use std::time::Duration;

use crate::options::retry::RetryOptions;
use crate::{impl_options_trait, Credentials, Position, StreamPosition, SubscriptionFilter};

#[derive(Clone)]
pub struct SubscribeToAllOptions {
    pub(crate) credentials: Option<Credentials>,
    pub(crate) position: StreamPosition<Position>,
    pub(crate) resolve_link_tos: bool,
    pub(crate) filter: Option<SubscriptionFilter>,
    pub(crate) retry: Option<RetryOptions>,
    pub(crate) require_leader: bool,
    pub(crate) deadline: Option<Duration>,
}

impl Default for SubscribeToAllOptions {
    fn default() -> Self {
        Self {
            filter: None,
            credentials: None,
            position: StreamPosition::Start,
            resolve_link_tos: false,
            retry: None,
            require_leader: false,
            deadline: None,
        }
    }
}

impl_options_trait!(SubscribeToAllOptions);

impl SubscribeToAllOptions {
    /// Starting point in the transaction journal log. By default, it will start at
    /// `StreamPosition::Start`
    pub fn position(self, position: StreamPosition<Position>) -> Self {
        Self { position, ..self }
    }

    /// Filters events or streams based upon a predicate.
    pub fn filter(self, filter: SubscriptionFilter) -> Self {
        Self {
            filter: Some(filter),
            ..self
        }
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
