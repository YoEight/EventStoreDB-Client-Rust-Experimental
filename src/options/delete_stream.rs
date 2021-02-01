use crate::{Credentials, ExpectedRevision};

#[derive(Clone)]
/// Options of the delete stream command.
pub struct DeleteStreamOptions {
    pub(crate) version: ExpectedRevision,
    pub(crate) credentials: Option<Credentials>,
    pub(crate) hard_delete: bool,
}

impl Default for DeleteStreamOptions {
    fn default() -> Self {
        Self {
            version: ExpectedRevision::Any,
            credentials: None,
            hard_delete: false,
        }
    }
}

impl DeleteStreamOptions {
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

    /// Makes use of Truncate before. When a stream is deleted, its Truncate
    /// before is set to the streams current last event number. When a soft
    /// deleted stream is read, the read will return a StreamNotFound. After
    /// deleting the stream, you are able to write to it again, continuing from
    /// where it left off.
    ///
    /// That is the default behavior.
    pub fn soft_delete(self) -> Self {
        Self {
            hard_delete: false,
            ..self
        }
    }

    /// A hard delete writes a tombstone event to the stream, permanently
    /// deleting it. The stream cannot be recreated or written to again.
    /// Tombstone events are written with the event type '$streamDeleted'. When
    /// a hard deleted stream is read, the read will return a StreamDeleted.
    pub fn hard_delete(self) -> Self {
        Self {
            hard_delete: true,
            ..self
        }
    }
}
