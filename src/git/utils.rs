//! Contains useful utilities that support Git functionality.

use std::{ffi::OsStr, path::Path};

use crate::{errors::NomadError, style::models::NomadStyle};

use ansi_term::{Colour, Style};
use anyhow::{anyhow, Result};
use git2::{Branch, Commit, ObjectType, Repository};

/// Try to discover a Git repository at or above the current path.
fn discover_repo(target_directory: &str) -> Option<Repository> {
    if let Ok(repo) = Repository::discover(target_directory) {
        if repo.is_bare() {
            println!("\n{}", Colour::Fixed(172).paint("Git repository is bare!"));
            None
        } else {
            Some(repo)
        }
    } else {
        None
    }
}

/// Try to get Git metadata from the target directory.
pub fn get_repo(target_directory: &str) -> Option<Repository> {
    if let Ok(repo) = Repository::open(target_directory) {
        if repo.is_bare() {
            println!("\n{}", Colour::Fixed(172).paint("Git repository is bare!"));
            None
        } else {
            Some(repo)
        }
    } else {
        discover_repo(target_directory)
    }
}

/// Try to get the current Git branch's name.
pub fn get_repo_branch(repo: &Repository) -> Option<String> {
    if let Ok(reference) = repo.head() {
        if let Ok(Some(name)) = Branch::wrap(reference).name() {
            let branch_name = name.to_string();
            Some(branch_name)
        } else {
            println!(
                "\n{}\n",
                Colour::Red
                    .bold()
                    .paint("Could not get the current Git branch name!")
            );
            None
        }
    } else {
        println!(
            "\n{}\n",
            Colour::Red.bold().paint("Could not get repository HEAD!")
        );
        None
    }
}

/// Get the last commit in the Git repository.
pub fn get_last_commit(repo: &Repository) -> Result<Commit, NomadError> {
    let object = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    object.into_commit().map_err(|_| {
        NomadError::Error(anyhow!(
            "Could not find the last commit in this Git repository!"
        ))
    })
}

/// Add color/style to the filename depending on its Git status.
pub fn paint_git_item(
    filename: &str,
    marker: &str,
    nomad_style: &NomadStyle,
    matched: Option<(usize, usize)>,
) -> String {
    let staged_deleted = &nomad_style
        .git
        .staged_deleted_color
        .paint(&nomad_style.git.staged_deleted_marker)
        .to_string();
    let staged_modified = &nomad_style
        .git
        .staged_modified_color
        .paint(&nomad_style.git.staged_modified_marker)
        .to_string();
    let staged_added = &nomad_style
        .git
        .staged_added_color
        .paint(&nomad_style.git.staged_added_marker)
        .to_string();
    let staged_renamed = &nomad_style
        .git
        .staged_renamed_color
        .paint(&nomad_style.git.staged_renamed_marker)
        .to_string();
    let conflicted = &nomad_style
        .git
        .conflicted_color
        .paint(&nomad_style.git.conflicted_marker)
        .to_string();

    let style = match marker.to_string() {
        _ if marker == staged_added => nomad_style.git.staged_added_color,
        _ if marker == staged_deleted => nomad_style.git.staged_deleted_color.strikethrough(),
        _ if marker == staged_modified => nomad_style.git.staged_modified_color,
        _ if marker == staged_renamed => nomad_style.git.staged_renamed_color,
        _ if marker == conflicted => nomad_style.git.conflicted_color,
        _ => Style::new(),
    };

    paint_with_highlight(filename, matched, nomad_style, style)
}

/// Highlight the filename in the Git status color. Also paint the pattern if a
/// pattern is matched.
fn paint_with_highlight(
    filename: &str,
    matched: Option<(usize, usize)>,
    nomad_style: &NomadStyle,
    style: Style,
) -> String {
    let painted_file = match matched {
        Some(ranges) => {
            if (0..filename.len()).contains(&ranges.0)
                && (0..filename.len() + 1).contains(&ranges.1)
            {
                let mut painted_prefix = filename[..ranges.0]
                    .chars()
                    .into_iter()
                    .map(|character| style.paint(format!("{character}")).to_string())
                    .collect::<Vec<String>>();
                let mut painted_matched = filename[ranges.0..ranges.1]
                    .chars()
                    .into_iter()
                    .map(|character| {
                        nomad_style
                            .tree
                            .regex
                            .match_color
                            .paint(format!("{character}"))
                            .to_string()
                    })
                    .collect::<Vec<String>>();
                let mut painted_suffix = filename[ranges.1..]
                    .chars()
                    .into_iter()
                    .map(|character| style.paint(format!("{character}")).to_string())
                    .collect::<Vec<String>>();

                painted_prefix.append(&mut painted_matched);
                painted_prefix.append(&mut painted_suffix);

                painted_prefix.join("").to_string()
            } else {
                filename
                    .chars()
                    .into_iter()
                    .map(|character| style.paint(format!("{character}")).to_string())
                    .collect::<Vec<String>>()
                    .join("")
                    .to_string()
            }
        }
        None => filename
            .chars()
            .into_iter()
            .map(|character| style.paint(format!("{character}")).to_string())
            .collect::<Vec<String>>()
            .join("")
            .to_string(),
    };

    let filename = Path::new(&painted_file)
        .file_name()
        .unwrap_or(OsStr::new("?"))
        .to_str()
        .unwrap_or("?")
        .to_string();

    filename
}
