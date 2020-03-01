use crate::config::{Config, ConfigError};
use http_client::prelude::*;
use snafu::{ResultExt, Snafu};
use std::io::{self, Error as IoError, Write};

#[derive(Debug, Snafu)]
pub enum SignupError {
    CreatingClient { source: LeavesClientError },
    PerformingRequest { source: LeavesClientError },
    PromptingUser { source: IoError },
    SavingConfig { source: ConfigError },
    WritingToStdout { source: IoError },
}

pub fn run() -> Result<(), SignupError> {
    let api_url =
        essentials::prompt("Where is your leaves 🍂 instance?\n❯ ").context(PromptingUser)?;
    let email = loop {
        let email = essentials::prompt("What is your email address?\n❯ ").context(PromptingUser)?;

        if !email.contains('@') || !email.contains('.') {
            writeln!(io::stdout(), "It looks like *{}* is invalid", email)
                .context(WritingToStdout)?;

            continue;
        }

        break email;
    };

    let client = LeavesClient::new(LeavesConfig::new(None, &api_url, None))
        .context(CreatingClient)?;

    match client.signup(email.trim()) {
        Ok(signup) => {
            Config::new(api_url, signup.email, signup.token)
                .save()
                .context(SavingConfig)?;

            writeln!(io::stdout(), "🍂 You're logged in and can start uploading")
                .context(WritingToStdout)?;
        }
        Err(LeavesClientError::ResourceAlreadyExists) => {
            writeln!(
                io::stdout(),
                "🍂 A user is already registered with that email address"
            )
            .context(WritingToStdout)?;
        }
        Err(LeavesClientError::InternalServerError) => {
            writeln!(
                io::stdout(),
                "🍂 The server encountered an error while making your account"
            )
            .context(WritingToStdout)?;
            writeln!(io::stdout(), "Please try again later").context(WritingToStdout)?;
        }
        Err(other) => {
            writeln!(io::stdout(), "🍂 An unknown error occurred: {:?}", other)
                .context(WritingToStdout)?;
            writeln!(io::stdout(), "Please try again later").context(WritingToStdout)?;
        }
    }

    Ok(())
}
