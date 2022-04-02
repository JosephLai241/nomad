//! Display the Git status command in tree form.

use super::markers::get_status_markers;
use crate::{
    cli::{
        git,
        global::{GlobalArgs, LabelArgs, MetaArgs, ModifierArgs, RegexArgs, StyleArgs},
    },
    errors::NomadError,
    style::models::NomadStyle,
    traverse::{
        models::FoundItem,
        modes::NomadMode,
        traits::{ToTree, TransformFound},
    },
};

use ansi_term::{Colour, Style};
use anyhow::{private, Result};
use git2::{ObjectType, Repository};
use itertools::Itertools;
use ptree::{item::StringItem, PrintConfig};
use regex::Regex;

use std::{collections::HashMap, path::Path};

/// Build a tree that only contains items that are tracked in Git.
pub fn display_status_tree(
    args: &git::StatusOptions,
    nomad_style: &NomadStyle,
    repo: &Repository,
    target_directory: &str,
) -> Result<Option<(StringItem, PrintConfig)>, NomadError> {
    // Hm... There is probably a better solution, but fuck it. Leaving it for now.
    let global_args = GlobalArgs {
        export: args.export.clone(),
        labels: LabelArgs {
            all_labels: args.labels.all_labels,
            label_directories: args.labels.label_directories,
            numbers: args.labels.numbers,
        },
        meta: MetaArgs {
            metadata: args.meta.metadata,
            tokei: args.meta.tokei,
        },
        modifiers: ModifierArgs {
            dirs: false,
            disrespect: false,
            hidden: false,
            max_depth: None,
            max_filesize: None,
        },
        regex: RegexArgs {
            pattern: args.regex.pattern.clone(),
        },
        style: StyleArgs {
            no_colors: args.style.no_colors,
            no_git: args.style.no_git,
            no_icons: args.style.no_icons,
            plain: args.style.plain,
        },
        statistics: args.statistics,
    };

    get_status_markers(&args.style, nomad_style, repo, target_directory).map_or_else(
        Err,
        |marker_map| {
            if marker_map.is_empty() {
                println!(
                    "\n{}\n",
                    Colour::Green
                        .bold()
                        .paint("Nothing to commit. Working tree clean.".to_string())
                );

                Ok(None)
            } else {
                Ok(Some(build_status_tree(
                    &global_args,
                    marker_map,
                    nomad_style,
                    target_directory,
                )?))
            }
        },
    )
}

/// Get the number of commits ahead of `origin`.
pub fn display_commits_ahead(branch_name: &str, repo: &Repository) -> Result<(), NomadError> {
    let head_oid = repo.head()?.peel(ObjectType::Commit)?.id();

    let origin_branch = format!("origin/{branch_name}");

    if let Ok(git_object) = repo.revparse_single(&origin_branch) {
        let last_commit_oid = git_object.id();

        let (ahead, _behind) = repo.graph_ahead_behind(head_oid, last_commit_oid)?;

        if ahead > 0 {
            println!(
                "{} of {} by {} commit{plurality}.\n  └── Run `{}` to publish your local changes.",
                Style::new().underline().paint("Ahead"),
                Colour::Blue.bold().paint(origin_branch),
                Colour::Green.bold().paint(format!("{}", ahead)),
                Style::new().bold().paint("git push"),
                plurality = if ahead > 1 { "s" } else { "" }
            );
        } else {
            println!(
                "Up to date with {}.",
                Colour::Blue.bold().paint(origin_branch)
            );
        }
    } else {
        println!(
            "{}",
            Colour::Fixed(172).bold().paint("No upstream branch found.")
        );
    }

    Ok(())
}

/// Traverse the repo and build the status tree.
fn build_status_tree(
    args: &GlobalArgs,
    marker_map: HashMap<String, String>,
    nomad_style: &NomadStyle,
    target_directory: &str,
) -> Result<(StringItem, PrintConfig), NomadError> {
    let regex_expression = if let Some(ref pattern) = args.regex.pattern {
        match Regex::new(&pattern.clone()) {
            Ok(regex) => Some(regex),
            Err(error) => return private::Err(NomadError::RegexError(error)),
        }
    } else {
        None
    };

    let (tree, config, _) = marker_map
        .iter()
        .filter_map(|(absolute_path, marker)| {
            if absolute_path.contains(target_directory) {
                match regex_expression {
                    Some(ref regex) => match regex.find(
                        Path::new(&absolute_path)
                            .strip_prefix(target_directory)
                            .unwrap_or_else(|_| Path::new("?"))
                            .to_str()
                            .unwrap_or("?"),
                    ) {
                        Some(matched) => Some(FoundItem {
                            marker: Some(marker.to_string()),
                            matched: Some((matched.start(), matched.end())),
                            path: absolute_path.clone(),
                        }),
                        None => None,
                    },
                    None => Some(FoundItem {
                        marker: Some(marker.to_string()),
                        matched: None,
                        path: absolute_path.to_string(),
                    }),
                }
            } else {
                None
            }
        })
        .sorted_by_key(|found_item| found_item.path.to_string())
        .collect::<Vec<FoundItem>>()
        .transform(target_directory)?
        .to_tree(args, NomadMode::GitStatus, nomad_style, target_directory)?;

    Ok((tree, config))
}
