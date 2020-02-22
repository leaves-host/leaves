use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::PathBuf,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub api_url: String,
    pub email: String,
    pub token: String,
}

impl Config {
    pub fn new(
        api_url: impl Into<String>,
        email: impl Into<String>,
        token: impl Into<String>,
    ) -> Self {
        Self {
            api_url: api_url.into(),
            email: email.into(),
            token: token.into(),
        }
    }

    pub fn auth(&self) -> String {
        format!("Basic {}/token:{}", self.email, self.token)
    }

    pub fn delete() -> Result<()> {
        let path = Dirs::new()?.config();
        fs::remove_file(path)?;

        Ok(())
    }

    pub fn load() -> Result<Self> {
        let auth = Dirs::new()?.config();
        let config = serde_json::from_reader(File::open(auth)?)?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let auth = Dirs::new()?.config();
        let json = serde_json::to_string_pretty(self)?;

        fs::write(auth, json)?;

        Ok(())
    }
}

pub struct Dirs(ProjectDirs);

impl Dirs {
    /// # Panics
    ///
    /// Panics if the current user has no home.
    pub fn new() -> Result<Self> {
        let project = ProjectDirs::from("", "leaves", "leaves-cli").expect("user has no home");
        let dirs = Self(project);

        dirs.make_dirs()?;

        Ok(dirs)
    }

    pub fn config(&self) -> PathBuf {
        let mut path = self.0.data_local_dir().to_owned();
        path.push("config");

        path
    }

    pub fn make_dirs(&self) -> Result<()> {
        let data = self.0.data_local_dir();

        if !data.exists() {
            fs::create_dir(data)?;
        }

        Ok(())
    }
}
