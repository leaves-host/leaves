use crate::config::{Config, ConfigError};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug)]
pub enum LogoutError {
    DeletingConfig { source: ConfigError },
}

impl Display for LogoutError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("loging out failed")
    }
}

impl Error for LogoutError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DeletingConfig { source } => Some(source),
        }
    }
}

pub fn run() -> Result<(), LogoutError> {
    Config::delete().map_err(|source| LogoutError::DeletingConfig { source })?;

    println!("🍂 Logged out");

    Ok(())
}
