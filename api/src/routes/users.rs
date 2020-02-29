use crate::prelude::*;
use log::warn;
use models::v1::{ApiToken, Signup, User};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize)]
struct PostBody {
    email: String,
}

pub async fn get(req: Request) -> Response {
    let user_id = match req.param::<u64>("id") {
        Ok(user_id) => user_id,
        Err(_) => return Response::new(400),
    };

    let query = { sqlx::query_file_as!(User, "sql/select_user_by_id.sql", user_id as i64) };
    let mut pool = &req.state().db;

    match query.fetch_optional(&mut pool).await {
        Ok(Some(user)) => utils::response(200, &user),
        Ok(None) => utils::response(
            404,
            &json!({
                "message": "That user doesn't exist",
            }),
        ),
        Err(why) => {
            warn!("Failed to get user {}: {:?}", user_id, why);

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

    let query = { sqlx::query_file_as!(ApiToken, "sql/select_api_tokens_by_user.sql", user.id) };
    let mut pool = &req.state().db;

    match query.fetch_all(&mut pool).await {
        Ok(tokens) => utils::response(200, &tokens),
        Err(why) => {
            warn!("Failed to get API tokens for user {}: {:?}", user.id, why);

            utils::response(
                500,
                &json!({
                    "message": "Error getting API tokens, please try again",
                }),
            )
        }
    }
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

    let mut pool = &req.state().db;
    let query = sqlx::query!(
        "insert into users (email) values ($1) returning id",
        email.to_owned()
    );

    let user = match query.fetch_one(&mut pool).await {
        Ok(user) => user,
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
    let query = sqlx::query!(
        "insert into api_tokens (contents, user_id) values ($1, $2)",
        token_content,
        user.id
    );
    let mut pool = &req.state().db;
    if let Err(why) = query.execute(&mut pool).await {
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
            id: user.id as u64,
            token: token_content,
        },
    )
}
