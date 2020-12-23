use crate::{common::auth::User, state::State, utils};
use async_std::fs;
use log::warn;
use rusqlite::params;
use serde::Deserialize;
use serde_json::json;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    path::PathBuf,
};
use tide::{Body, Error as TideError, Request, Response, Result as TideResult, StatusCode};

#[derive(Debug)]
enum FileError {
    BodyInvalid,
    CreatingFile,
    FileNonexistent,
    SavingFile,
    UserNonexistent,
}

impl Display for FileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::BodyInvalid => f.write_str("Request<State> body invalid"),
            Self::CreatingFile => f.write_str("Failed to create file, please try again"),
            Self::FileNonexistent => f.write_str("File doesn't exist"),
            Self::SavingFile => f.write_str("Failed to save file"),
            Self::UserNonexistent => f.write_str("user is not ready"),
        }
    }
}

impl Error for FileError {}

#[derive(Deserialize)]
struct TrimmedFileInfo {
    pub id: String,
    pub size: u64,
}

pub async fn get(req: Request<State>) -> TideResult<Response> {
    let id = req.param("id")?;
    let conn = req.state().db.get()?;

    let file = conn
        .query_row_and_then(
            "select id, size from files where id = ?1",
            &[id],
            serde_rusqlite::from_row::<TrimmedFileInfo>,
        )
        .map_err(|_| TideError::new(StatusCode::NotFound, FileError::FileNonexistent))?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&json!({
        "id": file.id,
        "size": file.size,
    }))?);

    Ok(res)
}

pub async fn post(mut req: Request<State>) -> TideResult<Response> {
    let body = req
        .body_bytes()
        .await
        .map_err(|_| TideError::from_str(StatusCode::BadRequest, FileError::BodyInvalid))?;

    let user = req.ext::<User>().ok_or(FileError::UserNonexistent)?;

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

    let mut res = Response::new(StatusCode::Created);
    res.set_body(Body::from_json(&json!({
        "id": id,
        "size": body_size,
        "url": url,
    }))?);

    Ok(res)
}
