use std::io::{self, Error as IoError, Write};

pub fn prompt(message: &str) -> Result<String, IoError> {
    {
        let out = io::stdout();
        let mut lock = out.lock();
        write!(lock, "{}", message)?;
        lock.flush()?;
    }

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;

    Ok(s.trim_end().to_owned())
}
