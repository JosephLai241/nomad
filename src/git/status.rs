//! Display the Git status command in tree form.

use super::markers::get_status_markers;
use crate::{
    cli::Args,
    errors::NomadError,
    traverse::{
        models::FoundItem,
        traits::{ToTree, TransformFound},
    },
};

use ansi_term::Colour;
use git2::Repository;
use itertools::Itertools;
use ptree::{item::StringItem, PrintConfig};

use std::collections::HashMap;

/// Build a tree that only contains items that are tracked in Git.
pub fn display_status_tree(
    args: &Args,
    repo: &Repository,
    target_directory: &str,
) -> Result<(), NomadError> {
    get_status_markers(&repo, target_directory).map_or_else(
        |error| Err(error),
        |marker_map| {
            if marker_map.is_empty() {
                println!(
                    "{}\n",
                    Colour::Fixed(172).bold().paint(format!("No Git changes."))
                );

                Ok(())
            } else {
                build_status_tree(args, marker_map, target_directory)?;

                Ok(())
            }
        },
    )
}

/// Traverse the repo and build the status tree.
fn build_status_tree(
    args: &Args,
    sliced_markers: HashMap<String, String>,
    target_directory: &str,
) -> Result<(StringItem, PrintConfig), NomadError> {
    let (tree, config) = sliced_markers
        .iter()
        .map(|(relative_path, marker)| FoundItem {
            marker: Some(marker.to_string()),
            path: relative_path.to_string(),
        })
        .sorted_by_key(|found_item| found_item.path.to_string())
        .collect::<Vec<FoundItem>>()
        .transform(target_directory)
        .to_tree(args, target_directory)?;

    Ok((tree, config))
}
