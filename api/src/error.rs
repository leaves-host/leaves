use envy::Error as EnvyError;
use r2d2::Error as R2d2Error;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    result::Result as StdResult,
};

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug)]
pub enum Error {
    Envy { source: EnvyError },
    R2d2Initialization { source: R2d2Error },
    ServerInitialization { port: u16, source: IoError },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Envy { .. } => f.write_str("environment variables invalid"),
            Self::R2d2Initialization { .. } => f.write_str("couldn't open database"),
            Self::ServerInitialization { .. } => {
                f.write_str("couldn't bind to port when starting server")
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Envy { source } => Some(source),
            Self::R2d2Initialization { source } => Some(source),
            Self::ServerInitialization { source, .. } => Some(source),
        }
    }
}
