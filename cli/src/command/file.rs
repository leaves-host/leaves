use crate::config::Config;
use http_client::prelude::*;
use snafu::{ResultExt, Snafu};
use std::io::Error as IoError;

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
    let config = match Config::load() {
        Ok(config) => config,
        Err(_) => {
            println!("You need to login first with `leaves login`");

            return Ok(());
        }
    };

    let id = match args.next() {
        Some(id) => id,
        None => {
            println!("🍂 You need to give the ID of the file to get");

            return Ok(());
        }
    };

    let client =
        LeavesClient::new(LeavesConfig::new(None, config.api_url, None)).context(CreatingClient)?;
    let file = client
        .file_info(&id)
        .with_context(|| PerformingRequest { id })?;

    let human_bytes = bytesize::to_string(file.size, true);
    println!("🍂 ID: {}\n🍂 Size: {} ({} bytes)", file.id, human_bytes, file.size);

    Ok(())
}
