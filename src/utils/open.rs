//! Open a file using the client's system's `$EDITOR`.

use super::temp::get_json_file;
use crate::{errors::NomadError, models::Contents};

use anyhow::{anyhow, Result};
use serde_json::{self, from_str};

use std::{
    env::var,
    io::Read,
    process::{Command, ExitStatus},
};

/// Open the target file with an editor.
fn spawn_editor(editor: String, found_items: Vec<String>) -> Result<ExitStatus, NomadError> {
    Command::new(editor.clone())
        .args(&found_items)
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

/// Get the deserialized JSON file.
pub fn get_deserialized_json() -> Result<Contents, NomadError> {
    let mut file = get_json_file(true)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    Ok(from_str(&data)?)
}

/// Open the target file.
pub fn open_files(found_items: Vec<String>) -> Result<(), NomadError> {
    let editors = get_text_editors();

    if editors.len() == 1 {
        spawn_editor(editors[0].to_string(), found_items).map_or_else(
            |error| Err(error),
            |status_code| {
                println!("{status_code}");
                Ok(())
            },
        )
    } else {
        for editor in editors {
            match spawn_editor(editor, found_items.clone()) {
                Ok(_) => {
                    return Ok(());
                }
                Err(_) => {}
            };
        }

        Err(NomadError::Error(anyhow!("Could not open the file with your $EDITOR, Neovim, Vim, Vi, or Nano!\nDo you have one of these editors installed?")))
    }
}
