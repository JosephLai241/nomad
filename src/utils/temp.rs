//! Creating temporary directories and files to store `nomad` metadata.

use std::{
    env::temp_dir,
    fs::{create_dir_all, File},
    io::{Error, Write},
    path::PathBuf,
};

use serde_json::Value;

/// Directory name within the system's temporary directory to store `nomad` metadata.
static DIRECTORY: &str = "nomad";
/// The JSON file that stores directory `Item`s for quick access.
static NUMBERED_FILE: &str = "items.json";

/// Get the path to the temporary directory to store `nomad` metadata.
pub fn get_temp_dir_path() -> PathBuf {
    let mut temp_dir = temp_dir();
    temp_dir.push(DIRECTORY);

    temp_dir
}

/// Create a temporary directory to store `nomad` metadata.
pub fn create_temp_dir() -> Result<(), Error> {
    create_dir_all(get_temp_dir_path())?;
    Ok(())
}

/// Return the `items.json` `File` object in write/overwrite or read-only mode.
pub fn get_json_file(read_only: bool) -> Result<File, Error> {
    let mut items_json = get_temp_dir_path();
    items_json.push(NUMBERED_FILE);

    let file = if read_only {
        File::open(items_json)?
    } else {
        File::create(items_json)?
    };

    Ok(file)
}

/// Write JSON string to `items.json`.
pub fn write_to_json(json_file: &mut File, values: Value) -> Result<(), Error> {
    json_file.write_all(serde_json::to_string(&values)?.as_bytes())?;

    Ok(())
}
