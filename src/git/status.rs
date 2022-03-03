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

use ansi_term::{Colour, Style};
use anyhow::{private, Result};
use git2::{ObjectType, Repository};
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
                    "\n{}\n",
                    Colour::Green
                        .bold()
                        .paint(format!("Nothing to commit. Working tree clean."))
                );

                Ok(())
            } else {
                build_status_tree(args, marker_map, target_directory)?;

                Ok(())
            }
        },
    )
}

/// Get the number of commits ahead of `origin`.
pub fn display_commits_ahead(branch_name: &str, repo: &Repository) -> Result<(), NomadError> {
    let head_oid = repo.head()?.peel(ObjectType::Commit)?.id();

    let origin_branch = format!("origin/{branch_name}");
    let last_commit_oid = repo.revparse_single(&origin_branch)?.id();

    let (ahead, _behind) = repo.graph_ahead_behind(head_oid, last_commit_oid)?;

    if ahead > 0 {
        println!(
            "{} of {} by {} commit{plurality}.\n  └── Run `{}` to publish your local changes.",
            Style::new().underline().paint("Ahead"),
            Colour::Blue.bold().paint(origin_branch),
            Colour::Green.bold().paint(format!("{}", ahead)),
            Style::new().bold().paint("nd git push"),
            plurality = if ahead > 1 { "s" } else { "" }
        );
    } else {
        println!(
            "Up to date with {}.",
            Colour::Blue.bold().paint(origin_branch)
        );
    }

    Ok(())
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
        .filter_map(|(relative_path, marker)| {
            if let Some(ref regex) = regex_expression {
                if let Some(matched) = regex.find(
                    Path::new(&relative_path)
                        .file_name()
                        .unwrap_or(OsStr::new("?"))
                        .to_str()
                        .unwrap_or("?"),
                ) {
                    Some(FoundItem {
                        marker: Some(marker.to_string()),
                        matched: Some((matched.start(), matched.end())),
                        path: relative_path.clone(),
                    })
                } else {
                    None
                }
            } else {
                Some(FoundItem {
                    marker: Some(marker.to_string()),
                    matched: None,
                    path: relative_path.to_string(),
                })
            }
        })
        .sorted_by_key(|found_item| found_item.path.to_string())
        .collect::<Vec<FoundItem>>()
        .transform(target_directory)?
        .to_tree(args, target_directory)?;

    Ok((tree, config))
}
