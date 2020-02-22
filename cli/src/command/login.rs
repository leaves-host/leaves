use crate::config::Config;
use anyhow::Result;
use reqwest::{blocking::Client, StatusCode};
use std::io::{self, Write};

pub fn run(mut args: impl Iterator<Item = String>) -> Result<()> {
    let (api_url, email, token) = match (args.next(), args.next(), args.next()) {
        (Some(api_url), Some(email), Some(token)) => (api_url, email, token),
        _ => {
            write!(io::stdout(), "Where is your leaves 🍂 instance?\n❯ ")?;
            io::stdout().flush()?;
            let mut api_url = String::new();
            io::stdin().read_line(&mut api_url)?;
            write!(io::stdout(), "What is your email address?\n❯ ")?;
            io::stdout().flush()?;

            let mut email = String::new();

            loop {
                io::stdin().read_line(&mut email)?;

                if !email.contains('@') || !email.contains('.') {
                    write!(io::stdout(), "It looks like *{}* is invalid", email)?;

                    email.clear();

                    continue;
                }

                break;
            }

            write!(io::stdout(), "What is your token?\n❯ ")?;
            io::stdout().flush()?;
            let mut token = String::new();
            io::stdin().read_line(&mut token)?;

            (api_url, email, token)
        }
    };
    let auth = format!("Basic {}/token:{}", email.trim(), token.trim());

    let client = Client::new();
    let res = client
        .get("http://0.0.0.0:10000/users/@me")
        .header("Authorization", &auth)
        .send()?;

    match res.status() {
        StatusCode::OK => {
            Config::new(api_url, email, token).save()?;

            writeln!(io::stdout(), "🍂 Signed in")?;
        }
        StatusCode::BAD_REQUEST | StatusCode::UNAUTHORIZED => {
            writeln!(io::stdout(), "🍂 Login credentials invalid")?;
        }
        other => {
            writeln!(io::stdout(), "🍂 Unknown response: {}", other)?;
        }
    }

    Ok(())
}
