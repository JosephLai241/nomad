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
    blame::bat_blame,
    commit::commit_changes,
    diff::{bat_diffs, get_repo_diffs},
    stage::stage_files,
    status::display_status_tree,
    utils::{get_repo, get_repo_branch},
};
use releases::{build_release_list, check_for_update, update_self};
use traverse::utils::{build_types, build_walker, TypeOption};
use utils::{
    bat::run_bat,
    icons::{get_icons_by_extension, get_icons_by_name},
    open::open_files,
    paint::paint_error,
    paths::{canonicalize_path, get_current_directory},
    search::{indiscriminate_search, SearchMode},
    table::{TableView, TabledItems},
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
    /// Xterm 256 color codes (excludes grayscale colors).
    ///
    /// Corresponds with the first three color tabledefaults here:
    ///     https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg
    static ref XTERM_COLORS: Vec<u8> = vec![
        016, 017, 018, 019, 020, 021, 022, 023, 024, 025, 026, 027, 028, 029, 030,
        031, 032, 033, 034, 035, 036, 037, 038, 039, 040, 041, 042, 043, 044, 045,
        046, 047, 048, 049, 050, 051, 082, 083, 084, 085, 086, 087, 076, 077, 078,
        079, 080, 081, 070, 071, 072, 073, 074, 075, 064, 065, 066, 067, 068, 069,
        058, 059, 060, 061, 062, 063, 052, 053, 054, 055, 056, 057, 093, 092, 091,
        010, 098, 088, 099, 098, 097, 096, 095, 094, 105, 104, 103, 102, 101, 100,
        111, 110, 109, 108, 107, 106, 117, 116, 115, 114, 113, 112, 123, 122, 121,
        120, 119, 118, 159, 158, 157, 156, 155, 154, 153, 152, 151, 150, 149, 148,
        147, 146, 145, 144, 143, 142, 141, 140, 139, 138, 137, 136, 135, 134, 133,
        132, 131, 130, 129, 128, 127, 126, 125, 124, 160, 161, 162, 163, 164, 165,
        166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180,
        181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195,
        226, 227, 228, 229, 230, 231, 220, 221, 222, 223, 224, 225, 214, 215, 216,
        217, 218, 219, 208, 209, 210, 211, 212, 213, 202, 203, 204, 205, 206, 207,
        196, 197, 198, 199, 200, 201
    ];
}

/// Run `nomad`.
fn main() -> Result<(), NomadError> {
    //check_for_update()?;

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
                    match indiscriminate_search(
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
                    match indiscriminate_search(
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
                        FileTypeOptions::Match { filetypes, globs } => {
                            match build_types(filetypes, globs, type_matcher, TypeOption::Match) {
                                Ok(types) => {
                                    match build_walker(&args, &target_directory, Some(types)) {
                                        Ok(mut walker) => {
                                            match traverse::walk_directory(
                                                &args,
                                                &target_directory,
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
                        FileTypeOptions::Negate { filetypes, globs } => {
                            match build_types(filetypes, globs, type_matcher, TypeOption::Negate) {
                                Ok(types) => {
                                    match build_walker(&args, &target_directory, Some(types)) {
                                        Ok(mut walker) => {
                                            match traverse::walk_directory(
                                                &args,
                                                &target_directory,
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
                            GitOptions::Blame(blame_options) => {
                                match blame_options.file_number.parse::<i32>() {
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
                                                    Colour::Red
                                                        .bold()
                                                        .paint("Could not find a file to blame!")
                                                ),
                                            },
                                            None => println!(
                                                "\n{}\n",
                                                Colour::Red
                                                    .bold()
                                                    .paint("Could not find a file to blame!")
                                            ),
                                        }
                                    }
                                    Err(_) => paint_error(NomadError::GitBlameError),
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
                            GitOptions::Restore { item_labels } => {
                                // TODO: MAKE AN ENUM FOR THE STAGE_FILES() FUNCTION
                                //       TO EITHER ADD OR REMOVE FROM THE INDEX?
                            }
                            GitOptions::Status => {
                                if let Some(branch_name) = get_repo_branch(&repo) {
                                    println!(
                                        "\nOn branch: {}",
                                        Colour::Green.bold().paint(format!("{branch_name}"))
                                    );
                                }

                                if let Err(error) =
                                    display_status_tree(&args, &repo, &target_directory)
                                {
                                    paint_error(error);
                                }
                            }
                        }
                    } else {
                        paint_error(NomadError::Error(anyhow!("Cannot run Git commands here!")));
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
                SubCommands::Upgrade => {
                    if let Err(error) = update_self() {
                        paint_error(error);
                    }
                }
            }
        } else {
            // Run `nomad` in normal mode.
            match build_walker(&args, &target_directory, None) {
                Ok(mut walker) => {
                    match traverse::walk_directory(&args, &target_directory, &mut walker) {
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
