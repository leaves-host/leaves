use crate::config::{Config, ConfigError};
use snafu::{ResultExt, Snafu};
use std::io::{self, Error as IoError, Write};

#[derive(Debug, Snafu)]
pub enum LogoutError {
    DeletingConfig { source: ConfigError },
    WritingMessage { source: IoError },
}

pub fn run() -> Result<(), LogoutError> {
    Config::delete().context(DeletingConfig)?;

    writeln!(io::stdout(), "🍂 Logged out").context(WritingMessage)?;

    Ok(())
}
