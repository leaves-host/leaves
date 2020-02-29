#![deny(clippy::all, clippy::nursery)]
#![forbid(unsafe_code)]

mod app;
mod common;
mod config;
mod error;
mod migrations;
mod prelude;
mod routes;
mod state;
pub mod utils;

use async_std::task;
use log::LevelFilter;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    femme::pretty::Logger::new().start(LevelFilter::Info)?;

    task::block_on(app::run())?;

    Ok(())
}
