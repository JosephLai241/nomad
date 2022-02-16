//! Creating temporary directories and files to store `nomad` metadata.

use std::{
    env::temp_dir,
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Result;
use serde_json::Value;

use crate::errors::NomadError;

/// Directory name within the system's temporary directory to store `nomad` metadata.
static DIRECTORY: &str = "nomad";
/// The JSON file that stores labeled directories.
static LABELED_FILE: &str = "labels.json";
/// The JSON file that stores numbered directory contents.
static NUMBERED_FILE: &str = "items.json";

/// Get the path to the temporary directory to store `nomad` metadata.
pub fn get_temp_dir_path() -> PathBuf {
    let mut temp_dir = temp_dir();
    temp_dir.push(DIRECTORY);

    temp_dir
}

/// Create a temporary directory to store `nomad` metadata.
pub fn create_temp_dir() -> Result<(), NomadError> {
    create_dir_all(get_temp_dir_path())?;
    Ok(())
}

/// Contains options for JSON file access.
#[derive(Copy, Clone)]
pub enum JSONTarget {
    /// Get the JSON file that contains numbered directory contents.
    Contents,
    /// Get the JSON file that contains labeled directories.
    Directories,
}

/// Return a JSON `File` object in write/overwrite or read-only mode.
pub fn get_json_file(json_target: JSONTarget, read_only: bool) -> Result<File, NomadError> {
    let mut items_json = get_temp_dir_path();
    items_json.push(match json_target {
        JSONTarget::Contents => NUMBERED_FILE,
        JSONTarget::Directories => LABELED_FILE,
    });

    let file = if read_only {
        File::open(items_json)?
    } else {
        File::create(items_json)?
    };

    Ok(file)
}

/// Write JSON string to `items.json`.
pub fn write_to_json(json_file: &mut File, values: Value) -> Result<(), NomadError> {
    json_file.write_all(serde_json::to_string(&values)?.as_bytes())?;

    Ok(())
}
