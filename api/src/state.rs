use crate::{
    config::Config,
    error::{Result, R2d2Initialization},
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use snafu::ResultExt;

pub struct State {
    pub config: Config,
    pub db: Pool<SqliteConnectionManager>,
}

impl State {
    pub async fn new() -> Result<Self> {
        let config = Config::new()?;
        let manager = SqliteConnectionManager::file(&config.db_path);
        let db = Pool::builder().max_size(5).build(manager).context(R2d2Initialization)?;

        Ok(Self { config, db })
    }
}
