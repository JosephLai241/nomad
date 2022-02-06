//! Set Git status markers for items within the tree.

use super::utils::get_repo;
use crate::utils::paths::canonicalize_path;

use ansi_term::Colour;
use git2::{Error, Repository, Status, StatusOptions, StatusShow};

use std::collections::HashMap;

/// Try to extend the `HashMap` containing status markers and their corresponding
/// filenames with new Git repository items.
pub fn extend_marker_map(git_markers: &mut HashMap<String, String>, target_directory: &str) {
    if let Some(repo) = get_repo(target_directory) {
        if let Ok(top_level_map) = get_status_markers(&repo, target_directory) {
            git_markers.extend(top_level_map);
        }
    }
}

/// Get the status markers (colored initials) that correspond with the Git status
/// of tracked files in the repository.
pub fn get_status_markers(
    repo: &Repository,
    target_directory: &str,
) -> Result<HashMap<String, String>, Error> {
    let mut status_options = StatusOptions::new();
    status_options
        .show(StatusShow::IndexAndWorkdir)
        .include_untracked(true)
        .recurse_untracked_dirs(true);

    let mut formatted_items = HashMap::new();

    for repo_item in repo.statuses(Some(&mut status_options))?.iter() {
        let item_path = format!("{target_directory}/{}", repo_item.path().unwrap_or("?"));
        let item_name = canonicalize_path(&item_path).unwrap_or("?".to_string());

        match repo_item.status() {
            s if s.contains(Status::INDEX_DELETED) => {
                formatted_items.insert(item_name, Colour::Red.bold().paint("SD").to_string());
            }
            s if s.contains(Status::INDEX_MODIFIED) => {
                formatted_items.insert(item_name, Colour::Yellow.bold().paint("SM").to_string());
            }
            s if s.contains(Status::INDEX_NEW) => {
                formatted_items.insert(item_name, Colour::Green.bold().paint("SA").to_string());
            }
            s if s.contains(Status::INDEX_RENAMED) => {
                formatted_items
                    .insert(item_name, Colour::Fixed(172).bold().paint("SR").to_string());
            }
            s if s.contains(Status::WT_DELETED) => {
                formatted_items.insert(item_name, Colour::Red.bold().paint("D").to_string());
            }
            s if s.contains(Status::WT_MODIFIED) => {
                formatted_items.insert(item_name, Colour::Yellow.bold().paint("M").to_string());
            }
            s if s.contains(Status::WT_NEW) => {
                formatted_items.insert(item_name, Colour::Green.bold().paint("U").to_string());
            }
            s if s.contains(Status::WT_RENAMED) => {
                formatted_items.insert(item_name, Colour::Fixed(172).bold().paint("R").to_string());
            }
            s if s.contains(Status::CONFLICTED) => {
                formatted_items.insert(item_name, Colour::Red.bold().paint("CONFLICT").to_string());
            }
            _ => {}
        }
    }

    Ok(formatted_items)
}
