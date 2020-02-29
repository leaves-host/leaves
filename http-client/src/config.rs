use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Config {
    pub api_token: Option<String>,
    pub api_url: String,
    pub email: Option<String>,
}

impl Config {
    pub fn new(
        api_token: impl Into<Option<String>>,
        api_url: impl Into<String>,
        email: impl Into<Option<String>>,
    ) -> Self {
        Self::_new(api_token.into(), api_url.into(), email.into())
    }

    const fn _new(api_token: Option<String>, api_url: String, email: Option<String>) -> Self {
        Self {
            api_token,
            api_url,
            email,
        }
    }

    pub fn auth(&self) -> Option<String> {
        if let (Some(api_token), Some(email)) = (self.api_token.as_ref(), self.email.as_ref()) {
            Some(format!("Basic {}/token:{}", email, api_token))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use std::fmt::Debug;

    static_assertions::assert_impl_all!(Config: Clone, Debug, Send, Sync);
}
