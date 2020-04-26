use super::super::auth::Auth;
use crate::prelude::*;
use models::v1::User as UserModel;
use serde_json::json;
use std::{convert::TryFrom, future::Future, pin::Pin};
use tide::{Middleware, Next};

pub struct TokenValid;

impl Middleware<State> for TokenValid {
    fn handle<'a>(
        &'a self,
        mut req: TideRequest<State>,
        next: Next<'a, State>,
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'a>> {
        Box::pin(async move {
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

            let conn = req.state().db.get().unwrap();
            let query = conn.query_row_and_then(
                "select users.email, users.id from users join api_tokens on \
                api_tokens.user_id = users.id where users.email = ?1 and \
                api_tokens.contents = ?2 limit 1",
                &[auth.email, auth.api_token],
                serde_rusqlite::from_row::<UserModel>,
            );
            let user_row = match query {
                Ok(user) => user,
                Err(why) => {
                    log::warn!("Error: {:?}", why);

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
        })
    }
}
