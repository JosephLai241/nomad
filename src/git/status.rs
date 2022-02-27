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
use anyhow::private;
use git2::Repository;
use itertools::Itertools;
use ptree::{item::StringItem, PrintConfig};
use regex::Regex;

use std::{collections::HashMap, ffi::OsStr, path::Path};

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
    marker_map: HashMap<String, String>,
    target_directory: &str,
) -> Result<(StringItem, PrintConfig), NomadError> {
    let regex_expression = if let Some(ref pattern) = args.pattern {
        match Regex::new(&pattern.clone()) {
            Ok(regex) => Some(regex),
            Err(error) => return private::Err(NomadError::RegexError(error)),
        }
    } else {
        None
    };

    let (tree, config) = marker_map
        .iter()
        .map(|(relative_path, marker)| {
            if let Some(ref regex) = regex_expression {
                if let Some(matched) = regex.find(
                    Path::new(&relative_path)
                        .file_name()
                        .unwrap_or(OsStr::new("?"))
                        .to_str()
                        .unwrap_or("?"),
                ) {
                    FoundItem {
                        marker: Some(marker.to_string()),
                        matched: Some((matched.start(), matched.end())),
                        path: relative_path.clone(),
                    }
                } else {
                    FoundItem {
                        marker: Some(marker.to_string()),
                        matched: None,
                        path: relative_path.to_string(),
                    }
                }
            } else {
                FoundItem {
                    marker: Some(marker.to_string()),
                    matched: None,
                    path: relative_path.to_string(),
                }
            }
        })
        .sorted_by_key(|found_item| found_item.path.to_string())
        .collect::<Vec<FoundItem>>()
        .transform(target_directory)?
        .to_tree(args, target_directory)?;

    Ok((tree, config))
}
