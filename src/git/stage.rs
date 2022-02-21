//! Stage changes in the Git repository.

use std::path::Path;

use ansi_term::Colour;
use git2::{Error, Repository};

use super::utils::{get_repo_branch, indiscriminate_search, SearchMode};

/// Stage file(s) by adding them to the Git index (the staging area between the
/// working directory and the repository). Then return the tree containing staged
/// items (the Git index tree).
pub fn stage_files(
    item_labels: &Vec<String>,
    repo: &Repository,
    target_directory: &str,
) -> Result<(), Error> {
    let mut index = repo.index()?;

    let found_items =
        indiscriminate_search(item_labels, Some(repo), SearchMode::Git, target_directory);

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

            staged_files += 1;
            index.add_path(relative_path)?;
        }
    } else {
        if let Some(branch_name) = get_repo_branch(repo) {
            println!(
                "\nOn branch: {}\n",
                Colour::Green.bold().paint(format!("{branch_name}"))
            );
        }
        // TODO: CALL THE GIT STATUS TREE HERE WITH THE BRANCH N SHIT.
    }

    if staged_files > 0 {
        index.write()?;

        println!(
            "\nStaged {} {ends_with}\n",
            Colour::Green.bold().paint(format!("{staged_files}")),
            ends_with = if staged_files == 1 { "item" } else { "items" }
        );
    }

    Ok(())
}
