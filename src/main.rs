//! `nomad` - The next gen [`tree`] command.
//!
//! [`tree`]: https://linux.die.net/man/1/tree

// NOTE - Clippy allowed lints are subject to change in the future.
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::format_in_format_args)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::wildcard_in_or_patterns)]
#![allow(clippy::zero_prefixed_literal)]

mod cli;
mod config;
mod errors;
mod git;
mod loc;
mod models;
mod releases;
mod style;
mod switches;
mod traverse;
mod ui;
mod utils;

use cli::{get_args, SubCommands};
use config::toml::parse_config;
use loc::run_tokei;
use releases::{check_for_update, update_self};
use style::settings::process_settings;
use switches::{config::run_config, filetype::run_filetypes, git::run_git, release::run_releases};
use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};
use traverse::{modes::NomadMode, utils::build_walker, walk_directory};
use ui::{enter_rootless_mode, ExitMode};
use utils::{
    bat::run_bat,
    export::{export_tree, ExportMode},
    icons::{get_icons_by_extension, get_icons_by_name},
    open::open_files,
    paint::{paint_error, show_banner},
    paths::{canonicalize_path, get_current_directory},
    search::{indiscriminate_search, SearchMode},
};

use anyhow::Result;
use errors::NomadError;
use lazy_static::lazy_static;

use std::collections::HashMap;

lazy_static! {
    /// The alphabet in `Vec<char>`.
    static ref ALPHABET: Vec<char> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ];
    /// A `HashMap` containing file extensions with a corresponding icon.
    static ref EXTENSION_ICON_MAP: HashMap<&'static str, &'static str> = get_icons_by_extension();
    /// A `HashMap` containing file names with a corresponding icon.
    static ref NAME_ICON_MAP: HashMap<&'static str, &'static str> = get_icons_by_name();
    /// Xterm 256 color codes (excludes grayscale colors).
    ///
    /// Corresponds with the first three [color tables here].
    ///
    /// [color tables here]: https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg
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
    /// Syntect `SyntaxSet` for syntax highlighting.
    static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_nonewlines();
    /// Syntect `ThemeSet` for syntax highlighting.
    static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
}

/// Run `nomad`.
fn main() -> Result<(), NomadError> {
    let mut args = get_args();

    if args.banner {
        show_banner();
    } else {
        let (nomad_config, config_path) = parse_config()?;
        let nomad_style = process_settings(nomad_config);

        let target_directory = if let Some(ref directory) = args.directory {
            canonicalize_path(directory).map_or_else(
                |error| {
                    paint_error(error);
                    None
                },
                Some,
            )
        } else {
            get_current_directory().map_or_else(
                |error| {
                    paint_error(error);
                    None
                },
                Some,
            )
        };

        if let Some(target_directory) = target_directory {
            if let Some(sub_command) = &args.sub_commands {
                match sub_command {
                    SubCommands::Bat { item_labels } => {
                        if let Some(found_items) = indiscriminate_search(
                            &args,
                            item_labels,
                            &nomad_style,
                            None,
                            SearchMode::Normal,
                            &target_directory,
                        ) {
                            if let Err(error) = run_bat(found_items) {
                                paint_error(error);
                            }
                        }
                    }
                    SubCommands::Config(config_options) => {
                        run_config(config_options, config_path, &nomad_style);
                    }
                    SubCommands::Edit { item_labels } => {
                        if let Some(found_items) = indiscriminate_search(
                            &args,
                            item_labels,
                            &nomad_style,
                            None,
                            SearchMode::Normal,
                            &target_directory,
                        ) {
                            if let Err(error) = open_files(found_items) {
                                paint_error(error);
                            }
                        }
                    }
                    SubCommands::Ft(filetype_option) => {
                        run_filetypes(filetype_option, &nomad_style, &target_directory);
                    }
                    SubCommands::Git(git_command) => {
                        run_git(&args, git_command, &nomad_style, &target_directory);
                    }
                    SubCommands::Rootless => {
                        // ANSI escape codes do not correctly render in the alternate screen,
                        // which is why `--no-colors` has to be enabled.
                        args.global.style.no_colors = true;

                        match enter_rootless_mode(&mut args.global, &nomad_style, &target_directory)
                        {
                            Ok(exit_mode) => {
                                if let ExitMode::Edit(found_items) = exit_mode {
                                    if let Err(error) = open_files(found_items) {
                                        paint_error(error);
                                    }
                                }
                            }
                            Err(error) => {
                                paint_error(error);
                            }
                        }
                    }
                    SubCommands::Releases(release_option) => {
                        run_releases(release_option);
                    }
                    SubCommands::Tokei => {
                        run_tokei(&target_directory);
                    }
                    SubCommands::Upgrade(upgrade_options) => {
                        if upgrade_options.check {
                            if let Err(error) = check_for_update() {
                                paint_error(error);
                            }
                        } else if let Err(error) = update_self() {
                            paint_error(error);
                        }
                    }
                }
            } else {
                // Run `nomad` in normal mode.
                match build_walker(&args.global, &target_directory, None) {
                    Ok(mut walker) => {
                        match walk_directory(
                            &args.global,
                            NomadMode::Normal,
                            &nomad_style,
                            &target_directory,
                            &mut walker,
                        ) {
                            Ok((tree, config, _)) => {
                                if let Some(export) = args.global.export {
                                    if let Err(error) =
                                        export_tree(config, ExportMode::Normal, &export, tree)
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
    }

    Ok(())
}
