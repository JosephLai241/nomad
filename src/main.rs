//! `nomad` - The `tree` command, but better.

mod cli;
mod git;
mod models;
mod traverse;
mod utils;

use ansi_term::Colour;
use tokio;

use std::io::{Error, ErrorKind, Result};

use cli::{GitOptions, SubCommands};
use git::{
    commands::{commit_changes, display_status_tree, stage_files},
    utils::{get_repo, get_repo_branch},
};
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

    if let Some(sub_command) = &args.sub_commands {
        match sub_command {
            SubCommands::Bat { file_number } => {
                let target_file = get_file(file_number.to_string())?;
                utils::bat::run_bat(target_file)?;
            }
            SubCommands::Cd { directory_label } => {
                println!("CHANGING DIRECTORY TO: {directory_label}");
                // TODO:
                //  Write to another JSON file(?) containing directory labels
                //  Refactor the function that pulls from the JSON files to take either
                //  JSON file?
                //  Return the filename and then call the function below.

                //set_current_dir(&Path::new(&directory_name))?;
            }
            SubCommands::Edit { file_number } => {
                let target_file = get_file(file_number.to_string())?;
                utils::open::open_file(target_file)?;
            }
            SubCommands::Export { filename } => {
                let mut walker = traverse::build_walker(&args, &target_directory)?;
                let (tree, config) = traverse::walk_directory(
                    &args,
                    &extension_icon_map,
                    &name_icon_map,
                    &mut walker,
                )?;

                utils::export::export_tree(config, filename, tree)?;
            }
            SubCommands::Git(git_command) => {
                if let Some(repo) = get_repo(&target_directory) {
                    match git_command {
                        GitOptions::Add { file_numbers } => {
                            if let Err(error) = stage_files(file_numbers, &repo) {
                                return Err(Error::new(
                                    ErrorKind::Other,
                                    format!("Unable to stage files! {error}"),
                                ));
                            }
                        }
                        GitOptions::Commit { message } => {
                            if let Err(error) = commit_changes(message, &repo) {
                                return Err(Error::new(
                                    ErrorKind::Other,
                                    format!("Unable to commit Git changes! {error}"),
                                ));
                            }
                        }
                        GitOptions::Diff { file_number } => {
                            let target_file = get_file(file_number.to_string())?;
                            utils::bat::run_bat(target_file)?;
                        }
                        GitOptions::Status => {
                            if let Some(branch_name) = get_repo_branch(&repo) {
                                println!(
                                    "\nOn branch: {}\n",
                                    Colour::Green.bold().paint(format!("{branch_name}"))
                                );
                            }

                            let mut walker = traverse::build_walker(&args, &target_directory)?;
                            display_status_tree(
                                &args,
                                &extension_icon_map,
                                &name_icon_map,
                                &repo,
                                &target_directory,
                                &mut walker,
                            );
                        }
                    }
                } else {
                    println!(
                        "\n{}\n",
                        Colour::Red.bold().paint(
                            "Cannot run Git commands here!\nThe directory does not contain a Git repository."
                        )
                    );
                }
            }
        }
    } else {
        // Run `nomad` in normal mode.
        let mut walker = traverse::build_walker(&args, &target_directory)?;
        let _ = traverse::walk_directory(&args, &extension_icon_map, &name_icon_map, &mut walker)?;
    }

    Ok(())
}
