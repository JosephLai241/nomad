//! Open a file using the client's system's `$EDITOR`.

use super::temp::get_json_file;
use crate::{models::Contents, traverse::canonicalize_path};

use serde_json::{self, from_str};

use std::{
    env::var,
    io::{Error, ErrorKind, Read},
    process::{Command, ExitStatus},
};

/// Open the target file with an editor.
fn spawn_editor(editor: String, file: String) -> Result<ExitStatus, Error> {
    Command::new(editor).arg(&file).status()
}

/// Get the default text editor from the environment. If that environment variable
/// is not set, try to open the file with Neovim, then Vim, and finally Nano.
fn get_text_editors() -> Vec<String> {
    var("EDITOR").map_or(
        vec!["nvim".to_string(), "vim".to_string(), "nano".to_string()],
        |editor| vec![editor],
    )
}

/// Search for the target file by parsing the JSON file and retrieving the value
/// associated with the target index that was passed in.
fn search_for_file(target_index: String) -> Result<Option<String>, Error> {
    let mut file = get_json_file(true)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let json: Contents = from_str(&data)?;
    if !json.items.contains_key(&target_index) {
        Ok(None)
    } else {
        json.items
            .get(&target_index)
            .map_or(Ok(None), |file_path| Ok(Some(file_path.into())))
    }
}

/// Checks the input for flags that take a filename or index number (`-b` and `-o`).
///
/// This enables the ability to open files via its index number within the tree
/// or directly by the filename.
pub fn get_file(target: String) -> Result<String, Error> {
    if let Some(file) = search_for_file(target.clone())? {
        Ok(file)
    } else {
        let target_file = canonicalize_path(&target)?;

        Ok(target_file)
    }
}

/// Open the target file.
pub fn open_file(file: String) -> Result<(), Error> {
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
                Ok(status_code) => {
                    println!("{status_code}");
                    return Ok(());
                }
                Err(_) => {}
            };
        }

        Err(Error::new(
            ErrorKind::NotFound,
            "Could not open the file with your $EDITOR, Neovim, Vim, or Nano! Do you have one of these editors installed?",
        ))
    }
}
