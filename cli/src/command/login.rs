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
pub enum LoginError {
    CreatingClient { source: LeavesClientError },
    PromptingUser { source: IoError },
    SavingConfig { source: ConfigError },
}

impl Display for LoginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("logging in failed")
    }
}

impl Error for LoginError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CreatingClient { source } => Some(source),
            Self::PromptingUser { source } => Some(source),
            Self::SavingConfig { source } => Some(source),
        }
    }
}

pub fn run(mut args: impl Iterator<Item = String>) -> Result<(), LoginError> {
    let (api_url, email, token) = match (args.next(), args.next(), args.next()) {
        (Some(api_url), Some(email), Some(token)) => (api_url, email, token),
        _ => {
            let api_url = util::prompt("Where is your leaves 🍂 instance?\n❯ ")
                .map_err(|source| LoginError::PromptingUser { source })?;

            let email = loop {
                let email = util::prompt("What is your email address?\n❯ ")
                    .map_err(|source| LoginError::PromptingUser { source })?;

                if !email.contains('@') || !email.contains('.') {
                    println!("It looks like *{}* is invalid", email);

                    continue;
                }

                break email;
            };

            let token = util::prompt("What is your token?\n❯ ")
                .map_err(|source| LoginError::PromptingUser { source })?;

            (api_url, email, token)
        }
    };
    let client = LeavesClient::new(LeavesConfig::new(
        Some(token.trim().to_owned()),
        &api_url,
        Some(email.trim().to_owned()),
    ))
    .map_err(|source| LoginError::CreatingClient { source })?;

    match client.me() {
        Ok(_) => {
            Config::new(api_url, email, token)
                .save()
                .map_err(|source| LoginError::SavingConfig { source })?;

            println!("🍂 Signed in");
        }
        Err(LeavesClientError::Unauthorized) => {
            println!("🍂 Login credentials invalid");
        }
        Err(other) => {
            println!("🍂 Unknown response: {:?}", other);
        }
    }

    Ok(())
}
