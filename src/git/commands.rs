//! Contains functions that simulate Git commands.

use crate::{
    cli::Args,
    git::utils::{get_last_commit, get_repo_branch},
    traverse::{build_tree, check_nesting},
    utils::{open::get_file, paths::get_current_directory},
};

use ansi_term::Colour;
use git2::{Error, ErrorClass, ErrorCode, Index, ObjectType, Repository, Tree};
use ignore::Walk;

use std::path::Path;

use super::markers::get_status_markers;

/// Build a tree that only contains items that are tracked in Git.
pub fn status_tree(args: &Args, repo: &Repository, target_directory: &str, walker: &mut Walk) {
    if let Ok(marker_map) = get_status_markers(&repo, target_directory) {
        //
        // TODO:
        //  1. Compare marker_map contents with walker items
        //      a. Create a Vec<Path>? that only contains items present in both iterators(?)
        //  2. Create a new function that builds a tree from the Vec of items.
        //
        let current_depth: usize = 0;
        let current_dir = get_current_directory().unwrap_or("?".to_string());
        let mut previous_item = walker
            .next() // Sets the first `previous_item` to the `target_directory`.
            .expect("No items were found in this directory!")
            .unwrap_or_else(|error| panic!("Could not retrieve items in this directory! {error}"));

        let (config, mut tree) = build_tree(args.metadata, &previous_item);
        while let Some(Ok(item)) = walker.next() {
            check_nesting(current_depth, &item, previous_item, &mut tree);

            let sliced_path = item.path().strip_prefix(&current_dir).expect(&format!(
                "Could not strip path prefix for {}!",
                item.path().to_str().unwrap_or("?")
            ));
            println!(
                "Sliced path is: {sliced_path}",
                sliced_path = sliced_path.to_str().unwrap_or("?")
            );

            previous_item = item;
        }
    } else {
        println!(
            "{}",
            Colour::Fixed(172).paint(format!("No Git changes found in {target_directory}.\n"))
        );
    }
}

/// Stage file(s) by adding them to the Git index (the staging area between the
/// working directory and the repository). Then return the tree containing staged
/// items (the Git index tree).
pub fn stage_files(file_numbers: &Vec<i32>, repo: &Repository) -> Result<(), Error> {
    let mut index = repo.index()?;

    let mut staged_files = 0;
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

/// Get Git diff statistics by comparing the HEAD and index.
fn get_diff_stats(
    index: &mut Index,
    old_tree: &Tree,
    repo: &Repository,
) -> (Option<usize>, Option<usize>, Option<usize>) {
    if let Ok(diff) = repo.diff_tree_to_index(Some(old_tree), Some(&index), None) {
        if let Ok(diff_stats) = diff.stats() {
            (
                Some(diff_stats.files_changed()),
                Some(diff_stats.insertions()),
                Some(diff_stats.deletions()),
            )
        } else {
            (None, None, None)
        }
    } else {
        (None, None, None)
    }
}

/// Commit the staged changes with an accompanying message if applicable.
pub fn commit_changes(message: &Option<String>, repo: &Repository) -> Result<(), Error> {
    if let Ok(signature) = repo.signature() {
        let checked_message = if let Some(message) = message {
            message.to_string()
        } else {
            "Updating".to_string()
        };

        let mut index = repo.index()?;
        let staged_tree = repo.find_tree(index.write_tree()?)?;

        let previous_head = repo.head()?.peel(ObjectType::Tree)?.id();

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

        let branch_name = get_repo_branch(&repo).unwrap_or("?".to_string());
        let branch = Colour::Green
            .bold()
            .paint(format!("{branch_name}"))
            .to_string();

        let sliced_oid = &commit_oid[..7];

        println!("\n[{branch} {sliced_oid}] {checked_message}\n");

        let old_tree = repo.find_tree(previous_head)?;
        if let (Some(files_changed), Some(insertions), Some(deletions)) =
            get_diff_stats(&mut index, &old_tree, repo)
        {
            println!(
                "{colored_changed} {changed_label} changed | {colored_insertions} {insertions_label} | {colored_deletions} {deletions_label}\n",
                colored_changed = Colour::Fixed(172).bold().paint(format!("{files_changed}")),
                changed_label = if files_changed == 1 { "file" } else { "files" },
                colored_insertions = Colour::Green.bold().paint(format!("+{insertions}")),
                insertions_label = if insertions == 1 { "insertion" } else { "insertions" },
                colored_deletions = Colour::Red.bold().paint(format!("-{deletions}")),
                deletions_label = if deletions == 1 { "deletion" } else { "deletions" },
            );
        }

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
