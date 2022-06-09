use crate::options::CommonOperationOptions;
use crate::{impl_options_trait, ExpectedRevision};

#[derive(Clone)]
/// Options of the delete stream command.
pub struct DeleteStreamOptions {
    pub(crate) version: ExpectedRevision,
    pub(crate) common_operation_options: CommonOperationOptions,
}

impl Default for DeleteStreamOptions {
    fn default() -> Self {
        Self {
            version: ExpectedRevision::Any,
            common_operation_options: Default::default(),
        }
    }
}

impl_options_trait!(DeleteStreamOptions);

impl DeleteStreamOptions {
    /// Asks the server to check that the stream receiving the event is at
    /// the given expected version. Default: `ExpectedVersion::Any`.
    pub fn expected_revision(self, version: ExpectedRevision) -> Self {
        Self { version, ..self }
    }
}
