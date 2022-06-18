use crate::ExpectedRevision;
use eventstore_macros::options;

options! {
    #[derive(Clone)]
    /// Options of the delete stream command.
    pub struct DeleteStreamOptions {
        pub(crate) version: ExpectedRevision,
    }
}

impl Default for DeleteStreamOptions {
    fn default() -> Self {
        Self {
            version: ExpectedRevision::Any,
            common_operation_options: Default::default(),
        }
    }
}

impl DeleteStreamOptions {
    /// Asks the server to check that the stream receiving the event is at
    /// the given expected version. Default: `ExpectedVersion::Any`.
    pub fn expected_revision(self, version: ExpectedRevision) -> Self {
        Self { version, ..self }
    }
}
