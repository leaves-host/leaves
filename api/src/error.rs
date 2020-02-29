use envy::Error as EnvyError;
use r2d2::Error as R2d2Error;
use snafu::Snafu;
use std::{io::Error as IoError, result::Result as StdResult};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("environment variables invalid"))]
    Envy { source: EnvyError },
    #[snafu(display("couldn't open database"))]
    R2d2Initialization { source: R2d2Error },
    #[snafu(display("couldn't bind to port when starting server"))]
    ServerInitialization { port: u16, source: IoError },
}
