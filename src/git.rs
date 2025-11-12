use gix::{Repository, state::InProgress};
use std::{
    env,
    io::{self, Write},
};
use termcolor::StandardStream;

pub fn write(stdout: &mut StandardStream) -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let Ok(repository) = gix::discover(current_dir) else {
        return Ok(());
    };

    let branch = branch_name(&repository);
    let op_state = operation_state(&repository);

    write!(stdout, "({branch}")?;

    if let Some(operation) = op_state {
        write!(stdout, "|{operation}")?;
    }

    write!(stdout, ") ")
}

fn branch_name(repo: &Repository) -> String {
    if let Ok(Some(name)) = repo.head_name() {
        name.shorten().to_string()
    } else if let Ok(id) = repo.head_id() {
        format!("({}...)", id.shorten_or_id())
    } else {
        "HEAD".to_string()
    }
}

fn operation_state(repo: &Repository) -> Option<&'static str> {
    match repo.state() {
        Some(InProgress::Bisect) => Some("BISECTING"),
        Some(InProgress::CherryPick | InProgress::CherryPickSequence) => Some("CHERRY-PICKING"),
        Some(InProgress::Rebase | InProgress::RebaseInteractive) => Some("REBASING"),
        Some(InProgress::Merge) => Some("MERGING"),
        Some(InProgress::Revert | InProgress::RevertSequence) => Some("REVERTING"),
        _ => None,
    }
}
