use crate::context::Context;
use std::{borrow::Cow, ffi::OsStr};

pub fn generate(context: &Context) -> Option<String> {
    error_state(&context.exit_code).map(|error| {
        let mut output = String::with_capacity(5);
        output.push_str("[");
        output.push_str(&error);
        output.push_str("]");

        output
    })
}

fn error_state(code: &OsStr) -> Option<Cow<'_, str>> {
    if code.is_empty() || code == "0" || code == "True" {
        None
    } else if code == "False" {
        Some(Cow::Borrowed("!"))
    } else {
        Some(code.to_string_lossy())
    }
}
