//! Exposing functionality to `tokei` - counts lines of code and other file metadata.

pub mod format;
pub mod utils;

use std::path::PathBuf;

use ansi_term::Colour;
use tokei::{Config, Language, Languages};

use self::{format::tree_stats_from_report, utils::get_file_report};

/// Get the `Language` struct for a directory.
pub fn loc_in_dir(target_directory: &str) -> Language {
    let mut languages = Languages::new();
    // FUTURE: Add a table in `nomad.toml` called `[tokei]` to set the `Config`
    //         and ignored paths.
    languages.get_statistics(&[target_directory], &[], &Config::default());

    languages.total()
}

/// Get the `CodeStats` for a single file from the `Language` struct.
pub fn loc_in_file(file_path: &str, tokei: &Language) -> Vec<String> {
    let report = get_file_report(&tokei.children, PathBuf::from(file_path));

    let mut formatted_stats = Vec::new();

    match tree_stats_from_report(report) {
        Some(stats) => {
            formatted_stats.push(stats.blanks);
            formatted_stats.push(stats.code);
            formatted_stats.push(stats.comments);
            formatted_stats.push(stats.lines);
        }
        None => formatted_stats.push(format!(
            "| {}",
            Colour::Red.bold().paint("No tokei data available")
        )),
    }

    formatted_stats
}
