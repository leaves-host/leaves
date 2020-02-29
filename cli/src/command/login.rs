use crate::config::{Config, ConfigError};
use reqwest::{blocking::Client, Error as ReqwestError, StatusCode};
use snafu::{ResultExt, Snafu};
use std::io::{self, Error as IoError, Write};

#[derive(Debug, Snafu)]
pub enum LoginError {
    FlushingStdout { source: IoError },
    ReadingStdin { source: IoError },
    SavingConfig { source: ConfigError },
    SendingRequest { source: ReqwestError },
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
    let auth = format!("Basic {}/token:{}", email.trim(), token.trim());

    let client = Client::new();
    let res = client
        .get("http://0.0.0.0:10000/users/@me")
        .header("Authorization", &auth)
        .send()
        .context(SendingRequest)?;

    match res.status() {
        StatusCode::OK => {
            Config::new(api_url, email, token)
                .save()
                .context(SavingConfig)?;

            writeln!(io::stdout(), "🍂 Signed in").context(WritingToStdout)?;
        }
        StatusCode::BAD_REQUEST | StatusCode::UNAUTHORIZED => {
            writeln!(io::stdout(), "🍂 Login credentials invalid").context(WritingToStdout)?;
        }
        other => {
            writeln!(io::stdout(), "🍂 Unknown response: {}", other).context(WritingToStdout)?;
        }
    }

    Ok(())
}
