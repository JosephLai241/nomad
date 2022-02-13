//! Miscellaneous utilities for dealing with file paths.

use anyhow::{Context, Result};
use ignore::DirEntry;

use std::{env, ffi::OsStr, path::PathBuf};

use crate::errors::NomadError;

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

/// Get the filename for a `DirEntry`.
pub fn get_filename(item: &DirEntry) -> String {
    item.path()
        .file_name()
        .unwrap_or(OsStr::new("?"))
        .to_str()
        .unwrap_or("?")
        .to_string()
}
