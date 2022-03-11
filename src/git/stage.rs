//! Stage changes in the Git repository.

use std::path::Path;

use ansi_term::Colour;
use git2::{Error, Repository};

use crate::{
    cli::Args,
    style::models::NomadStyle,
    utils::search::{indiscriminate_search, SearchMode},
};

/// Contains variants for stage/restore modes.
pub enum StageMode {
    /// Stage files from the working directory into the index.
    Stage,
    /// Restore staged files back to the working directory.
    RestoreStaged,
    /// Restore files in the working directory back to their clean Git state.
    RestoreWorkingDirectory,
}

/// Stage file(s) by adding them to the Git index (the staging area between the
/// working directory and the repository). Then return the tree containing staged
/// items (the Git index tree).
pub fn stage_files(
    args: &Args,
    item_labels: &Vec<String>,
    nomad_style: &NomadStyle,
    repo: &Repository,
    stage_mode: StageMode,
    target_directory: &str,
) -> Result<(), Error> {
    let mut index = repo.index()?;

    let found_items = indiscriminate_search(
        args,
        item_labels,
        nomad_style,
        Some(repo),
        SearchMode::Git,
        target_directory,
    );

    let mut staged_files = 0;
    if let Some(found_items) = found_items {
        for item in found_items {
            let target_file = Path::new(&item);
            let relative_path =
                if let Ok(prefix_stripped) = Path::new(&item).strip_prefix(target_directory) {
                    prefix_stripped
                } else {
                    target_file
                };

            match stage_mode {
                StageMode::Stage => {
                    if let Err(_) = index.add_path(relative_path) {
                        // May need to revisit this in the future to account for different errors. We'll see.
                        index.remove_path(relative_path)?;
                    }
                }
                StageMode::RestoreStaged => {
                    if let Err(_) = index.remove_path(relative_path) {
                        // May need to revisit this in the future to account for different errors. We'll see.
                        index.add_path(relative_path)?;
                    }
                }
                StageMode::RestoreWorkingDirectory => {}
            }

            staged_files += 1;
        }
    }

    if staged_files > 0 {
        index.write()?;

        println!(
            "\nStaged {} {}\n",
            Colour::Green.bold().paint(format!("{staged_files}")),
            if staged_files == 1 { "item" } else { "items" }
        );
    } else {
        println!("{}\n", Colour::Red.bold().paint("No items were staged!"));
    }

    Ok(())
}
