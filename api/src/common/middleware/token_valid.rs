use super::super::auth::{Auth, User};
use async_trait::async_trait;
use crate::state::State;
use log::warn;
use models::v1::User as UserModel;
use snafu::Snafu;
use std::{convert::TryFrom, str::FromStr};
use tide::{
    http::headers::HeaderName, Error as TideError, Middleware, Next, Request, Response,
    Result as TideResult, StatusCode,
};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("authorization malformed"))]
    AuthorizationMalformed,
    #[snafu(display("authorization header missing"))]
    AuthorizationMissing,
    #[snafu(display("couldn't retrieve authorization header"))]
    CreatingAuthorizationHeader,
    #[snafu(display("the authorization is invalid"))]
    Unauthorized,
}

#[derive(Debug)]
pub struct TokenValid;

#[async_trait]
impl Middleware<State> for TokenValid {
    async fn handle(
        &self,
        mut req: Request<State>,
        next: Next<'_, State>,
    ) -> TideResult<Response> {
        let header_name = HeaderName::from_str("authorization")
            .map_err(|_| Error::CreatingAuthorizationHeader)?;

        let header_values = req.header(&header_name).ok_or_else(|| {
            TideError::new(StatusCode::Unauthorized, Error::AuthorizationMissing)
        })?;

        let header_value = header_values.last();

        let auth = Auth::try_from(header_value.as_str()).map_err(|_| {
            TideError::new(StatusCode::Unauthorized, Error::AuthorizationMalformed)
        })?;

        let conn = req.state().db.get().unwrap();
        let query = conn.query_row_and_then(
            "select users.email, users.id from users join api_tokens on \
            api_tokens.user_id = users.id where users.email = ?1 and \
            api_tokens.contents = ?2 limit 1",
            &[auth.email, auth.api_token],
            serde_rusqlite::from_row::<UserModel>,
        );
        let user_row = query.map_err(|why| {
            warn!("Error: {:?}", why);

            TideError::new(StatusCode::Unauthorized, Error::Unauthorized)
        })?;

        let api_token = auth.api_token.to_owned();

        req.set_ext(User {
            api_token,
            email: user_row.email,
            id: user_row.id,
        });

        Ok(next.run(req).await)
    }
}
