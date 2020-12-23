use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    fs,
    io::Error as IoError,
    path::PathBuf,
};

#[derive(Debug)]
pub enum ConfigError {
    Deleting { source: IoError },
    DeletingConfig { source: IoError },
    DeserializingConfig { source: JsonError },
    Directories { source: DirectoriesError },
    ReadingConfig { source: IoError },
    SerializingConfig { source: JsonError },
    WritingConfig { source: IoError },
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("configuration is in an invalid state")
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Deleting { source } => Some(source),
            Self::DeletingConfig { source } => Some(source),
            Self::DeserializingConfig { source } => Some(source),
            Self::Directories { .. } => None,
            Self::ReadingConfig { source } => Some(source),
            Self::SerializingConfig { source } => Some(source),
            Self::WritingConfig { source } => Some(source),
        }
    }
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
            api_url: api_url.into().trim().to_owned(),
            email: email.into().trim().to_owned(),
            token: token.into().trim().to_owned(),
        }
    }

    pub fn delete() -> Result<(), ConfigError> {
        let path = Dirs::new()
            .map_err(|source| ConfigError::Directories { source })?
            .config();
        fs::remove_file(path).map_err(|source| ConfigError::DeletingConfig { source })?;

        Ok(())
    }

    pub fn load() -> Result<Self, ConfigError> {
        let auth = Dirs::new()
            .map_err(|source| ConfigError::Directories { source })?
            .config();
        let contents = fs::read(auth).map_err(|source| ConfigError::ReadingConfig { source })?;
        let config = serde_json::from_slice(&contents)
            .map_err(|source| ConfigError::DeserializingConfig { source })?;

        Ok(config)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let auth = Dirs::new()
            .map_err(|source| ConfigError::Directories { source })?
            .config();
        let json = serde_json::to_string_pretty(self)
            .map_err(|source| ConfigError::SerializingConfig { source })?;

        fs::write(auth, json).map_err(|source| ConfigError::WritingConfig { source })?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum DirectoriesError {
    CreatingDirectory { path: PathBuf, source: IoError },
    UserHasNoHome,
}

impl Display for DirectoriesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("directory is not proper")
    }
}

impl Error for DirectoriesError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CreatingDirectory { source, .. } => Some(source),
            Self::UserHasNoHome => None,
        }
    }
}

struct Dirs(ProjectDirs);

impl Dirs {
    /// # Panics
    ///
    /// Panics if the current user has no home.
    pub fn new() -> Result<Self, DirectoriesError> {
        let project =
            ProjectDirs::from("", "leaves", "leaves-cli").ok_or(DirectoriesError::UserHasNoHome)?;
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
            fs::create_dir(data).map_err(|source| DirectoriesError::CreatingDirectory {
                path: data.to_path_buf(),
                source,
            })?;
        }

        Ok(())
    }
}
