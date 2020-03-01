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
    WritingToStdout { source: IoError },
}

pub fn run(mut args: impl Iterator<Item = String>) -> Result<(), LoginError> {
    let (api_url, email, token) = match (args.next(), args.next(), args.next()) {
        (Some(api_url), Some(email), Some(token)) => (api_url, email, token),
        _ => {
            write!(io::stdout(), "Where is your leaves 🍂 instance?\n❯ ")
                .context(WritingToStdout)?;
            io::stdout().flush().context(FlushingStdout)?;
            let mut api_url = String::new();
            io::stdin().read_line(&mut api_url).context(ReadingStdin)?;
            write!(io::stdout(), "What is your email address?\n❯ ").context(WritingToStdout)?;
            io::stdout().flush().context(FlushingStdout)?;

            let mut email = String::new();

            loop {
                io::stdin().read_line(&mut email).context(ReadingStdin)?;

                if !email.contains('@') || !email.contains('.') {
                    write!(io::stdout(), "It looks like *{}* is invalid", email)
                        .context(WritingToStdout)?;

                    email.clear();

                    continue;
                }

                break;
            }

            write!(io::stdout(), "What is your token?\n❯ ").context(WritingToStdout)?;
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

            writeln!(io::stdout(), "🍂 Signed in").context(WritingToStdout)?;
        }
        Err(LeavesClientError::Unauthorized) => {
            writeln!(io::stdout(), "🍂 Login credentials invalid").context(WritingToStdout)?;
        }
        Err(other) => {
            writeln!(io::stdout(), "🍂 Unknown response: {:?}", other).context(WritingToStdout)?;
        }
    }

    Ok(())
}
