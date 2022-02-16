//! Exposing functionality for the Git diff command.

use git2::{Index, Repository, Tree};

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
