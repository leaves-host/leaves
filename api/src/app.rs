use crate::{
    common::middleware::TokenValid,
    error::{Result, ServerInitialization},
    migrations, routes,
    state::State,
};
use snafu::ResultExt;
use std::net::{Ipv4Addr, SocketAddrV4, SocketAddr};
use tide::log::LogMiddleware;

pub async fn run() -> Result<()> {
    let state = State::new().await?;

    {
        let mut conn = state.db.get().unwrap();

        migrations::runner()
            .run(&mut *conn)
            .expect("failed to run migrations");
    }

    let port = state.config.port;

    let mut app = tide::with_state(state);
    app.with(LogMiddleware::new());
    app.at("/").get(routes::index::get);
    app.at("/files")
        .with(TokenValid)
        .post(routes::files::post);
    app.at("/files/:id").get(routes::files::get);
    app.at("/users").post(routes::users::post);
    app.at("/users/:id")
        .with(TokenValid)
        .get(routes::users::get);
    app.at("/users/:id/api-tokens")
        .with(TokenValid)
        .get(routes::users::get_api_tokens);

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port));
    app.listen(addr)
        .await
        .with_context(|| ServerInitialization { port })?;

    Ok(())
}
