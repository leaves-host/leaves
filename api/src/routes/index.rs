use crate::prelude::*;
use tide::{Result as TideResult, StatusCode};

pub async fn get(_: Request) -> TideResult<Response> {
    Ok(Response::new(StatusCode::Ok))
}
