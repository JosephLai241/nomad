//! Open a file using the client's system's `$EDITOR`.

use super::temp::{get_json_file, JSONTarget};
use crate::{errors::NomadError, models::Contents};

use anyhow::{anyhow, Result};
use serde_json::{self, from_str};

use std::{
    env::var,
    io::Read,
    process::{Command, ExitStatus},
};

/// Open the target file with an editor.
fn spawn_editor(editor: String, file: String) -> Result<ExitStatus, NomadError> {
    Command::new(editor.clone())
        .arg(&file)
        .status()
        .map_err(|error| NomadError::EditorError {
            editor,
            reason: error,
        })
}

/// Get the default text editor from the environment. If that environment variable
/// is not set, try to open the file with Neovim, then Vim, and finally Nano.
fn get_text_editors() -> Vec<String> {
    var("EDITOR").map_or(
        vec![
            "nvim".to_string(),
            "vim".to_string(),
            "vi".to_string(),
            "nano".to_string(),
        ],
        |editor| vec![editor],
    )
}

/// Checks the JSON file for a filepath that corresponds with the entered file number.
pub fn get_file(target: String, json_target: JSONTarget) -> Result<String, NomadError> {
    if let Some(file) = search_for_file(target.clone(), json_target)? {
        Ok(file)
    } else {
        match json_target {
            JSONTarget::Contents => Err(NomadError::Error(anyhow!(
                "File #{target} is not in the tree!\nRun nomad in numbered mode and try again."
            ))),
            JSONTarget::Directories => Err(NomadError::Error(anyhow!(
                "Directory {target} is not in the tree!\nRun nomad in labeled directories mode and try again."
            )))
        }
    }
}

/// Get the deserialized JSON file.
pub fn get_deserialized_json(json_target: JSONTarget) -> Result<Contents, NomadError> {
    let mut file = get_json_file(json_target, true)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    Ok(from_str(&data)?)
}

/// Search for the target file by parsing the JSON file and retrieving the value
/// associated with the target index that was passed in.
fn search_for_file(target: String, json_target: JSONTarget) -> Result<Option<String>, NomadError> {
    let mut file = get_json_file(json_target, true)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let json: Contents = from_str(&data)?;
    if !json.items.contains_key(&target) {
        Ok(None)
    } else {
        json.items
            .get(&target)
            .map_or(Ok(None), |file_path| Ok(Some(file_path.into())))
    }
}

/// Open the target file.
pub fn open_file(file: String) -> Result<(), NomadError> {
    let editors = get_text_editors();

    if editors.len() == 1 {
        spawn_editor(editors[0].to_string(), file).map_or_else(
            |error| Err(error),
            |status_code| {
                println!("{status_code}");
                Ok(())
            },
        )
    } else {
        for editor in editors {
            match spawn_editor(editor, file.to_string()) {
                Ok(_) => {
                    return Ok(());
                }
                Err(_) => {}
            };
        }

        Err(NomadError::Error(anyhow!("Could not open the file with your $EDITOR, Neovim, Vim, Vi, or Nano!\nDo you have one of these editors installed?")))
    }
}
