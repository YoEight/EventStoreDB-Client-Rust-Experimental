use crate::{Credentials, ExpectedRevision};

#[derive(Clone)]
/// Options of the tombstone stream command.
pub struct TombstoneStreamOptions {
    pub(crate) version: ExpectedRevision,
    pub(crate) credentials: Option<Credentials>,
}

impl Default for TombstoneStreamOptions {
    fn default() -> Self {
        Self {
            version: ExpectedRevision::Any,
            credentials: None,
        }
    }
}

impl TombstoneStreamOptions {
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
        Self { version, ..self }
    }
}
