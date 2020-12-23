use crate::{
    config::{Config, ConfigError},
    util,
};
use http_client::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
};

#[derive(Debug)]
pub enum SignupError {
    CreatingClient { source: LeavesClientError },
    PerformingRequest { source: LeavesClientError },
    PromptingUser { source: IoError },
    SavingConfig { source: ConfigError },
}

impl Display for SignupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("signing up failed")
    }
}

impl Error for SignupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CreatingClient { source } => Some(source),
            Self::PerformingRequest { source } => Some(source),
            Self::PromptingUser { source } => Some(source),
            Self::SavingConfig { source } => Some(source),
        }
    }
}

pub fn run() -> Result<(), SignupError> {
    let api_url = util::prompt("Where is your leaves 🍂 instance?\n❯ ")
        .map_err(|source| SignupError::PromptingUser { source })?;
    let email = loop {
        let email = util::prompt("What is your email address?\n❯ ")
            .map_err(|source| SignupError::PromptingUser { source })?;

        if !email.contains('@') || !email.contains('.') {
            println!("It looks like *{}* is invalid", email);

            continue;
        }

        break email;
    };

    let client = LeavesClient::new(LeavesConfig::new(None, &api_url, None))
        .map_err(|source| SignupError::CreatingClient { source })?;

    match client.signup(email.trim()) {
        Ok(signup) => {
            Config::new(api_url, signup.email, signup.token)
                .save()
                .map_err(|source| SignupError::SavingConfig { source })?;

            println!("🍂 You're logged in and can start uploading");
        }
        Err(LeavesClientError::ResourceAlreadyExists) => {
            println!("🍂 A user is already registered with that email address");
        }
        Err(LeavesClientError::InternalServerError) => {
            println!("🍂 The server encountered an error while making your account");
        }
        Err(other) => {
            println!("🍂 An unknown error occurred: {:?}", other);
            println!("Please try again later");
        }
    }

    Ok(())
}
