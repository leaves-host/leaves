use crate::config::{Config, ConfigError};
use http_client::prelude::*;
use snafu::{ResultExt, Snafu};
use std::io::{self, Error as IoError, Write};

#[derive(Debug, Snafu)]
pub enum LoginError {
    CreatingClient { source: LeavesClientError },
    FlushingStdout { source: IoError },
    PerformingRequest { source: LeavesClientError },
    ReadingStdin { source: IoError },
    SavingConfig { source: ConfigError },
}

pub fn run(mut args: impl Iterator<Item = String>) -> Result<(), LoginError> {
    let (api_url, email, token) = match (args.next(), args.next(), args.next()) {
        (Some(api_url), Some(email), Some(token)) => (api_url, email, token),
        _ => {
            println!("Where is your leaves 🍂 instance?\n❯ ");
            io::stdout().flush().context(FlushingStdout)?;
            let mut api_url = String::new();
            io::stdin().read_line(&mut api_url).context(ReadingStdin)?;
            println!("What is your email address?\n❯ ");
            io::stdout().flush().context(FlushingStdout)?;

            let mut email = String::new();

            loop {
                io::stdin().read_line(&mut email).context(ReadingStdin)?;

                if !email.contains('@') || !email.contains('.') {
                    println!("It looks like *{}* is invalid", email);

                    email.clear();

                    continue;
                }

                break;
            }

            println!("What is your token?\n❯ ");
            io::stdout().flush().context(FlushingStdout)?;
            let mut token = String::new();
            io::stdin().read_line(&mut token).context(ReadingStdin)?;

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
