//! Parse `nomad.toml`, the configuration file.

use crate::errors::NomadError;

use super::models::NomadConfig;

use anyhow::Result;
use directories::ProjectDirs;
use toml::from_slice;

use std::fs::{create_dir_all, read, write};

/// Parse the settings that are specified in `nomad.toml` into the `NomadConfig` struct.
pub fn parse_config() -> Result<(NomadConfig, Option<String>), NomadError> {
    if let Some(ref project_directory) = ProjectDirs::from("", "", "nomad") {
        let config_path = project_directory.config_dir().join("nomad.toml");

        if !config_path.exists() {
            match &config_path.parent() {
                Some(parent) => create_dir_all(parent)?,
                None => {
                    return Err(NomadError::PathError(
                        "Could not get the path to nomad's application directory!".to_string(),
                    ))
                }
            }

            write(&config_path, include_bytes!("../../nomad.toml"))?;
        }

        let config_contents = read(&config_path)?;

        Ok((
            from_slice(&config_contents)?,
            config_path
                .to_str()
                .map_or(None, |path| Some(path.to_string())),
        ))
    } else {
        Err(NomadError::ApplicationError)
    }
}
