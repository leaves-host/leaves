use super::super::auth::Auth;
use crate::prelude::*;
use futures::future::{BoxFuture, FutureExt};
use serde_json::json;
use std::convert::TryFrom;
use tide::{Middleware, Next};

pub struct TokenValid;

impl Middleware<State> for TokenValid {
    fn handle<'a>(
        &'a self,
        mut req: TideRequest<State>,
        next: Next<'a, State>,
    ) -> BoxFuture<'a, Response> {
        async move {
            let header = match req.header("authorization") {
                Some(auth) => auth,
                None => {
                    return Response::new(401).body_string("Authorization missing.".to_owned());
                }
            };

            let auth = match Auth::try_from(header) {
                Ok(auth) => auth,
                Err(_) => {
                    return Response::new(401).body_string("Authorization malformed.".to_owned());
                }
            };

            let query = sqlx::query!(
                "select users.email, users.id from users join api_tokens on \
                api_tokens.user_id = users.id where users.email = $1 and \
                api_tokens.contents = $2 limit 1",
                auth.email.to_owned(),
                auth.api_token.to_owned()
            );
            let mut pool = &req.state().db;
            let user_row = match query.fetch_one(&mut pool).await {
                Ok(user) => user,
                Err(_) => {
                    return utils::response(
                        401,
                        &json!({
                            "message": "Credentials invalid.",
                        }),
                    );
                }
            };

            let api_token = auth.api_token.to_owned();

            req = req.set_local(User {
                api_token,
                email: user_row.email,
                id: user_row.id,
            });

            next.run(req).await
        }
        .boxed()
    }
}
