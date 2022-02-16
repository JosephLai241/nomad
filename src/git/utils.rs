//! Contains useful utilities that support Git functionality.

use std::{
    collections::HashMap,
    path::{Component, Path},
};

use ansi_term::Colour;
use anyhow::{anyhow, Result};
use git2::{Branch, Commit, ObjectType, Repository};
use itertools::Itertools;

use crate::errors::NomadError;

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
