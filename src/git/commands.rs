//! Contains functions that simulate Git commands.

use crate::{
    git::utils::{get_last_commit, get_repo_branch},
    utils::{open::get_file, paths::get_current_directory},
};

use git2::{Error, ErrorClass, ErrorCode, Repository, Tree};

use std::path::Path;

/// Stage file(s) by adding them to the Git index (the staging area between the
/// working directory and the repository. Then return the tree containing staged
/// items (the Git index tree).
pub fn stage_files(file_numbers: Vec<i32>, repo: &Repository) -> Result<Tree, Error> {
    let mut index = repo.index()?;

    for number in file_numbers {
        if let Ok(filename) = get_file(number.to_string()) {
            let target_file = Path::new(&filename);
            let relative_path = if let Ok(prefix_stripped) =
                Path::new(&filename).strip_prefix(get_current_directory().unwrap_or("".to_string()))
            {
                prefix_stripped
            } else {
                target_file
            };

            index.add_path(relative_path)?;
        }
    }

    Ok(repo.find_tree(index.write_tree()?)?)
}

/// Commit the staged changes with an accompanying message if applicable (`git commit -m <MESSAGE>`).
pub fn commit_changes(
    message: Option<String>,
    repo: &Repository,
    staged_tree: Tree,
) -> Result<(), Error> {
    if let Ok(signature) = repo.signature() {
        let checked_message = if let Some(message) = message {
            message
        } else {
            "Updating".to_string()
        };

        let parent_commit = get_last_commit(&repo)?;
        let commit_oid = repo
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                &checked_message,
                &staged_tree,
                &[&parent_commit],
            )?
            .to_string();

        let branch = get_repo_branch(&repo).unwrap_or("?".to_string());
        let sliced_oid = &commit_oid[commit_oid.len() - 7..commit_oid.len()];
        let metadata = "".to_string();
        println!("\n[{branch} {sliced_oid}] {checked_message}\n{metadata}");

        Ok(())
    } else {
        Err(
            Error::new(
                ErrorCode::NotFound,
                ErrorClass::Repository,
                "Could not find a Git signature within this Git repository! Unable to commit changes without the signature."
            )
        )
    }
}
