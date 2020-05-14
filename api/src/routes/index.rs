use crate::state::State;
use tide::{Request, Response, Result as TideResult, StatusCode};

pub async fn get(_: Request<State>) -> TideResult<Response> {
    Ok(Response::new(StatusCode::Ok))
}
