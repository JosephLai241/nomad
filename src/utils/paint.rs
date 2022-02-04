//! Apply colors to a directory's contents.

use ansi_term::Colour;
use ignore::DirEntry;

use std::{fs::read_link, path::PathBuf};

use super::paths::get_filename;

/// Paint a directory.
pub fn paint_directory(item: &DirEntry) -> String {
    Colour::Blue
        .bold()
        .paint(format!("{}", get_filename(item)))
        .to_string()
}

/// Paint a symlinked directory.
pub fn paint_symlink_directory(item: &DirEntry) -> String {
    let filename = get_filename(item);

    let points_to = read_link(item.path()).map_or("?".to_string(), |pathbuf_path| {
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
