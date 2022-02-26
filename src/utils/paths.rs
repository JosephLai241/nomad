//! Miscellaneous utilities for dealing with file paths.

use crate::errors::NomadError;

use ansi_term::Colour;
use anyhow::{Context, Result};
use ignore::DirEntry;
use regex::Match;

use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
};

/// Get the current directory.
pub fn get_current_directory() -> Result<String, NomadError> {
    Ok(env::current_dir()
        .with_context(|| "Could not get the current directory!")?
        .into_os_string()
        .into_string()
        .expect("Could not get the current directory!")
        .clone())
}

/// Get the absolute file path based for the target_string.
pub fn canonicalize_path(target: &str) -> Result<String, NomadError> {
    PathBuf::from(target)
        .canonicalize()
        .with_context(|| format!("\"{target}\" is not a directory!"))?
        .into_os_string()
        .into_string()
        .map_or(
            Err(NomadError::PathError(format!(
                "Could not canonicalize path to {target}"
            ))),
            |path| Ok(path),
        )
}

/// Get the filename for a `Path`.
pub fn get_filename(item: &Path) -> String {
    item.file_name()
        .unwrap_or(OsStr::new("?"))
        .to_str()
        .unwrap_or("?")
        .to_string()
}

/// Stylize a matched path based on the target regex pattern.
pub fn format_regex_match(entry: &DirEntry, matched: Match) -> String {
    let mut item_name = entry
        .path()
        .file_name()
        .unwrap_or(OsStr::new("?"))
        .to_str()
        .unwrap_or("?")
        .to_string();
    let matched_section = Colour::Red
        .bold()
        .paint(format!(
            "{}",
            item_name[matched.start()..matched.end()].to_string()
        ))
        .to_string();

    item_name.replace_range(matched.start()..matched.end(), &matched_section);

    entry
        .path()
        .parent()
        .unwrap_or(Path::new("?"))
        .join(Path::new(&item_name))
        .to_str()
        .unwrap_or("?")
        .to_string()
}
