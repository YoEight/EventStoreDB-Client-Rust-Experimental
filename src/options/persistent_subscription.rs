use crate::options::CommonOperationOptions;
use crate::{
    impl_options_trait, Credentials, PersistentSubscriptionSettings, Position, StreamPosition,
    SubscriptionFilter, SystemConsumerStrategy,
};
use std::time::Duration;

#[derive(Clone, Default)]
pub struct PersistentSubscriptionOptions {
    pub(crate) setts: PersistentSubscriptionSettings<u64>,
    pub(crate) common_operation_options: CommonOperationOptions,
}

impl_options_trait!(PersistentSubscriptionOptions);

impl PersistentSubscriptionOptions {
    /// Applies the specified persistent subscription settings.
    pub fn settings(self, setts: PersistentSubscriptionSettings<u64>) -> Self {
        Self { setts, ..self }
    }

    /// Whether or not the persistent subscription should resolve link
    /// events to their linked events.
    pub fn resolve_link_tos(mut self, value: bool) -> Self {
        self.setts.resolve_link_tos = value;
        self
    }

    /// Where the subscription should start from (event number).
    pub fn start_from(mut self, position: StreamPosition<u64>) -> Self {
        self.setts.start_from = position;
        self
    }

    /// Whether or not in depth latency statistics should be tracked on this
    /// subscription.
    pub fn extra_statistics(mut self, value: bool) -> Self {
        self.setts.extra_statistics = value;
        self
    }

    /// The amount of time after which a message should be considered to be
    /// timeout and retried.
    pub fn message_timeout(mut self, value: Duration) -> Self {
        self.setts.message_timeout = value;
        self
    }

    /// The maximum number of retries (due to timeout) before a message get
    /// considered to be parked.
    pub fn max_retry_count(mut self, value: i32) -> Self {
        self.setts.max_retry_count = value;
        self
    }

    /// The size of the buffer listening to live messages as they happen.
    pub fn live_buffer_size(mut self, value: i32) -> Self {
        self.setts.live_buffer_size = value;
        self
    }

    /// The number of events read at a time when paging in history.
    pub fn read_batch_size(mut self, value: i32) -> Self {
        self.setts.read_batch_size = value;
        self
    }

    /// The number of events to cache when paging through history.
    pub fn history_buffer_size(mut self, value: i32) -> Self {
        self.setts.history_buffer_size = value;
        self
    }

    /// The amount of time to try checkpoint after.
    pub fn checkpoint_after(mut self, value: Duration) -> Self {
        self.setts.checkpoint_after = value;
        self
    }

    /// The minimum number of messages to checkpoint.
    pub fn checkpoint_lower_bound(mut self, value: i32) -> Self {
        self.setts.checkpoint_lower_bound = value;
        self
    }

    /// The minimum number of messages to checkpoint.
    pub fn checkpoint_upper_bound(mut self, value: i32) -> Self {
        self.setts.checkpoint_upper_bound = value;
        self
    }

    /// The maximum number of subscribers allowed.
    pub fn max_subscriber_count(mut self, value: i32) -> Self {
        self.setts.max_subscriber_count = value;
        self
    }

    /// The strategy to use for distributing events to client consumers.
    pub fn consumer_strategy_name(mut self, value: SystemConsumerStrategy) -> Self {
        self.setts.consumer_strategy_name = value;
        self
    }

    pub fn settings_mut(&mut self) -> &mut PersistentSubscriptionSettings<u64> {
        &mut self.setts
    }
}

#[derive(Clone, Default)]
pub struct PersistentSubscriptionToAllOptions {
    pub(crate) setts: PersistentSubscriptionSettings<Position>,
    pub(crate) filter: Option<SubscriptionFilter>,
    pub(crate) common_operation_options: CommonOperationOptions,
}

impl_options_trait!(PersistentSubscriptionToAllOptions);

impl PersistentSubscriptionToAllOptions {
    /// Applies the specified persistent subscription settings.
    pub fn settings(self, setts: PersistentSubscriptionSettings<Position>) -> Self {
        Self { setts, ..self }
    }

    /// Filters events or streams based upon a predicate.
    pub fn filter(self, filter: SubscriptionFilter) -> Self {
        Self {
            filter: Some(filter),
            ..self
        }
    }

    /// Whether or not the persistent subscription should resolve link
    /// events to their linked events.
    pub fn resolve_link_tos(mut self, value: bool) -> Self {
        self.setts.resolve_link_tos = value;
        self
    }

    /// Where the subscription should start from (event number).
    pub fn start_from(mut self, position: StreamPosition<Position>) -> Self {
        self.setts.start_from = position;
        self
    }

    /// Whether or not in depth latency statistics should be tracked on this
    /// subscription.
    pub fn extra_statistics(mut self, value: bool) -> Self {
        self.setts.extra_statistics = value;
        self
    }

    /// The amount of time after which a message should be considered to be
    /// timeout and retried.
    pub fn message_timeout(mut self, value: Duration) -> Self {
        self.setts.message_timeout = value;
        self
    }

    /// The maximum number of retries (due to timeout) before a message get
    /// considered to be parked.
    pub fn max_retry_count(mut self, value: i32) -> Self {
        self.setts.max_retry_count = value;
        self
    }

    /// The size of the buffer listening to live messages as they happen.
    pub fn live_buffer_size(mut self, value: i32) -> Self {
        self.setts.live_buffer_size = value;
        self
    }

    /// The number of events read at a time when paging in history.
    pub fn read_batch_size(mut self, value: i32) -> Self {
        self.setts.read_batch_size = value;
        self
    }

    /// The number of events to cache when paging through history.
    pub fn history_buffer_size(mut self, value: i32) -> Self {
        self.setts.history_buffer_size = value;
        self
    }

    /// The amount of time to try checkpoint after.
    pub fn checkpoint_after(mut self, value: Duration) -> Self {
        self.setts.checkpoint_after = value;
        self
    }

    /// The minimum number of messages to checkpoint.
    pub fn checkpoint_lower_bound(mut self, value: i32) -> Self {
        self.setts.checkpoint_lower_bound = value;
        self
    }

    /// The minimum number of messages to checkpoint.
    pub fn checkpoint_upper_bound(mut self, value: i32) -> Self {
        self.setts.checkpoint_upper_bound = value;
        self
    }

    /// The maximum number of subscribers allowed.
    pub fn max_subscriber_count(mut self, value: i32) -> Self {
        self.setts.max_subscriber_count = value;
        self
    }

    /// The strategy to use for distributing events to client consumers.
    pub fn consumer_strategy_name(mut self, value: SystemConsumerStrategy) -> Self {
        self.setts.consumer_strategy_name = value;
        self
    }

    pub fn settings_mut(&mut self) -> &mut PersistentSubscriptionSettings<Position> {
        &mut self.setts
    }
}

#[derive(Clone, Default)]
pub struct DeletePersistentSubscriptionOptions {
    pub(crate) common_operation_options: CommonOperationOptions,
}

impl_options_trait!(DeletePersistentSubscriptionOptions);

#[derive(Clone)]
pub struct SubscribeToPersistentSubscriptionOptions {
    pub(crate) buffer_size: usize,
    pub(crate) common_operation_options: CommonOperationOptions,
}

impl Default for SubscribeToPersistentSubscriptionOptions {
    fn default() -> Self {
        Self {
            buffer_size: 10,
            common_operation_options: Default::default(),
        }
    }
}

impl_options_trait!(SubscribeToPersistentSubscriptionOptions);

impl SubscribeToPersistentSubscriptionOptions {
    /// The buffer size to use  for the persistent subscription.
    pub fn buffer_size(self, buffer_size: usize) -> Self {
        Self {
            buffer_size,
            ..self
        }
    }
}

#[derive(Clone, Default)]
pub struct ReplayParkedMessagesOptions {
    pub(crate) credentials: Option<Credentials>,
    pub(crate) stop_at: Option<Duration>,
}

impl ReplayParkedMessagesOptions {
    /// Performs the command with the given credentials.
    pub fn authenticated(self, creds: Credentials) -> Self {
        Self {
            credentials: Some(creds),
            ..self
        }
    }

    pub fn stop_at(self, value: Duration) -> Self {
        Self {
            stop_at: Some(value),
            ..self
        }
    }
}

#[derive(Clone, Default)]
pub struct ListPersistentSubscriptionsOptions {
    pub(crate) credentials: Option<Credentials>,
}

impl ListPersistentSubscriptionsOptions {
    pub fn authenticated(self, creds: Credentials) -> Self {
        Self {
            credentials: Some(creds),
        }
    }
}

#[derive(Clone, Default)]
pub struct GetPersistentSubscriptionInfoOptions {
    pub(crate) credentials: Option<Credentials>,
}

impl GetPersistentSubscriptionInfoOptions {
    pub fn authenticated(self, creds: Credentials) -> Self {
        Self {
            credentials: Some(creds),
        }
    }
}
