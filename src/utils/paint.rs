//! Apply colors to a directory's contents.

use crate::errors::NomadError;

use ansi_term::Colour;

use std::{
    fs::read_link,
    path::{Path, PathBuf},
};

/// Format and display a `NomadError`.
pub fn paint_error(error: NomadError) {
    println!("\n{}\n", Colour::Red.bold().paint(error.to_string()));
}

/// Get the painted symlinked item.
pub fn paint_symlink(item: &Path) -> String {
    let points_to = read_link(item).map_or("?".to_string(), |pathbuf_path| {
        pathbuf_path
            .canonicalize()
            .unwrap_or_else(|_| PathBuf::from("?"))
            .into_os_string()
            .into_string()
            .map_or("?".to_string(), |path_string| path_string)
    });

    Colour::Yellow
        .bold()
        .paint(format!("⇒ {points_to}"))
        .to_string()
}
