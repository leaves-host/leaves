use crate::config::Config;
use http_client::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug)]
pub enum FileError {
    CreatingClient {
        source: LeavesClientError,
    },
    PerformingRequest {
        id: String,
        source: LeavesClientError,
    },
}

impl Display for FileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("working with the file failed")
    }
}

impl Error for FileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CreatingClient { source } => Some(source),
            Self::PerformingRequest { source, .. } => Some(source),
        }
    }
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

    let client = LeavesClient::new(LeavesConfig::new(None, config.api_url, None))
        .map_err(|source| FileError::CreatingClient { source })?;
    let file = client
        .file_info(&id)
        .map_err(|source| FileError::PerformingRequest { id, source })?;

    let human_bytes = bytesize::to_string(file.size, true);
    println!(
        "🍂 ID: {}\n🍂 Size: {} ({} bytes)",
        file.id, human_bytes, file.size
    );

    Ok(())
}
