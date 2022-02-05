//! `nomad` - The `tree` command, but better.

mod cli;
mod git;
mod models;
mod traverse;
mod ui;
mod utils;

use ansi_term::Colour;
use tokio;

use std::io::{Error, ErrorKind, Result};

use cli::{Git, GitOptions};
use git::{
    commands::{commit_changes, stage_files},
    utils::{get_last_commit, get_repo, get_repo_branch},
};
use ui::{spawn_terminal, utils::convert_tree};
use utils::{
    icons::{get_icons_by_extension, get_icons_by_name},
    open::get_file,
    paths::get_current_directory,
};

/// Run `nomad`.
#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::get_args();

    // Set the target directory based on whether a directory was passed in.
    let target_directory = if let Some(ref directory) = args.directory {
        directory.clone()
    } else {
        get_current_directory().unwrap_or("?".to_string())
    };

    ///////////////////// TODO: MAKE THIS A LAZY STATIC?
    let extension_icon_map = get_icons_by_extension();
    let name_icon_map = get_icons_by_name();
    ///////////////////// TODO: MAKE THIS A LAZY STATIC?

    let repository = get_repo(&target_directory);

    if args.interactive {
        // TODO: RESERVE FOR NOMAD V0.1.1?

        let mut walker = traverse::build_walker(&args, &target_directory)?;
        let tree_items =
            traverse::walk_directory(&args, &extension_icon_map, &name_icon_map, &mut walker)?;
        if let Some((tree, config)) = tree_items {
            let tree_parts = convert_tree(config, tree);
            println!("{:?}", tree_parts);
        }

        let _ = spawn_terminal(&args, &target_directory, &mut walker)
            .await
            .map_err(|error| format!("UI ERROR: {error}"));
    } else if let Some(target) = args.open {
        let target_file = get_file(target)?;
        utils::open::open_file(target_file)?;
    } else if let Some(target) = args.bat {
        let target_file = get_file(target)?;
        utils::bat::run_bat(target_file)?;
    } else if let Some(sub_command) = args.git {
        if let Some(repo) = repository {
            if let Some(branch_name) = get_repo_branch(&repo) {
                println!(
                    "\nOn branch: {}\n",
                    Colour::Green.bold().paint(format!("{branch_name}"))
                );
            }

            match sub_command {
                Git::Git(git_command) => match git_command {
                    GitOptions::Add { file_numbers } => {
                        if let Err(error) = stage_files(file_numbers, &repo) {
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("Unable to stage files! {error}"),
                            ));
                        }
                    }
                    GitOptions::Commit { message } => {
                        if let Ok(parent_commit) = get_last_commit(&repo) {
                            if let Ok(staged_tree) = repo.find_tree(parent_commit.id()) {
                                if let Err(error) = commit_changes(message, &repo, staged_tree) {
                                    return Err(Error::new(
                                        ErrorKind::Other,
                                        format!("Unable to commit staged items! {error}"),
                                    ));
                                }
                            } else {
                                return Err(Error::new(
                                    ErrorKind::Other,
                                    format!("Unable to retrieve staged changes from this Git repository!"),
                                ));
                            }
                        } else {
                            return Err(Error::new(
                                ErrorKind::Other,
                                "Unable to retrieve the most recent commit from this Git repository!",
                            ));
                        }
                    }
                    GitOptions::Diff { file_number } => {
                        let target_file = get_file(file_number.to_string())?;
                        utils::bat::run_bat(target_file)?;
                    }
                    GitOptions::Status => {
                        // TODO: CREATE A TREE THAT ONLY CONTAINS ITEMS IN THE WORKING
                        // DIRECTORY/INDEX.
                    }
                },
            }
        } else {
            println!(
                "\n{}\n",
                Colour::Red
                    .bold()
                    .paint("Cannot run Git commands in this directory. Not a Git repository!")
            );
        }
    } else {
        let mut walker = traverse::build_walker(&args, &target_directory)?;
        let tree_items =
            traverse::walk_directory(&args, &extension_icon_map, &name_icon_map, &mut walker)?;

        if let (Some(file_name), Some((tree, config))) = (args.export, tree_items) {
            utils::export::export_tree(config, file_name, tree)?;
        }
    }

    Ok(())
}
