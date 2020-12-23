use crate::{
    config::Config,
    error::{Error, Result},
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::{path::PathBuf, sync::Arc};

#[derive(Clone, Debug)]
pub struct State {
    pub config: Arc<Config>,
    pub db: Pool<SqliteConnectionManager>,
}

impl State {
    pub async fn new() -> Result<Self> {
        let config = Arc::new(Config::new()?);
        let mut path = PathBuf::from(&config.data_path);
        path.push("db");
        let manager = SqliteConnectionManager::file(path);
        let db = Pool::builder()
            .max_size(5)
            .build(manager)
            .map_err(|source| Error::R2d2Initialization { source })?;

        Ok(Self { config, db })
    }
}
