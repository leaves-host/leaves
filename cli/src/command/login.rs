use crate::{
    config::{Config, ConfigError},
    util,
};
use http_client::prelude::*;
use snafu::{ResultExt, Snafu};
use std::io::Error as IoError;

#[derive(Debug, Snafu)]
pub enum LoginError {
    CreatingClient { source: LeavesClientError },
    PerformingRequest { source: LeavesClientError },
    PromptingUser { source: IoError },
    SavingConfig { source: ConfigError },
}

pub fn run(mut args: impl Iterator<Item = String>) -> Result<(), LoginError> {
    let (api_url, email, token) = match (args.next(), args.next(), args.next()) {
        (Some(api_url), Some(email), Some(token)) => (api_url, email, token),
        _ => {
            let api_url =
                util::prompt("Where is your leaves 🍂 instance?\n❯ ").context(PromptingUser)?;

            let email = loop {
                let email =
                    util::prompt("What is your email address?\n❯ ").context(PromptingUser)?;

                if !email.contains('@') || !email.contains('.') {
                    println!("It looks like *{}* is invalid", email);

                    continue;
                }

                break email;
            };

            let token = util::prompt("What is your token?\n❯ ").context(PromptingUser)?;

            (api_url, email, token)
        }
    };
    let client = LeavesClient::new(LeavesConfig::new(
        Some(token.trim().to_owned()),
        &api_url,
        Some(email.trim().to_owned()),
    ))
    .context(CreatingClient)?;

    match client.me() {
        Ok(_) => {
            Config::new(api_url, email, token)
                .save()
                .context(SavingConfig)?;

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
