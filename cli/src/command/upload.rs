use crate::config::Config;
use copypasta::{ClipboardContext, ClipboardProvider};
use models::v1::Upload;
use reqwest::{blocking::Client, Error as ReqwestError};
use snafu::{ResultExt, Snafu};
use std::{
    fs,
    io::{self, Error as IoError, Read, Write},
};

#[derive(Debug, Snafu)]
pub enum UploadError {
    ParsingResponse { source: ReqwestError },
    ReadingFile { source: IoError },
    ReadingStdin { source: IoError },
    SendingRequest { source: ReqwestError },
    WritingToStdout { source: IoError },
}

pub fn run(mut args: impl Iterator<Item = String>) -> Result<(), UploadError> {
    let config = match Config::load() {
        Ok(config) => config,
        Err(_) => {
            writeln!(io::stdout(), "You need to login first with `leaves login`")
                .context(WritingToStdout)?;

            return Ok(());
        }
    };

    let bytes = if let Some(filepath) = args.next() {
        writeln!(io::stdout(), "reading").context(WritingToStdout)?;
        let bytes = fs::read(filepath).context(ReadingFile)?;
        writeln!(io::stdout(), "read").context(WritingToStdout)?;

        bytes
    } else {
        let mut bytes = Vec::new();
        io::stdin().read_to_end(&mut bytes).context(ReadingStdin)?;

        bytes
    };

    let client = Client::new();
    writeln!(io::stdout(), "sending").context(WritingToStdout)?;
    let res = client
        .post("http://0.0.0.0:10000/files")
        .body(bytes)
        .header("Authorization", config.auth())
        .send()
        .context(SendingRequest)?;
    writeln!(io::stdout(), "sent").context(WritingToStdout)?;
    let file = res.json::<Upload>().context(ParsingResponse)?;
    writeln!(io::stdout(), "file").context(WritingToStdout)?;

    writeln!(io::stdout(), "🍂 {}", file.url).context(WritingToStdout)?;

    let mut clipboard = ClipboardContext::new().unwrap();
    clipboard.set_contents(file.url).unwrap();

    Ok(())
}
