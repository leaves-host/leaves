use crate::error::{Envy, Result};
use serde::Deserialize;
use snafu::ResultExt;

mod defaults {
    pub const fn open_registration() -> bool {
        true
    }

    pub const fn port() -> u16 {
        80
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "database_url")]
    pub db_url: String,
    #[serde(default = "defaults::open_registration")]
    pub open_registration: bool,
    pub public_url: String,
    #[serde(default = "defaults::port")]
    pub port: u16,
}

impl Config {
    pub fn new() -> Result<Self> {
        envy::from_env::<Self>().context(Envy)
    }
}
