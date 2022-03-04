//! Executing Git subcommands.

use crate::{
    cli::{git::GitOptions, Args},
    errors::NomadError,
    git::{
        blame::bat_blame,
        branch::display_branches,
        commit::commit_changes,
        diff::{bat_diffs, get_repo_diffs},
        push::push_commits,
        restore::{restore_files, RestoreMode},
        stage::{stage_files, StageMode},
        status::{display_commits_ahead, display_status_tree},
        utils::{get_repo, get_repo_branch},
    },
    utils::{
        paint::paint_error,
        search::{indiscriminate_search, SearchMode},
    },
};

use ansi_term::Colour;
use anyhow::anyhow;

pub fn run_git(args: &Args, git_command: &GitOptions, target_directory: &str) {
    if let Some(repo) = get_repo(&target_directory) {
        match git_command {
            GitOptions::Add { item_labels } => {
                if let Err(error) =
                    stage_files(item_labels, &repo, StageMode::Stage, &target_directory)
                {
                    paint_error(NomadError::GitError {
                        context: "Unable to stage files".into(),
                        source: error,
                    });
                }
            }
            GitOptions::Blame(blame_options) => match blame_options.file_number.parse::<i32>() {
                Ok(file_number) => {
                    match indiscriminate_search(
                        &vec![file_number.to_string()],
                        Some(&repo),
                        SearchMode::Normal,
                        &target_directory,
                    ) {
                        Some(ref mut found_items) => match found_items.pop() {
                            Some(item) => {
                                if &blame_options.lines.len() > &2 {
                                    println!(
                                        "\n{}\n",
                                        Colour::Red
                                            .bold()
                                            .paint("Line range only takes two values - a lower and upper bound")
                                    );
                                } else {
                                    if let Err(error) = bat_blame(
                                        item,
                                        blame_options.lines.clone(),
                                        &repo,
                                        &target_directory,
                                    ) {
                                        paint_error(error);
                                    }
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
            GitOptions::Branch => {
                if let Err(error) = display_branches(&args, &repo) {
                    paint_error(error);
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
                        item_labels,
                        Some(&repo),
                        SearchMode::GitDiff,
                        &target_directory,
                    ) {
                        Some(found_items) => {
                            if let Err(error) =
                                bat_diffs(diff, Some(found_items), &target_directory)
                            {
                                paint_error(error);
                            }
                        }
                        None => {
                            if let Err(error) = bat_diffs(diff, None, &target_directory) {
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
            GitOptions::Push => {
                if let Err(error) = push_commits(&repo) {
                    paint_error(error);
                }
            }
            GitOptions::Restore(restore_options) => {
                // TODO: MAKE AN ENUM FOR THE STAGE_FILES() FUNCTION
                //       TO EITHER ADD OR REMOVE FROM THE INDEX?
                if let Err(error) = restore_files(
                    &restore_options.item_labels,
                    RestoreMode::Staged,
                    &repo,
                    &target_directory,
                ) {
                    paint_error(NomadError::GitError {
                        context: "Unable to restore files".into(),
                        source: error,
                    });
                }
            }
            GitOptions::Status => {
                if let Some(branch_name) = get_repo_branch(&repo) {
                    println!(
                        "\nOn branch: {}",
                        Colour::Green.bold().paint(format!("{branch_name}"))
                    );

                    if let Err(error) = display_commits_ahead(&branch_name, &repo) {
                        paint_error(error);
                    }
                }

                if let Err(error) = display_status_tree(&args, &repo, &target_directory) {
                    paint_error(error);
                }
            }
        }
    } else {
        paint_error(NomadError::Error(anyhow!("Cannot run Git commands here!")));
    }
}
