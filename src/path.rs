use dunce::canonicalize;
use std::{
    env,
    io::{self, Write},
    path::{MAIN_SEPARATOR, PathBuf},
};
use termcolor::StandardStream;

pub fn write(stdout: &mut StandardStream) -> io::Result<()> {
    let current = match env::current_dir() {
        Ok(cwd) => canonicalize(&cwd).unwrap_or(cwd),
        Err(_) => return Ok(()),
    };

    let short = strip_home_dir(current);
    let normalized = normalize_separators(short);

    write!(stdout, "{} ", normalized)
}

fn strip_home_dir(current: PathBuf) -> String {
    let current_canon = canonicalize(&current).unwrap_or(current);

    if let Some(home) = dirs::home_dir() {
        let home_canon = canonicalize(&home).unwrap_or(home);

        if let Ok(suffix) = current_canon.strip_prefix(home_canon) {
            if suffix.as_os_str().is_empty() {
                return "~".to_string();
            }

            return format!("~{}{}", MAIN_SEPARATOR, suffix.display());
        }
    }

    current_canon.display().to_string()
}

#[cfg(windows)]
fn normalize_separators(path: String) -> String {
    // Heuristic: Only in emulated shells is the "SHELL" environment variable actually set. If it
    // is, then we want to swap the path separator to a forward slash
    if env::var("SHELL").is_ok() {
        path.replace("\\", "/")
    } else {
        path
    }
}

#[cfg(not(windows))]
fn normalize_separators(path: String) -> String {
    path
}
