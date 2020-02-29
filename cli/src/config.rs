use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;
use snafu::{OptionExt, ResultExt, Snafu};
use std::{fs, io::Error as IoError, path::PathBuf};

#[derive(Debug, Snafu)]
pub enum ConfigError {
    Deleting { source: IoError },
    DeletingConfig { source: IoError },
    DeserializingConfig { source: JsonError },
    Directories { source: DirectoriesError },
    ReadingConfig { source: IoError },
    SerializingConfig { source: JsonError },
    WritingConfig { source: IoError },
}

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

    pub fn delete() -> Result<(), ConfigError> {
        let path = Dirs::new().context(Directories)?.config();
        fs::remove_file(path).context(DeletingConfig)?;

        Ok(())
    }

    pub fn load() -> Result<Self, ConfigError> {
        let auth = Dirs::new().context(Directories)?.config();
        let contents = fs::read(auth).context(ReadingConfig)?;
        let config = serde_json::from_slice(&contents).context(DeserializingConfig)?;

        Ok(config)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let auth = Dirs::new().context(Directories)?.config();
        let json = serde_json::to_string_pretty(self).context(SerializingConfig)?;

        fs::write(auth, json).context(WritingConfig)?;

        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum DirectoriesError {
    CreatingDirectory { path: PathBuf, source: IoError },
    UserHasNoHome,
}

struct Dirs(ProjectDirs);

impl Dirs {
    /// # Panics
    ///
    /// Panics if the current user has no home.
    pub fn new() -> Result<Self, DirectoriesError> {
        let project = ProjectDirs::from("", "leaves", "leaves-cli").context(UserHasNoHome)?;
        let dirs = Self(project);

        dirs.make_dirs()?;

        Ok(dirs)
    }

    pub fn config(&self) -> PathBuf {
        let mut path = self.0.data_local_dir().to_owned();
        path.push("config");

        path
    }

    pub fn make_dirs(&self) -> Result<(), DirectoriesError> {
        let data = self.0.data_local_dir();

        if !data.exists() {
            fs::create_dir(data).context(CreatingDirectory { path: data })?;
        }

        Ok(())
    }
}
