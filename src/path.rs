use crate::context::Context;
use dunce::canonicalize;
use std::{
    env,
    fmt::Write,
    path::{MAIN_SEPARATOR, PathBuf},
};

pub fn generate(context: &Context) -> String {
    let current =
        canonicalize(&context.current_dir).unwrap_or_else(|_| context.current_dir.clone());

    let short = strip_home_dir(current);
    normalize_separators(short)
}

fn strip_home_dir(current: PathBuf) -> String {
    let current_canon = canonicalize(&current).unwrap_or(current);

    if let Some(home) = dirs::home_dir() {
        let home_canon = canonicalize(&home).unwrap_or(home);

        if let Ok(suffix) = current_canon.strip_prefix(home_canon) {
            if suffix.as_os_str().is_empty() {
                return "~".to_string();
            }

            let mut output = String::with_capacity(suffix.as_os_str().len() + 2);
            let _ = write!(&mut output, "~{}{}", MAIN_SEPARATOR, suffix.display());

            return output;
        }
    }

    current_canon.display().to_string()
}

#[cfg(windows)]
fn normalize_separators(path: String) -> String {
    // Heuristic: Only in unix-like shells is the "SHELL" environment variable actually set. If it
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
