//! Apply colors to a directory's contents.

use ansi_term::Colour;

use std::{
    fs::read_link,
    path::{Path, PathBuf},
};

use crate::errors::NomadError;

use super::paths::get_filename;

/// Format and display a `NomadError`.
pub fn paint_error(error: NomadError) {
    println!("\n{}\n", Colour::Red.bold().paint(error.to_string()));
}

/// Paint a directory.
pub fn paint_directory(item: &Path) -> String {
    Colour::Blue
        .bold()
        .paint(format!("{}", get_filename(item)))
        .to_string()
}

/// Paint a symlinked item.
pub fn paint_symlink(item: &Path) -> String {
    let filename = get_filename(item);

    let points_to = read_link(item).map_or("?".to_string(), |pathbuf_path| {
        pathbuf_path
            .canonicalize()
            .unwrap_or(PathBuf::from("?"))
            .into_os_string()
            .into_string()
            .map_or("?".to_string(), |path_string| path_string)
    });

    Colour::Yellow
        .bold()
        .paint(format!("{filename} ⇒ {points_to}"))
        .to_string()
}
