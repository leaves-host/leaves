use models::v1::FileInfo;
use pretty_bytes::converter as bytesize;
use reqwest::{blocking::Client, Error as ReqwestError};
use snafu::{ResultExt, Snafu};
use std::io::{self, Error as IoError, Write};

#[derive(Debug, Snafu)]
pub enum FileError {
    ParsingResponse { source: ReqwestError },
    SendingRequest { source: ReqwestError },
    WritingToStdout { source: IoError },
}

pub fn run(mut args: impl Iterator<Item = String>) -> Result<(), FileError> {
    let id = match args.next() {
        Some(id) => id,
        None => {
            writeln!(
                io::stdout(),
                "🍂 You need to give the ID of the file to get"
            )
            .context(WritingToStdout)?;

            return Ok(());
        }
    };

    let client = Client::new();
    let res = client
        .get(&format!("http://0.0.0.0:10000/files/{}", id))
        .send()
        .context(SendingRequest)?;
    let file = res.json::<FileInfo>().context(ParsingResponse)?;

    let human_bytes = bytesize::convert(file.size as f64);
    writeln!(
        io::stdout(),
        "🍂 ID: {}\n🍂 Size: {} ({} bytes)",
        file.id,
        human_bytes,
        file.size
    )
    .context(WritingToStdout)?;

    Ok(())
}
