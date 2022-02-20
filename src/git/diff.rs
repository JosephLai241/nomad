//! Exposing functionality for the Git diff command.

use ansi_term::Colour;
use anyhow::Result;
use git2::{Diff, Error, Index, ObjectType, Repository, Tree};

/// Get the diff between the old Git tree and the working directory using the Git index.
pub fn get_repo_diffs<'a>(repo: &'a Repository) -> Result<Diff<'a>, Error> {
    let previous_head = repo.head()?.peel(ObjectType::Tree)?.id();
    let old_tree = repo.find_tree(previous_head)?;

    let diff = repo.diff_tree_to_workdir_with_index(Some(&old_tree), None)?;

    Ok(diff)
}

/// Colorize the origin of the `DiffLine`.
pub fn colorize_origin(marker: char) -> String {
    match marker {
        '+' | '>' => Colour::Green.bold().paint(format!("{marker}")).to_string(),
        '-' | '<' => Colour::Red.bold().paint(format!("{marker}")).to_string(),
        _ => Colour::White.bold().paint(format!("{marker}")).to_string(),
    }
}

/// Get Git diff statistics by comparing the HEAD and index.
pub fn get_diff_stats(
    index: &mut Index,
    old_tree: &Tree,
    repo: &Repository,
) -> (Option<usize>, Option<usize>, Option<usize>) {
    if let Ok(diff) = repo.diff_tree_to_index(Some(old_tree), Some(&index), None) {
        if let Ok(diff_stats) = diff.stats() {
            (
                Some(diff_stats.files_changed()),
                Some(diff_stats.insertions()),
                Some(diff_stats.deletions()),
            )
        } else {
            (None, None, None)
        }
    } else {
        (None, None, None)
    }
}
