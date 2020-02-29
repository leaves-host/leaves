use crate::prelude::*;
use async_std::fs;
use log::warn;
use rusqlite::params;
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;

#[derive(Deserialize)]
struct TrimmedFileInfo {
    pub id: String,
    pub size: u64,
}

pub async fn get(req: Request) -> Response {
    let id = req.param::<String>("id").expect("infallible");
    let conn = req.state().db.get().unwrap();
    let query = conn.query_row_and_then(
        "select id, size from files where id = ?1",
        &[id],
        serde_rusqlite::from_row::<TrimmedFileInfo>,
    );

    match query {
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
    let body_size = body.len() as i64;

    let conn = req.state().db.get().unwrap();
    let query = conn.execute(
        "insert into files (id, size, user_id) values (?1, ?2, ?3)",
        params![id, body_size, user.id],
    );

    if let Err(why) = query {
        warn!("Failed to create file record: {:?}", why);

        return utils::response(
            500,
            &json!({
                "message": "Failed to create file, please try again",
            }),
        );
    }

    let mut filepath = PathBuf::from(&req.state().config.data_path);
    filepath.push("files");
    filepath.push(&id);

    if let Err(why) = fs::write(&filepath, body).await {
        warn!("Failed to write file to {}: {:?}", filepath.display(), why);

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
