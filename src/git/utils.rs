//! Contains useful utilities that support Git functionality.

use crate::{
    errors::NomadError,
    utils::{open::get_deserialized_json, temp::JSONTarget},
};

use std::{
    collections::HashMap,
    path::{Component, Path},
};

use ansi_term::Colour;
use anyhow::{anyhow, Result};
use git2::{Branch, Commit, ObjectType, Repository};
use itertools::Itertools;

use super::markers::get_status_markers;

/// Try to discover a Git repository at or above the current path.
fn discover_repo(target_directory: &str) -> Option<Repository> {
    if let Ok(repo) = Repository::discover(target_directory) {
        if repo.is_bare() {
            println!("\n{}", Colour::Fixed(172).paint("Git repository is bare!"));
            None
        } else {
            Some(repo)
        }
    } else {
        None
    }
}

/// Try to get Git metadata from the target directory.
pub fn get_repo(target_directory: &str) -> Option<Repository> {
    if let Ok(repo) = Repository::open(target_directory) {
        if repo.is_bare() {
            println!("\n{}", Colour::Fixed(172).paint("Git repository is bare!"));
            None
        } else {
            Some(repo)
        }
    } else {
        discover_repo(target_directory)
    }
}

/// Try to get the current Git branch's name.
pub fn get_repo_branch(repo: &Repository) -> Option<String> {
    if let Ok(reference) = repo.head() {
        if let Ok(Some(name)) = Branch::wrap(reference).name() {
            let branch_name = name.to_string();
            Some(branch_name)
        } else {
            println!(
                "\n{}\n",
                Colour::Red
                    .bold()
                    .paint("Could not get the current Git branch name!")
            );
            None
        }
    } else {
        println!(
            "\n{}\n",
            Colour::Red.bold().paint("Could not get repository HEAD!")
        );
        None
    }
}

/// Get the last commit in the Git repository.
pub fn get_last_commit(repo: &Repository) -> Result<Commit, NomadError> {
    let object = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    object.into_commit().map_err(|_| {
        NomadError::Error(anyhow!(
            "Could not find the last commit in this Git repository!"
        ))
    })
}

/// Contains metadata for each modified item in the Git repository.
#[derive(Debug)]
pub struct ModifiedItem {
    /// The filepath broken down into its individual components.
    pub components: Vec<String>,
    /// The depth of the file relative to the root of the Git repository.
    pub depth: i32,
    /// The Git status marker indicating the change that was made to the file.
    pub marker: String,
    /// The filepath.
    pub path: String,
}

/// Get the depth of each staged item and transform the HashMap into a Vec of tuples.
pub fn add_marker_depths(sliced_markers: HashMap<String, String>) -> Vec<ModifiedItem> {
    let mut markers = Vec::new();

    for (path, marker) in sliced_markers.iter().sorted() {
        let item = Path::new(path);

        let mut components = Vec::new();
        let mut depth = 0;
        for component in item.components() {
            match component {
                Component::Normal(section) => {
                    components.push(section.to_str().unwrap_or("?").to_string());
                    depth += 1;
                }
                _ => {}
            }
        }

        markers.push(ModifiedItem {
            components,
            depth,
            marker: marker.to_owned(),
            path: path.to_owned(),
        });
    }

    markers
}

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
pub fn indiscriminate_file_search(
    item_labels: &Vec<String>,
    repo: Option<&Repository>,
    search_mode: SearchMode,
    target_directory: &str,
) -> Option<Vec<String>> {
    if let (Ok(contents), Ok(directories)) = (
        get_deserialized_json(JSONTarget::Contents),
        get_deserialized_json(JSONTarget::Directories),
    ) {
        let mut found: Vec<String> = Vec::new();
        let mut not_found: Vec<String> = Vec::new();

        for label in item_labels {
            match label.parse::<i32>() {
                Ok(_) => match contents.items.get(label) {
                    Some(file_path) => {
                        found.push(file_path.to_string());
                    }
                    None => not_found.push(label.into()),
                },
                Err(_) => match directories.items.get(label) {
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
                            for path in contents.items.values() {
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
                SearchMode::Normal => println!(
                    "{}",
                    Colour::Red
                        .bold()
                        .paint("\nNo items were matched!\n")
                ),
                _ => {}
            }

            None
        }
    } else {
        println!("{}", Colour::Red.bold().paint("\nCould not retrieve stored directories and directory contents!\nDid you run nomad in numbered or labeled directories mode?\n"));

        None
    }
}

/// Strip prefix paths (the absolute path preceding the current target directory)
/// from existing paths in the Git status marker map.
pub fn strip_prefixes(
    current_dir: &str,
    marker_map: HashMap<String, String>,
) -> HashMap<String, String> {
    marker_map
        .iter()
        .map(|(key, value)| {
            let key_path = Path::new(key);
            let stripped_key = key_path
                .strip_prefix(current_dir)
                .expect(&format!(
                    "Could not strip path prefix for {}!",
                    key_path.to_str().unwrap_or("?")
                ))
                .to_str()
                .unwrap_or("?");

            (stripped_key.to_owned(), value.to_owned())
        })
        .collect()
}
