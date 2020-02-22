use snafu::Snafu;
use std::convert::TryFrom;

pub struct User {
    pub api_token: String,
    pub email: String,
    pub id: i64,
}

#[derive(Debug, Eq, PartialEq, Snafu)]
pub enum AuthParseError {
    ApiTokenInvalid,
    EmailMissing,
    NotBasic,
    ValueInvalid,
    ValueMissing,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Auth<'a> {
    pub api_token: &'a str,
    pub email: &'a str,
}

impl<'a> TryFrom<&'a str> for Auth<'a> {
    type Error = AuthParseError;

    fn try_from(auth: &'a str) -> Result<Self, AuthParseError> {
        if !auth.starts_with("Basic ") {
            return Err(AuthParseError::NotBasic);
        }

        let mut parts = auth.splitn(2, ' ');
        assert_eq!(parts.next(), Some("Basic"));
        let data = parts.next().ok_or(AuthParseError::ValueMissing)?;

        if data.matches("/token:").count() != 1 {
            return Err(AuthParseError::ValueInvalid);
        }

        let mut credentials = data.split("/token:");
        let email = credentials.next().ok_or(AuthParseError::EmailMissing)?;

        if email.is_empty() {
            return Err(AuthParseError::EmailMissing);
        }

        let api_token = credentials.next().ok_or(AuthParseError::ApiTokenInvalid)?;

        if api_token.len() != 50 {
            return Err(AuthParseError::ApiTokenInvalid);
        }

        Ok(Auth { api_token, email })
    }
}

#[cfg(test)]
mod tests {
    use super::{Auth, AuthParseError};
    use std::{convert::TryFrom, error::Error};

    #[test]
    fn test_not_basic() {
        assert_eq!(
            AuthParseError::NotBasic,
            Auth::try_from("Bearer foo/token:baz").unwrap_err(),
        );
    }

    #[test]
    fn test_email_missing() {
        assert_eq!(
            AuthParseError::EmailMissing,
            Auth::try_from(format!("Basic /token:{}", "a".repeat(50)).as_ref()).unwrap_err(),
        );
    }

    #[test]
    fn test_valid() -> Result<(), Box<dyn Error>> {
        Auth::try_from(format!("Basic foo/token:{}", "a".repeat(50)).as_ref())?;

        Ok(())
    }
}
