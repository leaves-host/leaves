use crate::prelude::*;
use async_std::fs;
use log::warn;
use serde_json::json;

pub async fn get(req: Request) -> Response {
    let id = req.param::<String>("id").expect("infallible");
    let query = sqlx::query!("select id, size from files where id = $1", id);
    let mut pool = &req.state().db;

    match query.fetch_one(&mut pool).await {
        Ok(file) => utils::response(
            200,
            &json!({
                "id": file.id,
                "size": file.size,
            }),
        ),
        Err(_) => utils::response(
            404,
            &json!({
                "message": "File doesn't exist",
            }),
        ),
    }
}

pub async fn post(mut req: Request) -> Response {
    let body = match req.body_bytes().await {
        Ok(body) => body,
        Err(_) => {
            return utils::response(
                422,
                &json!({
                    "message": "invalid form",
                }),
            );
        }
    };
    let user = req.local::<User>().expect("user must be present");

    let id = utils::random_string(6);
    let body_size = body.len() as i32;

    let query = { sqlx::query_file!("sql/insert_file.sql", id, body_size, user.id) };
    let mut pool = &req.state().db;

    if let Err(why) = query.fetch_all(&mut pool).await {
        warn!("Failed to create file record: {:?}", why);

        return utils::response(
            500,
            &json!({
                "message": "Failed to create file, please try again",
            }),
        );
    }

    let filepath = format!("./{}", id);

    if let Err(why) = fs::write(&filepath, body).await {
        warn!("Failed to write file to {}: {:?}", filepath, why);

        return Response::new(500);
    }

    let public_url = &req.state().config.public_url;
    let url = format!("{}/{}", public_url, id);

    utils::response(
        201,
        &json!({
            "id": id,
            "size": body_size,
            "url": url,
        }),
    )
}
