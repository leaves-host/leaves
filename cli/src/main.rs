#![deny(clippy::all, clippy::nursery)]
#![forbid(unsafe_code)]

mod command;
mod config;

use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);

    match args.next().as_ref().map(AsRef::as_ref) {
        Some("file") => command::file::run(args)?,
        Some("login") | Some("signin") => command::login::run(args)?,
        Some("logout") | Some("signout") => command::logout::run()?,
        Some("signup") => command::signup::run()?,
        Some("upload") => command::upload::run(args)?,
        _ => {}
    }

    Ok(())
}
