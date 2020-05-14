use crate::prelude::*;
use async_std::fs;
use log::warn;
use rusqlite::params;
use serde::Deserialize;
use serde_json::json;
use snafu::Snafu;
use std::path::PathBuf;
use tide::{Error as TideError, Result as TideResult, StatusCode};

#[derive(Debug, Snafu)]
enum FileError {
    #[snafu(display("Request body invalid"))]
    BodyInvalid,
    #[snafu(display("Failed to create file, please try again"))]
    CreatingFile,
    #[snafu(display("File doesn't exist"))]
    FileNonexistent,
    #[snafu(display("Failed to save file"))]
    SavingFile,
    #[snafu(display("user is not ready"))]
    UserNonexistent,
}

#[derive(Deserialize)]
struct TrimmedFileInfo {
    pub id: String,
    pub size: u64,
}

pub async fn get(req: Request) -> TideResult<Response> {
    let id = req.param::<String>("id")?;
    let conn = req.state().db.get()?;

    let file = conn
        .query_row_and_then(
            "select id, size from files where id = ?1",
            &[id],
            serde_rusqlite::from_row::<TrimmedFileInfo>,
        )
        .map_err(|_| TideError::new(StatusCode::NotFound, FileError::FileNonexistent))?;

    Ok(Response::new(StatusCode::Ok).body_json(&json!({
        "id": file.id,
        "size": file.size,
    }))?)
}

pub async fn post(mut req: Request) -> TideResult<Response> {
    let body = req
        .body_bytes()
        .await
        .map_err(|_| TideError::from_str(StatusCode::BadRequest, FileError::BodyInvalid))?;

    let user = req.local::<User>().ok_or(FileError::UserNonexistent)?;

    let id = utils::random_string(6);
    let body_size = body.len() as i64;

    let conn = req.state().db.get()?;
    conn.execute(
        "insert into files (id, size, user_id) values (?1, ?2, ?3)",
        params![id, body_size, user.id],
    )
    .map_err(|why| {
        warn!("Failed to create file record: {:?}", why);

        FileError::CreatingFile
    })?;

    let mut filepath = PathBuf::from(&req.state().config.data_path);
    filepath.push("files");
    filepath.push(&id);

    fs::write(&filepath, body).await.map_err(|why| {
        warn!("Failed to write file to {}: {:?}", filepath.display(), why);

        FileError::SavingFile
    })?;

    let url = format!("{}/{}", req.state().config.public_url, id);

    Ok(Response::new(StatusCode::Created).body_json(&json!({
        "id": id,
        "size": body_size,
        "url": url,
    }))?)
}
