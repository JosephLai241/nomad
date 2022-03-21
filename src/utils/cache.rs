//! Cache utilities for `nomad`.

use std::{
    fs::{create_dir_all, File},
    io::Write,
};

use anyhow::Result;
use directories::ProjectDirs;
use serde_json::Value;

use crate::errors::NomadError;

/// Return a JSON `File` object in write/overwrite or read-only mode.
pub fn get_json_file(read_only: bool) -> Result<File, NomadError> {
    match ProjectDirs::from("", "", "nomad") {
        Some(project_directory) => {
            let items_json = project_directory.cache_dir().join("items.json");

            if !items_json.exists() {
                match &items_json.parent() {
                    Some(parent) => create_dir_all(parent)?,
                    None => {
                        return Err(NomadError::PathError(
                            "Could not get the path to nomad's application directory!".to_string(),
                        ))
                    }
                }
            }

            let file = match read_only {
                true => File::open(items_json)?,
                false => File::create(items_json)?,
            };

            Ok(file)
        }
        None => Err(NomadError::ApplicationError),
    }
}

/// Write a JSON string to `items.json`.
pub fn write_to_json(json_file: &mut File, values: Value) -> Result<(), NomadError> {
    json_file.write_all(serde_json::to_string(&values)?.as_bytes())?;

    Ok(())
}
