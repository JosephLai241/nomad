//! Search for a file within the tree.

use std::path::Path;

use crate::git::markers::get_status_markers;

use super::open::get_deserialized_json;

use ansi_term::Colour;
use git2::Repository;

/// Modes for file searching.
pub enum SearchMode {
    /// Search for files when using Git commands besides Git diff. If a directory
    /// label is passed, directory items will be compared with the Git status marker
    /// map. Only matching items that are tracked by Git AND are changed will be returned.
    Git,
    /// Search for changed items that are tracked by Git. Mutes the warning message
    /// that usually appears if no item labels are passed into a subcommand.
    GitDiff,
    /// Search for files in normal mode. If a directory label is passed, all
    /// directory items are returned regardless of Git status.
    Normal,
}

/// Get files by its number in the tree, or traverse a directory and return all files
/// within it.
///
/// If this function is in Git mode and directory labels are passed into
/// `item_labels`, only matching items that are tracked by Git AND are changed
/// will be returned.
///
/// If this function is in normal mode and directory labels are passed into
/// `item_labels`, all items within that directory are returned.
pub fn indiscriminate_search(
    item_labels: &Vec<String>,
    repo: Option<&Repository>,
    search_mode: SearchMode,
    target_directory: &str,
) -> Option<Vec<String>> {
    if let Ok(contents) = get_deserialized_json() {
        let mut found: Vec<String> = Vec::new();
        let mut not_found: Vec<String> = Vec::new();

        for label in item_labels {
            match label.parse::<i32>() {
                Ok(_) => match contents.numbered.get(label) {
                    Some(file_path) => {
                        found.push(file_path.to_string());
                    }
                    None => not_found.push(label.into()),
                },
                Err(_) => match contents.labeled.get(label) {
                    Some(directory_path) => match search_mode {
                        SearchMode::Git | SearchMode::GitDiff => {
                            if let Some(repo) = repo {
                                if let Ok(marker_map) = get_status_markers(repo, target_directory) {
                                    for file_path in marker_map.keys() {
                                        let path_parent = Path::new(file_path)
                                            .parent()
                                            .unwrap_or(Path::new("?"))
                                            .to_str()
                                            .unwrap_or("?");

                                        if path_parent.contains(directory_path) {
                                            found.push(file_path.to_string());
                                        }
                                    }
                                } else {
                                    println!(
                                        "{}",
                                        Colour::Red.bold().paint(
                                            "\nCould not get the HashMap containing Git items!\n"
                                        )
                                    );
                                }
                            } else {
                                println!(
                                    "{}",
                                    Colour::Red.bold().paint(
                                        "\nUnable to search for Git files: The Git repository is missing!\n"
                                    )
                                );
                            }
                        }
                        SearchMode::Normal => {
                            for path in contents.numbered.values() {
                                if path.contains(directory_path) {
                                    found.push(path.to_owned());
                                }
                            }
                        }
                    },
                    None => not_found.push(label.into()),
                },
            };
        }

        if !not_found.is_empty() {
            println!(
                "{}",
                Colour::Fixed(172).bold().paint(
                    "\nThe following item numbers or directory labels did not match any items in the tree:\n"
                )
            );

            for label in not_found {
                println!(
                    "==> {}",
                    Colour::Fixed(172).bold().paint(format!("{label}"))
                );
            }
        }

        if !found.is_empty() {
            Some(found)
        } else {
            match search_mode {
                SearchMode::Git => println!(
                    "{}",
                    Colour::Fixed(172).bold().paint(
                        "\nDid not find any changed files matching the labels you've entered.\nAre you sure the file or directory contains changed files tracked by Git?\n"
                    )
                ),
                SearchMode::GitDiff => {
                    if !item_labels.is_empty() {
                        println!(
                            "{}",
                            Colour::Fixed(172).bold().paint("\nDid not find any changed files matching the labels you've entered.\nDisplaying all diffs.\n"));
                    }
                }
                SearchMode::Normal => println!("{}", Colour::Red.bold().paint("\nNo items were matched!\n")),
            }

            None
        }
    } else {
        println!(
            "{}",
            Colour::Red.bold().paint(
                "\nCould not retrieve stored directories and directory contents!\nDid you run nomad in numbered or labeled directories mode?\n"
            )
        );

        None
    }
}
