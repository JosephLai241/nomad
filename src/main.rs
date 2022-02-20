//! `nomad` - The next gen `tree` command.

mod cli;
mod errors;
mod git;
mod models;
mod releases;
mod traverse;
mod utils;

use cli::{FileTypeOptions, GitOptions, ReleaseOptions, SubCommands};
use git::{
    commit::commit_changes,
    diff::{bat_diffs, get_repo_diffs},
    stage::stage_files,
    status::display_status_tree,
    utils::{get_repo, get_repo_branch, indiscriminate_file_search, SearchMode},
};
use releases::{build_release_list, update_self};
use traverse::{
    modes::TraversalMode,
    utils::{build_types, build_walker, TypeOption},
};
use utils::{
    bat::run_bat,
    icons::{get_icons_by_extension, get_icons_by_name},
    open::{get_file, open_files},
    paint::paint_error,
    paths::{canonicalize_path, get_current_directory},
    table::{TableView, TabledItems},
    temp::JSONTarget,
};

use ansi_term::Colour;
use anyhow::{anyhow, Result};
use errors::NomadError;
use ignore::types::TypesBuilder;
use lazy_static::lazy_static;

use std::collections::HashMap;

lazy_static! {
    /// The alphabet in `char`s.
    static ref ALPHABET: Vec<char> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ];
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
                SubCommands::Bat { item_labels } => {
                    match indiscriminate_file_search(
                        item_labels,
                        None,
                        SearchMode::Normal,
                        &target_directory,
                    ) {
                        Some(found_items) => {
                            if let Err(error) = run_bat(found_items) {
                                paint_error(error);
                            }
                        }
                        None => {}
                    }
                }
                SubCommands::Edit { item_labels } => {
                    match indiscriminate_file_search(
                        item_labels,
                        None,
                        SearchMode::Normal,
                        &target_directory,
                    ) {
                        Some(found_items) => {
                            if let Err(error) = open_files(found_items) {
                                paint_error(error);
                            }
                        }
                        None => {}
                    }
                }
                SubCommands::Filetype(filetype_option) => {
                    let mut type_matcher = TypesBuilder::new();
                    type_matcher.add_defaults();

                    match filetype_option {
                        FileTypeOptions::Match { filetypes } => {
                            match build_types(filetypes, type_matcher, TypeOption::Match) {
                                Ok(types) => {
                                    match build_walker(&args, &target_directory, Some(types)) {
                                        Ok(mut walker) => {
                                            match traverse::walk_directory(
                                                &args,
                                                &target_directory,
                                                TraversalMode::Filetype,
                                                &mut walker,
                                            ) {
                                                Ok((tree, config)) => {
                                                    if let Some(filename) = args.export {
                                                        if let Err(error) =
                                                            utils::export::export_tree(
                                                                config, &filename, tree,
                                                            )
                                                        {
                                                            paint_error(error);
                                                        }
                                                    }
                                                }
                                                Err(error) => paint_error(error),
                                            }
                                        }
                                        Err(error) => paint_error(error),
                                    }
                                }
                                Err(error) => paint_error(error),
                            }
                        }
                        FileTypeOptions::Negate { filetypes } => {
                            match build_types(filetypes, type_matcher, TypeOption::Negate) {
                                Ok(types) => {
                                    match build_walker(&args, &target_directory, Some(types)) {
                                        Ok(mut walker) => {
                                            match traverse::walk_directory(
                                                &args,
                                                &target_directory,
                                                TraversalMode::Filetype,
                                                &mut walker,
                                            ) {
                                                Ok((tree, config)) => {
                                                    if let Some(filename) = args.export {
                                                        if let Err(error) =
                                                            utils::export::export_tree(
                                                                config, &filename, tree,
                                                            )
                                                        {
                                                            paint_error(error);
                                                        }
                                                    }
                                                }
                                                Err(error) => paint_error(error),
                                            }
                                        }
                                        Err(error) => paint_error(error),
                                    }
                                }
                                Err(error) => paint_error(error),
                            }
                        }
                        FileTypeOptions::Options { filetype } => TabledItems::new(
                            type_matcher.definitions(),
                            vec!["Name".into(), "Globs".into()],
                            120,
                            filetype.to_owned(),
                        )
                        .display_table(),
                    }
                }
                SubCommands::Git(git_command) => {
                    if let Some(repo) = get_repo(&target_directory) {
                        match git_command {
                            GitOptions::Add { item_labels } => {
                                if let Err(error) =
                                    stage_files(item_labels, &repo, &target_directory)
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
                            GitOptions::Diff { item_labels } => match get_repo_diffs(&repo) {
                                Ok(diff) => {
                                    match indiscriminate_file_search(
                                        item_labels,
                                        Some(&repo),
                                        SearchMode::GitDiff,
                                        &target_directory,
                                    ) {
                                        Some(found_items) => {
                                            if let Err(error) = bat_diffs(
                                                diff,
                                                Some(found_items),
                                                &target_directory,
                                            ) {
                                                paint_error(error);
                                            }
                                        }
                                        None => {
                                            if let Err(error) =
                                                bat_diffs(diff, None, &target_directory)
                                            {
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
                            GitOptions::Status => {
                                if let Some(branch_name) = get_repo_branch(&repo) {
                                    println!(
                                        "\nOn branch: {}\n",
                                        Colour::Green.bold().paint(format!("{branch_name}"))
                                    );
                                }

                                match build_walker(&args, &target_directory, None) {
                                    Ok(mut walker) => {
                                        if let Err(error) = display_status_tree(
                                            &args,
                                            &repo,
                                            &target_directory,
                                            &mut walker,
                                        ) {
                                            paint_error(error);
                                        }
                                    }
                                    Err(error) => paint_error(error),
                                }
                            }
                        }
                    } else {
                        paint_error(NomadError::Error(anyhow!("Cannot run Git commands here!\nThe directory does not contain a Git repository.")));
                    }
                }
                SubCommands::Releases(release_option) => match release_option {
                    ReleaseOptions::All => match build_release_list() {
                        Ok(releases) => TabledItems::new(
                            releases,
                            vec![
                                "Name".into(),
                                "Version".into(),
                                "Release Date".into(),
                                "Description".into(),
                                "Assets".into(),
                            ],
                            180,
                            None,
                        )
                        .display_table(),
                        Err(error) => paint_error(error),
                    },
                    ReleaseOptions::Info { release_version } => match build_release_list() {
                        Ok(releases) => TabledItems::new(
                            releases,
                            vec![
                                "Name".into(),
                                "Version".into(),
                                "Release Date".into(),
                                "Description".into(),
                                "Assets".into(),
                            ],
                            180,
                            release_version.to_owned(),
                        )
                        .display_table(),
                        Err(error) => paint_error(error),
                    },
                },
                SubCommands::Update => {
                    if let Err(error) = update_self() {
                        paint_error(error);
                    }
                }
            }
        } else {
            // Run `nomad` in normal mode.
            match build_walker(&args, &target_directory, None) {
                Ok(mut walker) => {
                    let traversal_mode = if args.pattern.is_some() {
                        TraversalMode::Regex
                    } else {
                        TraversalMode::Normal
                    };

                    match traverse::walk_directory(
                        &args,
                        &target_directory,
                        traversal_mode,
                        &mut walker,
                    ) {
                        Ok((tree, config)) => {
                            if let Some(filename) = args.export {
                                if let Err(error) =
                                    utils::export::export_tree(config, &filename, tree)
                                {
                                    paint_error(error);
                                }
                            }
                        }
                        Err(error) => paint_error(error),
                    }
                }
                Err(error) => paint_error(error),
            }
        }
    }

    Ok(())
}
