use crate::context::Context;
use gix::{Repository, state::InProgress};
use std::fmt;

pub struct GitState {
    head_name: String,
    operation_state: Option<&'static str>,
}

impl fmt::Display for GitState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}", self.head_name)?;

        if let Some(op) = self.operation_state {
            write!(f, "|{}", op)?;
        }

        f.write_str(")")
    }
}

pub fn generate(context: &Context) -> Option<GitState> {
    let Ok(repository) = gix::discover(&context.current_dir) else {
        return None;
    };

    Some(GitState {
        head_name: head_name(&repository),
        operation_state: operation_state(&repository),
    })
}

fn head_name(repo: &Repository) -> String {
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
