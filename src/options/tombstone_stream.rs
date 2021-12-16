use std::time::Duration;

use crate::{impl_options_trait, Credentials, ExpectedRevision};

#[derive(Clone)]
/// Options of the tombstone stream command.
pub struct TombstoneStreamOptions {
    pub(crate) version: ExpectedRevision,
    pub(crate) credentials: Option<Credentials>,
    pub(crate) require_leader: bool,
    pub(crate) deadline: Option<Duration>,
}

impl Default for TombstoneStreamOptions {
    fn default() -> Self {
        Self {
            version: ExpectedRevision::Any,
            credentials: None,
            require_leader: false,
            deadline: None,
        }
    }
}

impl_options_trait!(TombstoneStreamOptions);

impl TombstoneStreamOptions {
    /// Asks the server to check that the stream receiving the event is at
    /// the given expected version. Default: `ExpectedVersion::Any`.
    pub fn expected_revision(self, version: ExpectedRevision) -> Self {
        Self { version, ..self }
    }
}
