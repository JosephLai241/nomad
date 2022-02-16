//! Stage changes in the Git repository.

use std::path::Path;

use ansi_term::Colour;
use git2::{Error, Repository};

use crate::utils::{open::get_file, temp::JSONTarget};

/// Stage file(s) by adding them to the Git index (the staging area between the
/// working directory and the repository). Then return the tree containing staged
/// items (the Git index tree).
pub fn stage_files(
    file_numbers: &Vec<i32>,
    repo: &Repository,
    target_directory: &str,
) -> Result<(), Error> {
    let mut index = repo.index()?;

    let mut staged_files = 0;
    for number in file_numbers {
        if let Ok(filename) = get_file(number.to_string(), JSONTarget::Contents) {
            let target_file = Path::new(&filename);
            let relative_path =
                if let Ok(prefix_stripped) = Path::new(&filename).strip_prefix(target_directory) {
                    prefix_stripped
                } else {
                    target_file
                };

            staged_files += 1;
            index.add_path(relative_path)?;
        }
    }

    println!(
        "\nStaged {} {ends_with}\n",
        Colour::Green.bold().paint(format!("{staged_files}")),
        ends_with = if staged_files == 1 { "item" } else { "items" }
    );
    index.write()?;

    Ok(())
}
