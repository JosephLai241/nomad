//! Miscellaneous utilities for dealing with file paths.

use ignore::DirEntry;

use std::{
    env,
    ffi::OsStr,
    io::{Error, ErrorKind},
    path::PathBuf,
};

/// Get the current directory.
pub fn get_current_directory() -> Result<String, Error> {
    Ok(env::current_dir()?
        .into_os_string()
        .into_string()
        .expect("Could not get the current directory!")
        .clone())
}

/// Get the absolute file path based for the target_string.
pub fn canonicalize_path(target: &str) -> Result<String, Error> {
    PathBuf::from(target)
        .canonicalize()?
        .into_os_string()
        .into_string()
        .map_or(
            Err(Error::new(ErrorKind::Other, "Could not canonicalize path!")),
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
