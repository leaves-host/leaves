use crate::config::{Config, ConfigError};
use models::v1::Signup;
use reqwest::{blocking::Client, Error as ReqwestError, StatusCode};
use serde_json::json;
use snafu::{ResultExt, Snafu};
use std::io::{self, Error as IoError, Write};

#[derive(Debug, Snafu)]
pub enum SignupError {
    ParsingResponse { source: ReqwestError },
    PromptingUser { source: IoError },
    SavingConfig { source: ConfigError },
    SendingRequest { source: ReqwestError },
    WritingToStdout { source: IoError },
}

pub fn run() -> Result<(), SignupError> {
    let api_url =
        essentials::prompt("Where is your leaves 🍂 instance?\n❯ ").context(PromptingUser)?;
    let api_url = api_url.trim();
    let email = loop {
        let email = essentials::prompt("What is your email address?\n❯ ").context(PromptingUser)?;

        if !email.contains('@') || !email.contains('.') {
            writeln!(io::stdout(), "It looks like *{}* is invalid", email)
                .context(WritingToStdout)?;

            continue;
        }

        break email;
    };
    let email = email.trim();

    let client = Client::new();
    let req_body = json!({
        "email": email,
    })
    .to_string();
    let resp = client
        .post("http://0.0.0.0:10000/users")
        .body(req_body)
        .send()
        .context(SendingRequest)?;

    match resp.status() {
        StatusCode::CREATED => {
            let body = resp.json::<Signup>().context(ParsingResponse)?;
            Config::new(api_url, body.email, body.token)
                .save()
                .context(SavingConfig)?;

            writeln!(io::stdout(), "🍂 You're logged in and can start uploading")
                .context(WritingToStdout)?;
        }
        StatusCode::CONFLICT => {
            writeln!(
                io::stdout(),
                "🍂 A user is already registered with that email address"
            )
            .context(WritingToStdout)?;
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            writeln!(
                io::stdout(),
                "🍂 The server encountered an error while making your account"
            )
            .context(WritingToStdout)?;
            writeln!(io::stdout(), "Please try again later").context(WritingToStdout)?;
        }
        other => {
            writeln!(io::stdout(), "🍂 An unknown error occurred: {}", other)
                .context(WritingToStdout)?;
            writeln!(io::stdout(), "Please try again later").context(WritingToStdout)?;
        }
    }

    Ok(())
}
