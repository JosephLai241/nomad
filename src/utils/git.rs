//! Providing Git functionality in tree form.

use ansi_term::Colour;
use git2::{Error, Repository, Status, StatusOptions, StatusShow};

use std::collections::HashMap;

/// Try to get Git metadata from the target directory.
pub fn get_repo(target_directory: &str) -> Option<Repository> {
    Repository::open(target_directory).map_or_else(
        |error| {
            println!("\n{}", Colour::Red.bold().paint(format!("{error}")));
            None
        },
        |repo| {
            if repo.is_bare() {
                println!("\n{}", Colour::Fixed(172).paint("Git repository is bare!"));
                None
            } else {
                Some(repo)
            }
        },
    )
}

/// Get the status markers (colored initials) that correspond with the Git status
/// of tracked files in the repository.
pub fn get_status_markers(repo: Repository) -> Result<HashMap<String, String>, Error> {
    let mut status_options = StatusOptions::new();
    status_options
        .show(StatusShow::IndexAndWorkdir)
        .include_untracked(true)
        .recurse_untracked_dirs(true);

    let mut formatted_items = HashMap::new();

    for repo_item in repo.statuses(Some(&mut status_options))?.iter() {
        let item_name = repo_item.path().unwrap_or("?").to_string();

        match repo_item.status() {
            s if s.contains(Status::INDEX_DELETED) => {
                formatted_items.insert(item_name, Colour::Red.bold().paint("ID").to_string());
            }
            s if s.contains(Status::INDEX_MODIFIED) => {
                formatted_items
                    .insert(item_name, Colour::Fixed(172).bold().paint("IM").to_string());
            }
            s if s.contains(Status::INDEX_NEW) => {
                formatted_items.insert(item_name, Colour::Green.bold().paint("IU").to_string());
            }
            s if s.contains(Status::INDEX_RENAMED) => {
                formatted_items.insert(item_name, Colour::Red.bold().paint("IR").to_string());
            }
            s if s.contains(Status::WT_DELETED) => {
                formatted_items.insert(item_name, Colour::Red.bold().paint("WD").to_string());
            }
            s if s.contains(Status::WT_MODIFIED) => {
                formatted_items
                    .insert(item_name, Colour::Fixed(172).bold().paint("WM").to_string());
            }
            s if s.contains(Status::WT_NEW) => {
                formatted_items.insert(item_name, Colour::Green.bold().paint("WU").to_string());
            }
            s if s.contains(Status::WT_RENAMED) => {
                formatted_items.insert(item_name, Colour::Red.bold().paint("WR").to_string());
            }
            _ => {}
        }
    }

    Ok(formatted_items)
}
