use crate::{common::middleware::TokenValid, prelude::*, routes, state::State};
use snafu::ResultExt;
use std::net::{Ipv4Addr, SocketAddrV4};
use tide::middleware::RequestLogger;

pub async fn run() -> Result<()> {
    let state = State::new().await?;
    let port = state.config.port;

    let mut app = tide::with_state(state);
    app.middleware(RequestLogger::new());
    app.at("/").get(routes::index::get);
    app.at("/files")
        .middleware(TokenValid)
        .post(routes::files::post);
    app.at("/files/:id").get(routes::files::get);
    app.at("/users").post(routes::users::post);
    app.at("/users/:id")
        .middleware(TokenValid)
        .get(routes::users::get);
    app.at("/users/:id/api-tokens")
        .middleware(TokenValid)
        .get(routes::users::get_api_tokens);

    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    app.listen(addr)
        .await
        .with_context(|| ServerInitialization { port })?;

    Ok(())
}
