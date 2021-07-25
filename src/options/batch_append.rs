use crate::Credentials;

#[derive(Clone)]
pub struct BatchAppendOptions {
    pub(crate) credentials: Option<Credentials>,
}

impl Default for BatchAppendOptions {
    fn default() -> Self {
        Self { credentials: None }
    }
}

impl BatchAppendOptions {
    /// Performs the command with the given credentials.
    pub fn authenticated(self, credentials: Credentials) -> Self {
        Self {
            credentials: Some(credentials),
        }
    }
}
