use crate::config::Config;
use anyhow::Result;
use std::io::{self, Write};

pub fn run() -> Result<()> {
    Config::delete()?;

    writeln!(io::stdout(), "🍂 Logged out")?;

    Ok(())
}
