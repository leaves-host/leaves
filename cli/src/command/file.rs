use anyhow::Result;
use pretty_bytes::converter as bytesize;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::io::{self, Write};

#[derive(Clone, Debug, Deserialize)]
struct GetFile {
    id: String,
    size: u32,
}

pub fn run(mut args: impl Iterator<Item = String>) -> Result<()> {
    let id = match args.next() {
        Some(id) => id,
        None => {
            writeln!(
                io::stdout(),
                "🍂 You need to give the ID of the file to get"
            )?;

            return Ok(());
        }
    };

    let client = Client::new();
    let res = client
        .get(&format!("http://0.0.0.0:10000/files/{}", id))
        .send()?;
    let file = res.json::<GetFile>()?;

    let human_bytes = bytesize::convert(file.size as f64);
    writeln!(
        io::stdout(),
        "🍂 ID: {}\n🍂 Size: {} ({} bytes)",
        file.id,
        human_bytes,
        file.size
    )?;

    Ok(())
}
