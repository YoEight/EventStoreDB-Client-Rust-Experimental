use std::time::Duration;

use crate::{impl_options_trait, types::Credentials};

#[derive(Debug, Clone, Default)]
pub struct CreateProjectionOptions {
    pub(crate) credentials: Option<Credentials>,
    pub(crate) track_emitted_streams: bool,
    pub(crate) emit: bool,
    pub(crate) require_leader: bool,
    pub(crate) deadline: Option<Duration>,
}

impl_options_trait!(CreateProjectionOptions);

impl CreateProjectionOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn track_emitted_streams(self, track_emitted_streams: bool) -> Self {
        Self {
            track_emitted_streams,
            ..self
        }
    }

    pub fn emit(self, emit: bool) -> Self {
        Self { emit, ..self }
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateProjectionOptions {
    pub(crate) credentials: Option<Credentials>,
    pub(crate) emit: Option<bool>,
    pub(crate) require_leader: bool,
    pub(crate) deadline: Option<Duration>,
}

impl_options_trait!(UpdateProjectionOptions);

impl UpdateProjectionOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn emit(self, emit: bool) -> Self {
        Self {
            emit: Some(emit),
            ..self
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DeleteProjectionOptions {
    pub(crate) credentials: Option<Credentials>,
    pub(crate) delete_emitted_streams: bool,
    pub(crate) delete_state_stream: bool,
    pub(crate) delete_checkpoint_stream: bool,
    pub(crate) require_leader: bool,
    pub(crate) deadline: Option<Duration>,
}

impl_options_trait!(DeleteProjectionOptions);

impl DeleteProjectionOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn delete_emitted_streams(self, delete_emitted_streams: bool) -> Self {
        Self {
            delete_emitted_streams,
            ..self
        }
    }

    pub fn delete_state_stream(self, delete_state_stream: bool) -> Self {
        Self {
            delete_state_stream,
            ..self
        }
    }

    pub fn delete_checkpoint_stream(self, delete_checkpoint_stream: bool) -> Self {
        Self {
            delete_checkpoint_stream,
            ..self
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GetStateProjectionOptions {
    pub(crate) credentials: Option<Credentials>,
    pub(crate) partition: String,
    pub(crate) require_leader: bool,
    pub(crate) deadline: Option<Duration>,
}

impl_options_trait!(GetStateProjectionOptions);

impl GetStateProjectionOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn partition(self, value: impl AsRef<str>) -> Self {
        Self {
            partition: value.as_ref().to_string(),
            ..self
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GetResultProjectionOptions {
    pub(crate) credentials: Option<Credentials>,
    pub(crate) partition: String,
    pub(crate) require_leader: bool,
    pub(crate) deadline: Option<Duration>,
}

impl_options_trait!(GetResultProjectionOptions);

impl GetResultProjectionOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn partition(self, value: impl AsRef<str>) -> Self {
        Self {
            partition: value.as_ref().to_string(),
            ..self
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GenericProjectionOptions {
    pub(crate) credentials: Option<Credentials>,
    pub(crate) require_leader: bool,
    pub(crate) deadline: Option<Duration>,
}

impl_options_trait!(GenericProjectionOptions);
