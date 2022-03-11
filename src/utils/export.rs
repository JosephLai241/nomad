//! Export a directory's tree to a file instead of saving.

use crate::errors::NomadError;

use ansi_term::*;
use anyhow::Result;
use chrono::Local;
use ptree::{item::StringItem, write_tree_with, PrintConfig};

use std::{env, fs::File, io::Write};

/// Get the absolute path for the file name.
fn get_absolute_path(file_name: &str) -> Result<String, NomadError> {
    Ok(env::current_dir()?
        .join(file_name)
        .into_os_string()
        .into_string()
        .expect("Could not get the current directory!")
        .clone())
}

/// Variants for export modes.
pub enum ExportMode<'a> {
    /// `nomad` was run in filetype mode.
    Filetype(&'a Vec<String>, &'a Vec<String>),
    /// `nomad` was run in normal mode.
    Normal,
    /// `nomad` was run in Git status mode.
    GitStatus,
}

/// Export the tree to a file. Writes to a custom filename if specified, otherwise
/// the filename corresponds to the tree mode (normal, filetype, or Git status)
/// and the current timestamp.
pub fn export_tree(
    config: PrintConfig,
    export_mode: ExportMode,
    filename: &Option<String>,
    tree: StringItem,
) -> Result<(), NomadError> {
    let mut file_header = "nomad".to_string();

    let mut default_filename = match export_mode {
        ExportMode::Filetype(filetypes, globs) => {
            let mut filetype_info = "\n\n".to_string();
            if !filetypes.is_empty() {
                filetype_info.push_str(&format!("Filetypes: {}", filetypes.join(", ").to_string()));
            }

            if !globs.is_empty() {
                filetype_info.push_str(&format!("\nGlobs: {}", globs.join(", ").to_string()));
            }

            filetype_info.push_str("\n\n");

            file_header.push_str(&filetype_info);

            "filetype".to_string()
        }
        ExportMode::Normal => {
            file_header.push_str("\n\n");

            "nomad".to_string()
        }
        ExportMode::GitStatus => {
            file_header.push_str("\n\nMode: Git status\n\n");

            "git_status".to_string()
        }
    };

    let export_filename = if let Some(filename) = filename {
        filename.to_string()
    } else {
        let timestamp = Local::now().format("%F_%H-%M-%S").to_string();
        default_filename.push_str(&format!("_{}.txt", timestamp));

        default_filename
    };

    let file_path = get_absolute_path(&export_filename)?;
    let mut file = File::create(&file_path)?;
    write!(file, "{}", file_header)?;

    write_tree_with(&tree, file, &config).map_or_else(
        |error| {
            Err(NomadError::PTreeError {
                context: format!("Unable to export directory tree to {file_path}"),
                source: error,
            })
        },
        |_| {
            let success_message = Colour::Green
                .bold()
                .paint(format!("Tree was exported to {file_path}\n"));
            println!("{success_message}");

            Ok(())
        },
    )
}
