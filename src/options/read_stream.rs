use std::time::Duration;

use crate::{impl_options_trait, Credentials, ReadDirection, StreamPosition};

#[derive(Clone)]
pub struct ReadStreamOptions {
    pub(crate) credentials: Option<Credentials>,
    pub(crate) direction: ReadDirection,
    pub(crate) position: StreamPosition<u64>,
    pub(crate) resolve_link_tos: bool,
    pub(crate) require_leader: bool,
    pub(crate) deadline: Option<Duration>,
}

impl Default for ReadStreamOptions {
    fn default() -> Self {
        Self {
            credentials: None,
            direction: ReadDirection::Forward,
            position: StreamPosition::Start,
            resolve_link_tos: false,
            require_leader: false,
            deadline: None,
        }
    }
}

impl_options_trait!(ReadStreamOptions);

impl ReadStreamOptions {
    /// Asks the command to read forward (toward the end of the stream).
    /// That's the default behavior.
    pub fn forwards(self) -> Self {
        Self {
            direction: ReadDirection::Forward,
            ..self
        }
    }

    /// Asks the command to read backward (toward the begining of the stream).
    pub fn backwards(self) -> Self {
        Self {
            direction: ReadDirection::Backward,
            ..self
        }
    }

    /// Starts the read at the given event number. Default `StreamPosition::Start`
    pub fn position(self, position: StreamPosition<u64>) -> Self {
        match position {
            StreamPosition::Start => Self {
                position,
                direction: ReadDirection::Forward,
                ..self
            },

            StreamPosition::End => Self {
                position,
                direction: ReadDirection::Backward,
                ..self
            },

            StreamPosition::Position(_) => Self { position, ..self },
        }
    }

    /// When using projections, you can have links placed into another stream.
    /// If you set `true`, the server will resolve those links and will return
    /// the event that the link points to. Default: [NoResolution](../types/enum.LinkTos.html).
    pub fn resolve_link_tos(self) -> Self {
        Self {
            resolve_link_tos: true,
            ..self
        }
    }
}
