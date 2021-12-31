//! Creating temporary directories and files to store `nomad` metadata.

use std::{
    env::temp_dir,
    fs::{create_dir_all, File},
    io::{BufRead, BufReader, Error, Write},
    path::PathBuf,
};

use serde_json::Value;

/// The name of the file that will store the name of the most recently traversed directory.
static CURRENT: &str = "current";
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

/// Create a file in the temporary directory to store the most recently traversed directory.
pub fn set_current_dir(directory_name: String) -> Result<(), Error> {
    let mut current_dir_file = get_temp_dir_path();
    current_dir_file.push(CURRENT);

    let mut file = File::create(current_dir_file)?;
    file.write_all(directory_name.as_bytes())?;

    Ok(())
}

/// Determine if `items.json` needs to be overwritten (a new directory was traversed).
pub fn should_overwrite(target_directory: String) -> Result<bool, Error> {
    let mut existing_dir_file = get_temp_dir_path();
    existing_dir_file.push(CURRENT);

    File::open(existing_dir_file).map_or_else(
        |_| {
            set_current_dir(target_directory.clone())?;
            Ok(true)
        },
        |file| {
            let mut existing_directory = String::new();
            BufReader::new(file).read_line(&mut existing_directory)?;

            if existing_directory == target_directory {
                Ok(false)
            } else {
                Ok(true)
            }
        },
    )
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
