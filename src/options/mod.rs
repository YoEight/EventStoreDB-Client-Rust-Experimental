use std::time::Duration;

use crate::Credentials;

pub mod append_to_stream;
pub mod batch_append;
pub mod delete_stream;
pub mod persistent_subscription;
pub mod projections;
pub mod read_all;
pub mod read_stream;
pub mod retry;
pub mod subscribe_to_all;
pub mod subscribe_to_stream;
pub mod tombstone_stream;

pub(crate) trait Options {
    fn common_operation_options(&self) -> &CommonOperationOptions;
}

#[derive(Clone)]
pub(crate) struct CommonOperationOptions {
    pub(crate) credentials: Option<Credentials>,
    pub(crate) requires_leader: bool,
    pub(crate) deadline: Option<Duration>,
}

impl Default for CommonOperationOptions {
    fn default() -> Self {
        Self {
            credentials: None,
            requires_leader: true,
            deadline: None,
        }
    }
}

// TODO - Use procedural macros instead. It will need a separate crate
// though.
#[macro_export]
macro_rules! impl_options_trait {
    ($typ:ty) => {
        impl crate::options::Options for $typ {
            fn common_operation_options(&self) -> &crate::options::CommonOperationOptions {
                &self.common_operation_options
            }
        }

        impl $typ {
            /// Performs the command with the given credentials.
            pub fn authenticated(mut self, credentials: crate::types::Credentials) -> Self {
                self.common_operation_options.credentials = Some(credentials);
                self
            }

            pub fn requires_leader(mut self, requires_leader: bool) -> Self {
                self.common_operation_options.requires_leader = requires_leader;
                self
            }

            pub fn deadline(mut self, deadline: std::time::Duration) -> Self {
                self.common_operation_options.deadline = Some(deadline);
                self
            }
        }
    };
}
