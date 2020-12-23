use crate::{common::auth::User, state::State, utils};
use log::warn;
use models::v1::{ApiToken, Signup, User as UserModel};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::json;
use snafu::Snafu;
use tide::{Body, Error as TideError, Request, Response, Result as TideResult, StatusCode};

#[derive(Debug, Snafu)]
enum UserError {
    #[snafu(display("A user with that email address already exists"))]
    EmailTaken,
    #[snafu(display("Failed to execute database query"))]
    QueryExecution,
    #[snafu(display("user is not ready"))]
    UserNonexistent,
}

#[derive(Deserialize, Serialize)]
struct PostBody {
    email: String,
}

pub async fn get(req: Request<State>) -> TideResult<Response> {
    match req.param("id").as_ref().map(AsRef::as_ref) {
        Ok("@me") => {}
        Ok(_) => return Ok(Response::new(StatusCode::Forbidden)),
        Err(_) => return Ok(Response::new(StatusCode::BadRequest)),
    };
    let user = req.ext::<User>().ok_or(UserError::UserNonexistent)?;

    let conn = req.state().db.get().expect("couldn't get connection");
    let user = conn
        .query_row_and_then(
            "select id, email from users where id = ?1",
            &[user.id as i64],
            serde_rusqlite::from_row::<UserModel>,
        )
        .map_err(|why| {
            warn!("Failed to get user {}: {:?}", user.id, why);

            UserError::QueryExecution
        })?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&user)?);

    Ok(res)
}

pub async fn get_api_tokens(req: Request<State>) -> TideResult<Response> {
    let user = req.ext::<User>().ok_or(UserError::UserNonexistent)?;

    let conn = req.state().db.get()?;
    let mut statement = conn
        .prepare("select id, contents, user_id from api_tokens where user_id = ?1")
        .map_err(|why| {
            warn!("Failed to prepare statement: {:?}", why);

            UserError::QueryExecution
        })?;

    let rows = statement
        .query_and_then(params![user.id], serde_rusqlite::from_row::<ApiToken>)
        .map_err(|why| {
            warn!("Failed to get API tokens for user {}: {:?}", user.id, why);

            UserError::QueryExecution
        })?;

    let tokens = rows.filter_map(|r| r.ok()).collect::<Vec<ApiToken>>();

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&tokens)?);

    Ok(res)
}

pub async fn post(mut req: Request<State>) -> TideResult<Response> {
    let PostBody { email } = req.body_json().await.map_err(|_| {
        TideError::from_str(
            StatusCode::BadRequest,
            json!({
                "message": "Body must include an email",
            }),
        )
    })?;
    let email = email.trim();

    let conn = req.state().db.get()?;
    conn.execute("insert into users (email) values (?1)", params![email])
        .map_err(|why| {
            warn!("Failed to create user {}: {:?}", email, why);

            TideError::new(StatusCode::Conflict, UserError::EmailTaken)
        })?;
    let id = conn.last_insert_rowid();

    let token_content = utils::random_string(50);
    let conn = req.state().db.get()?;
    conn.execute(
        "insert into api_tokens (contents, user_id) values (?1, ?2)",
        params![token_content, id],
    )
    .map_err(|why| {
        warn!("Failed to create API token {}: {:?}", token_content, why);

        UserError::QueryExecution
    })?;

    let mut res = Response::new(StatusCode::Created);
    res.set_body(Body::from_json(&Signup {
        email: email.to_owned(),
        id: id as u64,
        token: token_content,
    })?);

    Ok(res)
}
