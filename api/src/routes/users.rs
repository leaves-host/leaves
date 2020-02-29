use crate::prelude::*;
use log::warn;
use models::v1::{ApiToken, Signup, User as UserModel};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize)]
struct PostBody {
    email: String,
}

pub async fn get(req: Request) -> Response {
    match req.param::<String>("id").as_ref().map(AsRef::as_ref) {
        Ok("@me") => {},
        Ok(_) => return Response::new(403),
        Err(_) => return Response::new(400),
    };
    let user = req.local::<User>().expect("user must be present");

    let conn = req.state().db.get().expect("couldn't get connection");
    let query = conn.query_row_and_then(
        "select id, email from users where id = ?1",
        &[user.id as i64],
        serde_rusqlite::from_row::<UserModel>,
    );

    match query {
        Ok(user) => utils::response(200, &user),
        Err(why) => {
            warn!("Failed to get user {}: {:?}", user.id, why);

            utils::response(
                500,
                &json!({
                    "message": "Encountered an error while getting the user",
                }),
            )
        }
    }
}

pub async fn get_api_tokens(req: Request) -> Response {
    let user = req.local::<User>().expect("user must be present");

    let conn = req.state().db.get().unwrap();
    let mut statement = match conn.prepare("select id, contents, user_id from api_tokens where user_id = ?1") {
        Ok(statement) => statement,
        Err(why) => {
            warn!("Failed to prepare statement: {:?}", why);

            return utils::response(500, &json!({
                "message": "Failed to perform database statement",
            }));
        },
    };

    let rows = match statement.query_and_then(params![user.id], serde_rusqlite::from_row::<ApiToken>) {
        Ok(rows) => rows,
        Err(why) => {
            warn!("Failed to get API tokens for user {}: {:?}", user.id, why);

            return utils::response(
                500,
                &json!({
                    "message": "Error getting API tokens, please try again",
                }),
            );
        },
    };

    let tokens = rows.filter_map(|r| r.ok()).collect::<Vec<ApiToken>>();

    utils::response(200, &tokens)
}

pub async fn post(mut req: Request) -> Response {
    let PostBody { email } = match req.body_json().await {
        Ok(body) => body,
        Err(_) => {
            return utils::response(
                400,
                &json!({
                    "message": "Body must include an email",
                }),
            )
        }
    };
    let email = email.trim();

    let conn = req.state().db.get().unwrap();
    let query = conn.execute("insert into users (email) values (?1)", params![email]);

    let id = match query {
        Ok(_) => conn.last_insert_rowid(),
        Err(why) => {
            warn!("Failed to create user {}: {:?}", email, why);

            return utils::response(
                409,
                &json!({
                    "message": "A user with that email address already exists",
                }),
            );
        }
    };

    let token_content = utils::random_string(50);
    let conn = req.state().db.get().unwrap();
    let query = conn.execute(
        "insert into api_tokens (contents, user_id) values (?1, ?2)",
        params![token_content, id],
    );

    if let Err(why) = query {
        warn!("Failed to create API token {}: {:?}", token_content, why);

        return utils::response(
            500,
            &json!({
                "message": "Failed to create API token for user",
            }),
        );
    }

    utils::response(
        201,
        &Signup {
            email: email.to_owned(),
            id: id as u64,
            token: token_content,
        },
    )
}
