use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod git;
mod path;
mod status;

fn main() -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut color = ColorSpec::new();

    stdout.set_color(color.set_fg(Some(Color::Yellow)))?;
    path::write(&mut stdout)?;

    stdout.set_color(color.set_fg(Some(Color::Cyan)))?;
    git::write(&mut stdout)?;

    stdout.set_color(color.set_fg(Some(Color::Red)))?;
    status::write(&mut stdout)?;

    stdout.reset()?;

    write!(stdout, "> ")
}
