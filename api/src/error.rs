use envy::Error as EnvyError;
use snafu::Snafu;
use sqlx::Error as SqlxError;
use std::{io::Error as IoError, result::Result as StdResult};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Environment variables invalid"))]
    Envy { source: EnvyError },
    #[snafu(display("couldn't open database"))]
    SqlxInitialization { source: SqlxError },
    #[snafu(display("couldn't bind to port when starting server"))]
    ServerInitialization { port: u16, source: IoError },
}
