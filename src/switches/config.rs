//! Executing config subcommands.

use crate::{
    cli::config::ConfigOptions,
    config::preview::display_preview_tree,
    style::models::NomadStyle,
    utils::{open::open_files, paint::paint_error},
};

use ansi_term::Colour;

/// `match` the config subcommand and execute it.
pub fn run_config(
    config_options: &ConfigOptions,
    config_path: Option<String>,
    nomad_style: &NomadStyle,
) {
    match config_options {
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
        ConfigOptions::Preview => {
            if let Err(error) = display_preview_tree(nomad_style) {
                paint_error(error);
            }
        }
    }
}
