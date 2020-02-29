use http_client::prelude::*;
use pretty_bytes::converter as bytesize;
use snafu::{ResultExt, Snafu};
use std::io::{self, Error as IoError, Write};

#[derive(Debug, Snafu)]
pub enum FileError {
    CreatingClient {
        source: LeavesClientError,
    },
    PerformingRequest {
        id: String,
        source: LeavesClientError,
    },
    WritingToStdout {
        source: IoError,
    },
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

    let client = LeavesClient::new(LeavesConfig::new(None, "http://0.0.0.0", None))
        .context(CreatingClient)?;
    let file = client
        .file_info(&id)
        .with_context(|| PerformingRequest { id })?;

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
