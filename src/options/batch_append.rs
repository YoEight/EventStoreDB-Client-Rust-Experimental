use crate::impl_options_trait;
use crate::options::CommonOperationOptions;

#[derive(Clone, Default)]
pub struct BatchAppendOptions {
    pub(crate) common_operation_options: CommonOperationOptions,
}

impl_options_trait!(BatchAppendOptions);
