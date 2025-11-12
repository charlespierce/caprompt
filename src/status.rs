use std::{
    borrow::Cow,
    env,
    ffi::OsStr,
    io::{self, Write},
};
use termcolor::StandardStream;

pub fn write(stdout: &mut StandardStream) -> io::Result<()> {
    if let Some(code) = env::args_os().nth(1)
        && let Some(error) = error_code(&code)
    {
        write!(stdout, "[{}] ", error)?;
    }

    Ok(())
}

fn error_code(code: &OsStr) -> Option<Cow<'_, str>> {
    if code == "0" || code == "True" {
        None
    } else if code == "False" {
        Some(Cow::Borrowed("!"))
    } else {
        Some(code.to_string_lossy())
    }
}
