use crate::error::{Envy, Result};
use serde::Deserialize;
use snafu::ResultExt;
use std::{fs, path::PathBuf};

mod defaults {
    pub fn data_path() -> String {
        "/data".to_owned()
    }

    pub const fn open_registration() -> bool {
        true
    }

    pub const fn port() -> u16 {
        80
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "defaults::data_path", rename = "data_path")]
    pub data_path: String,
    #[serde(default = "defaults::open_registration")]
    pub open_registration: bool,
    pub public_url: String,
    #[serde(default = "defaults::port")]
    pub port: u16,
}

impl Config {
    pub fn new() -> Result<Self> {
        let this = envy::from_env::<Self>().context(Envy)?;

        let mut files = PathBuf::from(&this.data_path);
        files.push("files");
        fs::create_dir_all(files).unwrap();

        Ok(this)
    }
}
