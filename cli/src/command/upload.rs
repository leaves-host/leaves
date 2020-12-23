use crate::config::Config;
use http_client::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    fs,
    io::{self, Error as IoError, Read},
};

#[derive(Debug)]
pub enum UploadError {
    CreatingClient { source: LeavesClientError },
    PerformingRequest { source: LeavesClientError },
    ReadingFile { source: IoError },
    ReadingStdin { source: IoError },
}

impl Display for UploadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("uploading a file failed")
    }
}

impl Error for UploadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CreatingClient { source } => Some(source),
            Self::PerformingRequest { source } => Some(source),
            Self::ReadingFile { source } => Some(source),
            Self::ReadingStdin { source } => Some(source),
        }
    }
}

pub fn run(mut args: impl Iterator<Item = String>) -> Result<(), UploadError> {
    let config = match Config::load() {
        Ok(config) => config,
        Err(_) => {
            println!("You need to login first with `leaves login`");

            return Ok(());
        }
    };

    let bytes = if let Some(filepath) = args.next() {
        fs::read(filepath).map_err(|source| UploadError::ReadingFile { source })?
    } else {
        let mut bytes = Vec::new();
        io::stdin()
            .read_to_end(&mut bytes)
            .map_err(|source| UploadError::ReadingStdin { source })?;

        bytes
    };

    let client = LeavesClient::new(LeavesConfig::new(
        Some(config.token),
        config.api_url,
        Some(config.email),
    ))
    .map_err(|source| UploadError::CreatingClient { source })?;
    let file = client
        .upload(bytes)
        .map_err(|source| UploadError::PerformingRequest { source })?;

    println!("{}", file.url);

    Ok(())
}
