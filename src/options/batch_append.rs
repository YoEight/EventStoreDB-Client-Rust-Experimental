use std::time::Duration;

use crate::{impl_options_trait, Credentials};

#[derive(Clone, Default)]
pub struct BatchAppendOptions {
    pub(crate) credentials: Option<Credentials>,
    pub(crate) require_leader: bool,
    pub(crate) deadline: Option<Duration>,
}

impl_options_trait!(BatchAppendOptions);
