//! `nomad` - The next gen `tree` command.

mod cli;
mod errors;
mod git;
mod models;
mod traverse;
mod utils;

use std::collections::HashMap;

use ansi_term::Colour;
use anyhow::{anyhow, Result};
use errors::NomadError;
use ignore::types::TypesBuilder;
use lazy_static::lazy_static;

use cli::{FileTypeOptions, GitOptions, SubCommands};
use git::{
    commands::{commit_changes, display_status_tree, stage_files},
    utils::{get_repo, get_repo_branch},
};
use traverse::utils::{build_types, build_walker, TypeOption};
use utils::{
    icons::{get_icons_by_extension, get_icons_by_name},
    open::get_file,
    paint::paint_error,
    paths::{canonicalize_path, get_current_directory},
    table::display_filetype_definitions,
};

lazy_static! {
    /// A HashMap containing file extensions with a corresponding icon.
    static ref EXTENSION_ICON_MAP: HashMap<&'static str, &'static str> = get_icons_by_extension();
    /// A HashMap containing file names with a corresponding icon.
    static ref NAME_ICON_MAP: HashMap<&'static str, &'static str> = get_icons_by_name();
}

/// Run `nomad`.
fn main() -> Result<(), NomadError> {
    let args = cli::get_args();

    let target_directory = if let Some(ref directory) = args.directory {
        canonicalize_path(directory).map_or_else(
            |error| {
                paint_error(error);
                None
            },
            |path| Some(path),
        )
    } else {
        get_current_directory().map_or_else(
            |error| {
                paint_error(error);
                None
            },
            |path| Some(path),
        )
    };

    if let Some(target_directory) = target_directory {
        if let Some(sub_command) = &args.sub_commands {
            match sub_command {
                SubCommands::Bat { file_number } => {
                    let target_file = get_file(file_number.to_string())?;
                    if let Err(error) = utils::bat::run_bat(target_file) {
                        paint_error(error);
                    }
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
                SubCommands::FileType(filetype_option) => {
                    let mut type_matcher = TypesBuilder::new();
                    type_matcher.add_defaults();

                    match filetype_option {
                        FileTypeOptions::Match { filetypes } => {
                            let types = build_types(filetypes, type_matcher, TypeOption::Match)?;
                            let mut walker = build_walker(&args, &target_directory, Some(types))?;
                            let _ = traverse::walk_directory(
                                &args,
                                &EXTENSION_ICON_MAP,
                                &NAME_ICON_MAP,
                                &mut walker,
                            )?;
                        }
                        FileTypeOptions::Negate { filetypes } => {
                            let types = build_types(filetypes, type_matcher, TypeOption::Negate)?;
                            let mut walker = build_walker(&args, &target_directory, Some(types))?;
                            let _ = traverse::walk_directory(
                                &args,
                                &EXTENSION_ICON_MAP,
                                &NAME_ICON_MAP,
                                &mut walker,
                            )?;
                        }
                        FileTypeOptions::Options { filetype } => {
                            display_filetype_definitions(
                                type_matcher.definitions(),
                                filetype.to_owned(),
                            );
                        }
                    }
                }
                SubCommands::Git(git_command) => {
                    if let Some(repo) = get_repo(&target_directory) {
                        match git_command {
                            GitOptions::Add { file_numbers } => {
                                if let Err(error) =
                                    stage_files(file_numbers, &repo, &target_directory)
                                {
                                    paint_error(NomadError::GitError {
                                        context: "Unable to stage files".into(),
                                        source: error,
                                    });
                                }
                            }
                            GitOptions::Commit { message } => {
                                if let Err(error) = commit_changes(message, &repo) {
                                    paint_error(error);
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

                                let mut walker = build_walker(&args, &target_directory, None)?;
                                display_status_tree(
                                    &args,
                                    &EXTENSION_ICON_MAP,
                                    &NAME_ICON_MAP,
                                    &repo,
                                    &target_directory,
                                    &mut walker,
                                )?;
                            }
                        }
                    } else {
                        paint_error(NomadError::Error(anyhow!("Cannot run Git commands here!\nThe directory does not contain a Git repository.")));
                    }
                }
            }
        } else {
            // Run `nomad` in normal mode.
            let mut walker = build_walker(&args, &target_directory, None)?;
            let (tree, config) =
                traverse::walk_directory(&args, &EXTENSION_ICON_MAP, &NAME_ICON_MAP, &mut walker)?;

            if let Some(filename) = args.export {
                utils::export::export_tree(config, &filename, tree)?;
            }
        }
    }

    Ok(())
}
