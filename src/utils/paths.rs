//! Miscellaneous utilities for dealing with file paths.

use ignore::DirEntry;

use std::ffi::OsStr;

/// Get the filename for a `DirEntry`.
pub fn get_filename(item: &DirEntry) -> String {
    item.path()
        .file_name()
        .unwrap_or(OsStr::new("?"))
        .to_str()
        .unwrap_or("?")
        .to_string()
}
