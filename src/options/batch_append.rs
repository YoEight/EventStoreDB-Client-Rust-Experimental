use crate::Credentials;

#[derive(Clone, Default)]
pub struct BatchAppendOptions {
    pub(crate) credentials: Option<Credentials>,
}

impl BatchAppendOptions {
    /// Performs the command with the given credentials.
    pub fn authenticated(self, credentials: Credentials) -> Self {
        Self {
            credentials: Some(credentials),
        }
    }
}
