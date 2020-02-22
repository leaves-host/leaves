use crate::{
    config::Config,
    error::{Result, SqlxInitialization},
};
use snafu::ResultExt;
use sqlx::PgPool;

pub struct State {
    pub config: Config,
    pub db: PgPool,
}

impl State {
    pub async fn new() -> Result<Self> {
        let config = Config::new()?;
        let db = PgPool::new(&config.db_url)
            .await
            .context(SqlxInitialization)?;

        Ok(Self { config, db })
    }
}
