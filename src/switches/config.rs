//! Executing config subcommands.

use crate::{
    cli::config::ConfigOptions,
    style::models::NomadStyle,
    traverse::format::highlight_matched,
    utils::{
        open::open_files,
        paint::paint_error,
        table::{TableView, TabledItems},
    },
};

use ansi_term::Colour;

/// `match` the config subcommand and execute it.
pub fn run_config(
    config_options: &ConfigOptions,
    config_path: Option<String>,
    nomad_style: &NomadStyle,
) {
    match config_options {
        ConfigOptions::Display => {
            TabledItems::new(
                vec![
                    (
                        "Conflicted",
                        &nomad_style
                            .git
                            .conflicted_color
                            .paint(&nomad_style.git.conflicted_marker)
                            .to_string(),
                    ),
                    (
                        "Deleted",
                        &nomad_style
                            .git
                            .deleted_color
                            .paint(&nomad_style.git.deleted_marker)
                            .to_string(),
                    ),
                    (
                        "Modified",
                        &nomad_style
                            .git
                            .modified_color
                            .paint(&nomad_style.git.modified_marker)
                            .to_string(),
                    ),
                    (
                        "Renamed",
                        &nomad_style
                            .git
                            .renamed_color
                            .paint(&nomad_style.git.renamed_marker)
                            .to_string(),
                    ),
                    (
                        "Untracked",
                        &nomad_style
                            .git
                            .untracked_color
                            .paint(&nomad_style.git.untracked_marker)
                            .to_string(),
                    ),
                ],
                vec!["Git Status".to_string(), "Marker".to_string()],
                120,
                None,
            )
            .display_table();

            TabledItems::new(
                vec![
                    (
                        "Staged Added",
                        &nomad_style
                            .git
                            .staged_added_color
                            .paint(&nomad_style.git.staged_added_marker)
                            .to_string(),
                        &nomad_style
                            .git
                            .staged_added_color
                            .paint("nomad.rs")
                            .to_string(),
                    ),
                    (
                        "Staged Deleted",
                        &nomad_style
                            .git
                            .staged_deleted_color
                            .paint(&nomad_style.git.staged_deleted_marker)
                            .to_string(),
                        &nomad_style
                            .git
                            .staged_deleted_color
                            .strikethrough()
                            .paint("nomad.rs")
                            .to_string(),
                    ),
                    (
                        "Staged Modified",
                        &nomad_style
                            .git
                            .staged_modified_color
                            .paint(&nomad_style.git.staged_modified_marker)
                            .to_string(),
                        &nomad_style
                            .git
                            .staged_modified_color
                            .paint("nomad.rs")
                            .to_string(),
                    ),
                    (
                        "Staged Renamed",
                        &nomad_style
                            .git
                            .staged_renamed_color
                            .paint(&nomad_style.git.staged_renamed_marker)
                            .to_string(),
                        &nomad_style
                            .git
                            .staged_renamed_color
                            .paint("nomad.rs")
                            .to_string(),
                    ),
                ],
                vec![
                    "Git Status".to_string(),
                    "Marker".to_string(),
                    "Filename Style".to_string(),
                ],
                180,
                None,
            )
            .display_table();

            TabledItems::new(
                vec![highlight_matched(
                    "nomad.rs".to_string(),
                    &nomad_style,
                    (2, 5),
                )],
                vec!["Regex Match".to_string()],
                180,
                None,
            )
            .display_table();
        }
        ConfigOptions::Edit => {
            if let Some(config_path) = config_path {
                if let Err(error) = open_files(vec![config_path]) {
                    paint_error(error)
                }
            } else {
                println!(
                    "\n{}\n",
                    Colour::Red
                        .bold()
                        .paint("Could not get the path to the configuration file!")
                );
            }
        }
    }
}
