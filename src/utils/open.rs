//! Open a file using the client's system's `$EDITOR`.

use super::temp::get_json_file;
use crate::models::Contents;

use serde_json::{self, from_str};

use std::{
    env::var,
    io::{Error, ErrorKind, Read},
    process::Command,
};

/// Get the default text editor from the environment. If that environment variable
/// is not set, search for installed text editors, then open the file with one
/// of the installed editors.
fn get_text_editor() -> Result<String, Error> {
    let editor = var("EDITOR").expect("Could not get the system's default editor!");
    println!("EDITOR IS {}", editor);

    Ok(editor)
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

/// Open the target file.
pub fn open_file(target_index: String) -> Result<(), Error> {
    search_for_file(target_index)?.map_or_else(
        || Err(Error::new(ErrorKind::NotFound, "That file does not exist!")),
        |filename| {
            println!("FOUND FILE: {}", filename);
            Ok(())
        },
    )
}
