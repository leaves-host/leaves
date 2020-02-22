use crate::config::Config;
use anyhow::Result;
use reqwest::{blocking::Client, StatusCode};
use serde::Deserialize;
use serde_json::json;
use std::io::{self, Write};

#[derive(Debug, Deserialize)]
struct SignupBody {
    pub token: String,
    pub email: String,
    pub id: u64,
}

pub fn run() -> Result<()> {
    let api_url = essentials::prompt("Where is your leaves 🍂 instance?\n❯ ")?;
    let api_url = api_url.trim();
    let email = loop {
        let email = essentials::prompt("What is your email address?\n❯ ")?;

        if !email.contains('@') || !email.contains('.') {
            writeln!(io::stdout(), "It looks like *{}* is invalid", email)?;

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
        .send()?;

    match resp.status() {
        StatusCode::CREATED => {
            let body = resp.json::<SignupBody>()?;
            Config::new(api_url, body.email, body.token).save()?;

            writeln!(io::stdout(), "🍂 You're logged in and can start uploading")?;
        }
        StatusCode::CONFLICT => {
            writeln!(
                io::stdout(),
                "🍂 A user is already registered with that email address"
            )?;
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            writeln!(
                io::stdout(),
                "🍂 The server encountered an error while making your account"
            )?;
            writeln!(io::stdout(), "Please try again later")?;
        }
        other => {
            writeln!(io::stdout(), "🍂 An unknown error occurred: {}", other)?;
            writeln!(io::stdout(), "Please try again later")?;
        }
    }

    Ok(())
}
