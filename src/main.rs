use std::fmt::Write;

mod color;
mod context;
mod git;
mod path;
mod status;

use color::Color;
use context::Context;

fn main() {
    let Some(context) = Context::from_env() else {
        return;
    };
    let mut output = String::with_capacity(128);

    output.push_str("\n");

    // Path info
    output.push_str(Color::Yellow.as_str());
    output.push_str(&path::generate(&context));

    // Git Info
    if let Some(git_state) = git::generate(&context) {
        let _ = write!(output, " {}{}", Color::Cyan.as_str(), git_state);
    }

    // Error Info
    if let Some(error) = status::generate(&context) {
        output.push_str(" ");
        output.push_str(Color::Red.as_str());
        output.push_str(&error);
    }

    output.push_str(Color::Reset.as_str());
    output.push_str(" \u{276F} ");

    print!("{output}");
}
