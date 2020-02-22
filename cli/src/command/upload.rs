use crate::config::Config;
use anyhow::Result;
use copypasta::{ClipboardContext, ClipboardProvider};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{
    fs,
    io::{self, Read, Write},
};

#[derive(Clone, Debug, Deserialize)]
struct Upload {
    pub id: String,
    pub size: u64,
    pub url: String,
}

pub fn run(mut args: impl Iterator<Item = String>) -> Result<()> {
    let config = match Config::load() {
        Ok(config) => config,
        Err(_) => {
            writeln!(io::stdout(), "You need to login first with `leaves login`")?;

            return Ok(());
        }
    };

    let bytes = if let Some(filepath) = args.next() {
        writeln!(io::stdout(), "reading")?;
        let bytes = fs::read(filepath)?;
        writeln!(io::stdout(), "read")?;

        bytes
    } else {
        let mut bytes = Vec::new();
        io::stdin().read_to_end(&mut bytes)?;

        bytes
    };

    let client = Client::new();
    writeln!(io::stdout(), "sending")?;
    let res = client
        .post("http://0.0.0.0:10000/files")
        .body(bytes)
        .header("Authorization", config.auth())
        .send()?;
    writeln!(io::stdout(), "sent")?;
    let file = res.json::<Upload>()?;
    writeln!(io::stdout(), "file")?;

    writeln!(io::stdout(), "🍂 {}", file.url)?;

    let mut clipboard = ClipboardContext::new().unwrap();
    clipboard.set_contents(file.url).unwrap();

    Ok(())
}
