//! Commit staged changes in the Git repository.

use ansi_term::Colour;
use git2::{ObjectType, Repository};

use crate::{
    errors::NomadError,
    git::{
        diff::get_diff_stats,
        utils::{get_last_commit, get_repo_branch},
    },
};

/// Commit the staged changes with an accompanying message if applicable.
pub fn commit_changes(message: &Option<String>, repo: &Repository) -> Result<(), NomadError> {
    match repo.signature() {
        Ok(signature) => {
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
        }
        Err(error) => Err(NomadError::GitError {
            context: "Unable to commit changes without a Git signature".into(),
            source: error,
        }),
    }
}
