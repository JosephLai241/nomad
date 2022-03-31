//! Set Git status markers for items within the tree.

use super::utils::get_repo;
use crate::{cli::global::StyleArgs, errors::NomadError, style::models::NomadStyle};

use anyhow::Result;
use git2::{Repository, Status, StatusOptions, StatusShow};

use std::{collections::HashMap, path::Path};

/// Try to extend the `HashMap` containing status markers and their corresponding
/// filenames with new Git repository items.
pub fn extend_marker_map(
    args: &StyleArgs,
    git_markers: &mut HashMap<String, String>,
    nomad_style: &NomadStyle,
    target_directory: &str,
) {
    if let Some(repo) = get_repo(target_directory) {
        if let Ok(top_level_map) = get_status_markers(args, nomad_style, &repo, target_directory) {
            git_markers.extend(top_level_map);
        }
    }
}

/// Get the status markers (colored initials) that correspond with the Git status
/// of tracked files in the repository.
pub fn get_status_markers(
    args: &StyleArgs,
    nomad_style: &NomadStyle,
    repo: &Repository,
    target_directory: &str,
) -> Result<HashMap<String, String>, NomadError> {
    let mut status_options = StatusOptions::new();
    status_options
        .show(StatusShow::IndexAndWorkdir)
        .include_untracked(true)
        .recurse_untracked_dirs(true);

    let mut formatted_items = HashMap::new();

    for repo_item in repo.statuses(Some(&mut status_options))?.iter() {
        let item_name = repo
            .path()
            .parent()
            .unwrap_or_else(|| Path::new(target_directory))
            .join(repo_item.path().unwrap_or("?"))
            .to_str()
            .unwrap_or("?")
            .to_string();

        let items = {
            let marker = match repo_item.status() {
                s if s.contains(Status::INDEX_DELETED) => {
                    if args.no_colors {
                        nomad_style.git.staged_deleted_marker.clone()
                    } else {
                        nomad_style
                            .git
                            .staged_deleted_color
                            .paint(&nomad_style.git.staged_deleted_marker)
                            .to_string()
                    }
                }
                s if s.contains(Status::INDEX_MODIFIED) => {
                    if args.no_colors {
                        nomad_style.git.staged_modified_marker.clone()
                    } else {
                        nomad_style
                            .git
                            .staged_modified_color
                            .paint(&nomad_style.git.staged_modified_marker)
                            .to_string()
                    }
                }
                s if s.contains(Status::INDEX_NEW) => {
                    if args.no_colors {
                        nomad_style.git.staged_added_marker.clone()
                    } else {
                        nomad_style
                            .git
                            .staged_added_color
                            .paint(&nomad_style.git.staged_added_marker)
                            .to_string()
                    }
                }
                s if s.contains(Status::INDEX_RENAMED) => {
                    if args.no_colors {
                        nomad_style.git.staged_renamed_marker.clone()
                    } else {
                        nomad_style
                            .git
                            .staged_renamed_color
                            .paint(&nomad_style.git.staged_renamed_marker)
                            .to_string()
                    }
                }
                s if s.contains(Status::INDEX_TYPECHANGE) => {
                    if args.no_colors {
                        nomad_style.git.staged_typechanged_marker.clone()
                    } else {
                        nomad_style
                            .git
                            .staged_typechanged_color
                            .paint(&nomad_style.git.staged_typechanged_marker)
                            .to_string()
                    }
                }
                s if s.contains(Status::WT_DELETED) => {
                    if args.no_colors {
                        nomad_style.git.deleted_marker.clone()
                    } else {
                        nomad_style
                            .git
                            .deleted_color
                            .paint(&nomad_style.git.deleted_marker)
                            .to_string()
                    }
                }
                s if s.contains(Status::WT_MODIFIED) => {
                    if args.no_colors {
                        nomad_style.git.modified_marker.clone()
                    } else {
                        nomad_style
                            .git
                            .modified_color
                            .paint(&nomad_style.git.modified_marker)
                            .to_string()
                    }
                }
                s if s.contains(Status::WT_NEW) => {
                    if args.no_colors {
                        nomad_style.git.untracked_marker.clone()
                    } else {
                        nomad_style
                            .git
                            .untracked_color
                            .paint(&nomad_style.git.untracked_marker)
                            .to_string()
                    }
                }
                s if s.contains(Status::WT_RENAMED) => {
                    if args.no_colors {
                        nomad_style.git.renamed_marker.clone()
                    } else {
                        nomad_style
                            .git
                            .renamed_color
                            .paint(&nomad_style.git.renamed_marker)
                            .to_string()
                    }
                }
                s if s.contains(Status::WT_TYPECHANGE) => {
                    if args.no_colors {
                        nomad_style.git.typechanged_marker.clone()
                    } else {
                        nomad_style
                            .git
                            .typechanged_color
                            .paint(&nomad_style.git.typechanged_marker)
                            .to_string()
                    }
                }
                s if s.contains(Status::CONFLICTED) => {
                    if args.no_colors {
                        nomad_style.git.conflicted_marker.clone()
                    } else {
                        nomad_style
                            .git
                            .conflicted_color
                            .paint(&nomad_style.git.conflicted_marker)
                            .to_string()
                    }
                }
                _ => "".to_string(),
            };

            (item_name, marker)
        };

        formatted_items.insert(items.0, items.1);
    }

    Ok(formatted_items)
}
