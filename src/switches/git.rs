//! Executing Git subcommands.

use crate::{
    cli::{git::GitOptions, Args},
    errors::NomadError,
    git::{
        blame::bat_blame,
        branch::display_branches,
        commit::commit_changes,
        diff::{bat_diffs, get_repo_diffs},
        status::{display_commits_ahead, display_status_tree},
        trees::{modify_trees, TreeMode},
        utils::{get_repo, get_repo_branch},
    },
    style::models::NomadStyle,
    utils::{
        export::{export_tree, ExportMode},
        paint::paint_error,
        search::{indiscriminate_search, SearchMode},
    },
};

use ansi_term::Colour;
use anyhow::anyhow;

pub fn run_git(
    args: &Args,
    git_command: &GitOptions,
    nomad_style: &NomadStyle,
    target_directory: &str,
) {
    if let Some(repo) = get_repo(target_directory) {
        match git_command {
            GitOptions::Add(add_options) => {
                let stage_mode = match add_options.all {
                    true => TreeMode::StageAll,
                    false => TreeMode::Stage,
                };

                if let Err(error) = modify_trees(
                    args,
                    &add_options.item_labels,
                    nomad_style,
                    &repo,
                    stage_mode,
                    target_directory,
                ) {
                    paint_error(NomadError::GitError {
                        context: "Unable to stage files".into(),
                        source: error,
                    });
                }
            }
            GitOptions::Blame(blame_options) => match blame_options.file_number.parse::<i32>() {
                Ok(file_number) => {
                    match indiscriminate_search(
                        args,
                        &[file_number.to_string()],
                        nomad_style,
                        Some(&repo),
                        SearchMode::Normal,
                        target_directory,
                    ) {
                        Some(ref mut found_items) => match found_items.pop() {
                            Some(item) => {
                                if blame_options.lines.len() > 2 {
                                    println!(
                                        "\n{}\n",
                                        Colour::Red
                                            .bold()
                                            .paint("Line range only takes two values - a lower and upper bound")
                                    );
                                } else if let Err(error) =
                                    bat_blame(item, blame_options, &repo, target_directory)
                                {
                                    paint_error(error);
                                }
                            }
                            None => println!(
                                "\n{}\n",
                                Colour::Red.bold().paint("Could not find a file to blame!")
                            ),
                        },
                        None => println!(
                            "\n{}\n",
                            Colour::Red.bold().paint("Could not find a file to blame!")
                        ),
                    }
                }
                Err(_) => paint_error(NomadError::GitBlameError),
            },
            GitOptions::Branch(branch_options) => {
                match display_branches(branch_options, nomad_style, &repo, target_directory) {
                    Ok(tree_items) => {
                        if let Some((tree, config, _)) = tree_items {
                            if let Some(export) = &branch_options.export {
                                if let Err(error) =
                                    export_tree(config, ExportMode::GitBranch, export, tree)
                                {
                                    paint_error(error);
                                }
                            }
                        }
                    }
                    Err(error) => paint_error(error),
                }
            }
            GitOptions::Commit { message } => {
                if let Err(error) = commit_changes(message, &repo) {
                    paint_error(error);
                }
            }
            GitOptions::Diff { item_labels } => match get_repo_diffs(&repo) {
                Ok(diff) => {
                    match indiscriminate_search(
                        args,
                        item_labels,
                        nomad_style,
                        Some(&repo),
                        SearchMode::GitDiff,
                        target_directory,
                    ) {
                        Some(found_items) => {
                            if let Err(error) = bat_diffs(diff, Some(found_items), target_directory)
                            {
                                paint_error(error);
                            }
                        }
                        None => {
                            if let Err(error) = bat_diffs(diff, None, target_directory) {
                                paint_error(error);
                            }
                        }
                    }
                }
                Err(error) => paint_error(NomadError::GitError {
                    context: "Unable to get Git diff".into(),
                    source: error,
                }),
            },
            GitOptions::Restore(restore_options) => {
                if let Err(error) = modify_trees(
                    args,
                    &restore_options.item_labels,
                    nomad_style,
                    &repo,
                    TreeMode::RestoreWorkingDirectory,
                    target_directory,
                ) {
                    paint_error(NomadError::GitError {
                        context: "Unable to restore files!".to_string(),
                        source: error,
                    });
                }
            }
            GitOptions::Status(status_options) => {
                if let Some(branch_name) = get_repo_branch(&repo) {
                    println!(
                        "\nOn branch: {}",
                        Colour::Green.bold().paint(branch_name.to_string())
                    );

                    if let Err(error) = display_commits_ahead(&branch_name, &repo) {
                        paint_error(error);
                    }
                }

                match display_status_tree(status_options, nomad_style, &repo, target_directory) {
                    Ok(tree_items) => {
                        if let Some((tree, config)) = tree_items {
                            if let Some(export) = &status_options.export {
                                if let Err(error) =
                                    export_tree(config, ExportMode::GitStatus, export, tree)
                                {
                                    paint_error(error);
                                }
                            }
                        }
                    }
                    Err(error) => {
                        paint_error(error);
                    }
                }
            }
        }
    } else {
        paint_error(NomadError::Error(anyhow!("Cannot run Git commands here!")));
    }
}
