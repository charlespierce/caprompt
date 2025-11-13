use std::{env, ffi::OsString, path::PathBuf};

pub struct Context {
    pub current_dir: PathBuf,
    pub exit_code: OsString,
}

impl Context {
    pub fn from_env() -> Option<Self> {
        let Ok(current_dir) = env::current_dir() else {
            return None;
        };
        let exit_code = env::args_os().nth(1).unwrap_or_else(OsString::new);

        Some(Context {
            current_dir,
            exit_code,
        })
    }
}
