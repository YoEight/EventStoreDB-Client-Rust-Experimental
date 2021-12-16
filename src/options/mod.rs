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
    fn get_credentials(&self) -> Option<Credentials>;
    fn is_requires_leader_set(&self) -> bool;
    fn get_deadline(&self) -> Option<Duration>;
}

// TODO - Use procedural macros instead. It will need a separate crate
// though.
#[macro_export]
macro_rules! impl_options_trait {
    ($typ:ty) => {
        impl crate::options::Options for $typ {
            fn get_credentials(&self) -> Option<Credentials> {
                self.credentials.clone()
            }

            fn is_requires_leader_set(&self) -> bool {
                self.require_leader
            }

            fn get_deadline(&self) -> Option<Duration> {
                self.deadline
            }
        }

        impl $typ {
            /// Performs the command with the given credentials.
            pub fn authenticated(self, credentials: crate::types::Credentials) -> Self {
                Self {
                    credentials: Some(credentials),
                    ..self
                }
            }

            pub fn requires_leader(self, require_leader: bool) -> Self {
                Self {
                    require_leader,
                    ..self
                }
            }

            pub fn deadline(self, deadline: std::time::Duration) -> Self {
                Self {
                    deadline: Some(deadline),
                    ..self
                }
            }
        }
    };
}
