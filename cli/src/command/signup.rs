use crate::{
    config::{Config, ConfigError},
    util,
};
use http_client::prelude::*;
use snafu::{ResultExt, Snafu};
use std::io::Error as IoError;

#[derive(Debug, Snafu)]
pub enum SignupError {
    CreatingClient { source: LeavesClientError },
    PerformingRequest { source: LeavesClientError },
    PromptingUser { source: IoError },
    SavingConfig { source: ConfigError },
}

pub fn run() -> Result<(), SignupError> {
    let api_url = util::prompt("Where is your leaves 🍂 instance?\n❯ ").context(PromptingUser)?;
    let email = loop {
        let email = util::prompt("What is your email address?\n❯ ").context(PromptingUser)?;

        if !email.contains('@') || !email.contains('.') {
            println!("It looks like *{}* is invalid", email);

            continue;
        }

        break email;
    };

    let client =
        LeavesClient::new(LeavesConfig::new(None, &api_url, None)).context(CreatingClient)?;

    match client.signup(email.trim()) {
        Ok(signup) => {
            Config::new(api_url, signup.email, signup.token)
                .save()
                .context(SavingConfig)?;

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
