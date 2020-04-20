use crate::config::{Config, ConfigError};
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum LogoutError {
    DeletingConfig { source: ConfigError },
}

pub fn run() -> Result<(), LogoutError> {
    Config::delete().context(DeletingConfig)?;

    println!("🍂 Logged out");

    Ok(())
}
