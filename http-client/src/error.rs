use isahc::{http::Error as HttpError, Error as IsahcError};
use serde_json::Error as JsonError;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    result::Result as StdResult,
};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    BuildingHttpClient {
        source: IsahcError,
    },
    BuildingRequest {
        source: HttpError,
    },
    CopyingResponseBody {
        source: IoError,
    },
    DeserializingBody {
        contents: Vec<u8>,
        source: JsonError,
    },
    InternalServerError,
    NotFound,
    ResourceAlreadyExists,
    SendingRequest {
        source: IsahcError,
    },
    Unauthorized,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::BuildingHttpClient { .. } => f.write_str("failed to construct isahc http client"),
            Self::BuildingRequest { .. } => f.write_str("failed to construct the http request"),
            Self::CopyingResponseBody { .. } => f.write_str("failed to copy body out of response"),
            Self::DeserializingBody { .. } => {
                f.write_str("failed to deserialize the response body as json")
            }
            Self::InternalServerError => f.write_str("server encountered an error"),
            Self::NotFound => f.write_str("resource not found"),
            Self::ResourceAlreadyExists => {
                f.write_str("a resource with that information already exists")
            }
            Self::SendingRequest { .. } => {
                f.write_str("failed to send the request or receive the response")
            }
            Self::Unauthorized => f.write_str("not authorized to perform request"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::BuildingHttpClient { source } => Some(source),
            Self::BuildingRequest { source } => Some(source),
            Self::CopyingResponseBody { source, .. } => Some(source),
            Self::DeserializingBody { source, .. } => Some(source),
            Self::InternalServerError => None,
            Self::NotFound => None,
            Self::ResourceAlreadyExists => None,
            Self::SendingRequest { source } => Some(source),
            Self::Unauthorized => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use std::{
        error::Error as StdError,
        fmt::{Debug, Display},
    };

    static_assertions::assert_impl_all!(Error: Debug, Display, Send, StdError, Sync);
}
