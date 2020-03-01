use crate::config::Config;
use copypasta::{ClipboardContext, ClipboardProvider};
use http_client::prelude::*;
use snafu::{ResultExt, Snafu};
use std::{
    fs,
    io::{self, Error as IoError, Read, Write},
};

#[derive(Debug, Snafu)]
pub enum UploadError {
    CreatingClient { source: LeavesClientError },
    PerformingRequest { source: LeavesClientError },
    ReadingFile { source: IoError },
    ReadingStdin { source: IoError },
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
        let bytes = fs::read(filepath).context(ReadingFile)?;

        bytes
    } else {
        let mut bytes = Vec::new();
        io::stdin().read_to_end(&mut bytes).context(ReadingStdin)?;

        bytes
    };

    let client = LeavesClient::new(LeavesConfig::new(
        Some(config.token),
        config.api_url,
        Some(config.email),
    ))
    .context(CreatingClient)?;
    let file = client.upload(bytes).context(PerformingRequest)?;

    writeln!(io::stdout(), "{}", file.url).context(WritingToStdout)?;

    let mut clipboard = ClipboardContext::new().unwrap();
    clipboard.set_contents(file.url).unwrap();

    Ok(())
}
