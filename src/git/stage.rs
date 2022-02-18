//! Stage changes in the Git repository.

use std::path::Path;

use ansi_term::Colour;
use git2::{Error, Repository};

use crate::{
    git::markers::get_status_markers,
    utils::{open::get_deserialized_json, temp::JSONTarget},
};

/// Stage file(s) by adding them to the Git index (the staging area between the
/// working directory and the repository). Then return the tree containing staged
/// items (the Git index tree).
pub fn stage_files(
    item_labels: &Vec<String>,
    repo: &Repository,
    target_directory: &str,
) -> Result<(), Error> {
    let mut index = repo.index()?;

    if let (Ok(contents), Ok(directories), Ok(marker_map)) = (
        get_deserialized_json(JSONTarget::Contents),
        get_deserialized_json(JSONTarget::Directories),
        get_status_markers(repo, target_directory),
    ) {
        let mut not_found: Vec<String> = Vec::new();

        let mut staged_files = 0;
        for label in item_labels {
            match label.parse::<i32>() {
                Ok(_) => match contents.items.get(label) {
                    Some(file_path) => {
                        let target_file = Path::new(&file_path);
                        let relative_path = if let Ok(prefix_stripped) =
                            Path::new(&file_path).strip_prefix(target_directory)
                        {
                            prefix_stripped
                        } else {
                            target_file
                        };

                        staged_files += 1;
                        index.add_path(relative_path)?;
                    }
                    None => not_found.push(label.into()),
                },
                Err(_) => match directories.items.get(label) {
                    Some(directory_path) => {
                        for file_path in marker_map.keys() {
                            let path_parent = Path::new(file_path)
                                .parent()
                                .unwrap_or(Path::new("?"))
                                .to_str()
                                .unwrap_or("?");

                            if path_parent.contains(directory_path) {
                                let target_file = Path::new(&file_path);
                                let relative_path = if let Ok(prefix_stripped) =
                                    Path::new(&file_path).strip_prefix(target_directory)
                                {
                                    prefix_stripped
                                } else {
                                    target_file
                                };

                                staged_files += 1;
                                index.add_path(relative_path)?;
                            }
                        }
                    }
                    None => not_found.push(label.into()),
                },
            };
        }

        if !not_found.is_empty() {
            println!(
                "{}",
                Colour::Fixed(172).bold().paint(
                    "\nThe following item numbers or directory labels did not match any items in the tree:\n"
                )
            );

            for label in not_found {
                println!("=> {}", Colour::Fixed(172).bold().paint(format!("{label}")));
            }

            // TODO: CALL THE GIT STATUS TREE HERE AND DISPLAY IT WITH ALL NUMBERS AND DIRECTORY
            // LABELS.
        }

        if staged_files > 0 {
            println!(
                "\nStaged {} {ends_with}\n",
                Colour::Green.bold().paint(format!("{staged_files}")),
                ends_with = if staged_files == 1 { "item" } else { "items" }
            );
        }

        index.write()?;
    } else {
        println!("{}", Colour::Red.bold().paint("\nCould not retrieve stored directories and directory contents!\nDid you run nomad in numbered or labeled directories mode?\n"));
    }

    Ok(())
}
