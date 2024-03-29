//! Modify the Git trees - stages or restores files.

use std::path::Path;

use ansi_term::Colour;
use git2::{build::CheckoutBuilder, Error, Index, IndexAddOption, Repository, Tree};

use crate::{
    cli::Args,
    style::models::NomadStyle,
    utils::search::{indiscriminate_search, SearchMode},
};

/// Contains variants for stage/unstage/restore modes.
pub enum TreeMode {
    /// Stage specified files from the working directory into the index.
    Stage,
    /// Stage all modified, deleted, or untracked files from the working directory
    /// into the index.
    StageAll,
    /// Restore files in the working directory back to their clean Git state.
    RestoreWorkingDirectory,
}

/// Modify the Git trees to stage/unstage/restore files.
///
/// This function may do any of the following:
///     * Adds new or modified files to the current index.
///     * Restores staged files from the staging area to the index (unstage a file).
///     * Restores modified files from the working directory to its clean state.
///
pub fn modify_trees(
    args: &Args,
    item_labels: &[String],
    nomad_style: &NomadStyle,
    repo: &Repository,
    stage_mode: TreeMode,
    target_directory: &str,
) -> Result<(), Error> {
    let head_tree = repo.head()?.peel_to_tree()?;
    let mut index = repo.index()?;

    let mut staged_files = 0;
    match stage_mode {
        TreeMode::StageAll => {
            index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
            index.write()?;

            println!("\n{}\n", Colour::Green.bold().paint("Staged all files"));
        }
        _ => {
            let found_items = indiscriminate_search(
                args,
                item_labels,
                nomad_style,
                Some(repo),
                SearchMode::Git,
                target_directory,
            );

            if let Some(found_items) = found_items {
                for item in found_items {
                    let target_file = Path::new(&item);
                    let relative_path = match Path::new(&item).strip_prefix(target_directory) {
                        Ok(prefix_stripped) => prefix_stripped,
                        Err(_) => target_file,
                    };

                    match stage_mode {
                        TreeMode::Stage => {
                            if index.add_path(relative_path).is_err() {
                                index.remove_path(relative_path)?;
                            }

                            staged_files += 1;
                        }
                        TreeMode::RestoreWorkingDirectory => {
                            restore_file(
                                &head_tree,
                                &mut index,
                                relative_path,
                                repo,
                                &mut staged_files,
                            )?;
                        }
                        _ => {}
                    }
                }
            }

            if staged_files > 0 {
                index.write()?;

                let info = match stage_mode {
                    TreeMode::Stage => "Staged",
                    TreeMode::RestoreWorkingDirectory => "Restored",
                    _ => "",
                };

                println!(
                    "\n{} {} {}\n",
                    info,
                    Colour::Green.bold().paint(format!("{staged_files}")),
                    if staged_files == 1 { "item" } else { "items" }
                );
            } else {
                println!("{}\n", Colour::Red.bold().paint("No items were staged!"));
            }
        }
    }

    Ok(())
}

/// Restore a file to its working directory or clean state.
fn restore_file(
    head_tree: &Tree,
    index: &mut Index,
    relative_path: &Path,
    repo: &Repository,
    staged_files: &mut i32,
) -> Result<(), Error> {
    if head_tree
        .get_name(relative_path.to_str().unwrap_or("?"))
        .is_some()
    {
        let mut checkout_options = CheckoutBuilder::new();
        checkout_options.force();
        checkout_options.path(relative_path);

        repo.checkout_head(Some(&mut checkout_options))?;
        *staged_files += 1;
    } else {
        // Indicates this file was untracked prior to adding it to the index.
        index.remove_path(relative_path)?;
        *staged_files += 1;
    }

    Ok(())
}
