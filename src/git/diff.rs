//! Exposing functionality for the Git diff command.

use std::{path::Path, str::from_utf8};

use ansi_term::Colour;
use anyhow::Result;
use bat::{Input, PagingMode, PrettyPrinter, WrappingMode};
use git2::{Delta, Diff, DiffDelta, DiffFormat, Error, Index, ObjectType, Repository, Tree};

use crate::errors::NomadError;

/// Get the diff between the old Git tree and the working directory using the Git index.
pub fn get_repo_diffs<'a>(repo: &'a Repository) -> Result<Diff<'a>, Error> {
    let previous_head = repo.head()?.peel(ObjectType::Tree)?.id();
    let old_tree = repo.find_tree(previous_head)?;

    let diff = repo.diff_tree_to_workdir_with_index(Some(&old_tree), None)?;

    Ok(diff)
}

/// Colorize the origin of the `DiffLine`.
pub fn colorize_origin(marker: char) -> String {
    match marker {
        '+' | '>' => Colour::Green.bold().paint(format!("{marker}")).to_string(),
        '-' | '<' => Colour::Red.bold().paint(format!("{marker}")).to_string(),
        _ => Colour::White.bold().paint(format!("{marker}")).to_string(),
    }
}

/// Use `bat` to display Git diffs.
pub fn bat_diffs(
    diff: Diff,
    found_items: Option<Vec<String>>,
    target_directory: &str,
) -> Result<(), NomadError> {
    let formatted_diffs = get_diffs(diff, found_items, target_directory)?;

    if !formatted_diffs.is_empty() {
        if let Err(error) = PrettyPrinter::new()
            .grid(true)
            .header(true)
            .inputs(
                formatted_diffs
                    .iter()
                    .map(|(name, diff)| Input::from_bytes(diff.as_bytes()).name(name))
                    .collect::<Vec<Input>>(),
            )
            .paging_mode(PagingMode::QuitIfOneScreen)
            .rule(true)
            .true_color(true)
            .wrapping_mode(WrappingMode::Character)
            .print()
        {
            return Err(NomadError::BatError(error));
        }
    } else {
        println!("{}", Colour::Red.bold().paint("\nNo diffs available!\n"));
    }

    Ok(())
}

/// Traverse Git diffs, format each line, and return a Vec containing the formatted
/// filename and the associated diff.
/// If item labels are specified, only items that are tracked by Git AND contain
/// changes are returned.
fn get_diffs(
    diff: Diff,
    found_items: Option<Vec<String>>,
    target_directory: &str,
) -> Result<Vec<(String, String)>, NomadError> {
    let stripped_items = if let Some(found_items) = found_items {
        if !found_items.is_empty() {
            Some(
                found_items
                    .iter()
                    .map(|full_path| {
                        Path::new(full_path)
                            .strip_prefix(target_directory)
                            .unwrap_or(Path::new("?"))
                            .to_str()
                            .unwrap_or("?")
                            .to_string()
                    })
                    .collect::<Vec<String>>(),
            )
        } else {
            None
        }
    } else {
        None
    };

    let mut content: Vec<String> = Vec::new();

    let mut current_delta: Option<Delta> = None;
    let mut file_mode = String::new();
    let mut filename = String::new();
    let mut new_old_oids = String::new();
    let mut new_file = String::new();
    let mut old_file = String::new();

    let mut added_lines: u32 = 0;
    let mut deleted_lines: u32 = 0;

    let mut formatted_diffs: Vec<(String, String)> = Vec::new();

    diff.print(DiffFormat::Patch, |delta, hunk, line| {
        if filename.is_empty() {
            let (new_filename, old_filename) = get_new_old_filenames(&delta);
            new_file = new_filename;
            old_file = old_filename;

            filename = if old_file == new_file {
                new_file.clone()
            } else {
                format!("{old_file} ==> {new_file}")
            };
        }

        if let Some(hunk) = hunk {
            match line.origin() {
                // Format the file or hunk header for better clarity.
                'H' | 'F' => {
                    let number_line = if hunk.old_start() != hunk.new_start() {
                        format!(
                            "\n@@ {} {} {} {}",
                            Colour::White.bold().paint("Line"),
                            Colour::Red.bold().paint(hunk.old_start().to_string()),
                            Colour::White.bold().paint("==>"),
                            Colour::Green.bold().paint(hunk.new_start().to_string())
                        )
                        .to_string()
                    } else {
                        Colour::White
                            .bold()
                            .paint(format!("\n@@ Line {}", hunk.old_start()))
                            .to_string()
                    };

                    let num_lines = if hunk.old_lines() != hunk.new_lines() {
                        format!(
                            "{} {} {} {}",
                            Colour::White.bold().paint("# of lines:"),
                            Colour::Red.bold().paint(hunk.old_lines().to_string()),
                            Colour::White.bold().paint("==>"),
                            Colour::Green.bold().paint(hunk.new_lines().to_string())
                        )
                    } else {
                        Colour::White
                            .bold()
                            .paint(format!("# of lines: {}", hunk.old_lines()))
                            .to_string()
                    };

                    content.push(format!(
                        "{number_line} {} {num_lines}\n\n",
                        Colour::White.bold().paint("|")
                    ));
                }
                // Otherwise format the line based on the Git Delta's status.
                _ => {
                    let content_text = from_utf8(line.content()).unwrap_or("?");

                    match delta.status() {
                        Delta::Added => {
                            content.push(format!(
                                "{} {}",
                                colorize_origin(line.origin()),
                                Colour::Green.paint(content_text)
                            ));

                            current_delta = Some(Delta::Added);
                            added_lines += 1;
                        }
                        Delta::Conflicted => {
                            content.push(format!(
                                "{} {}",
                                colorize_origin(line.origin()),
                                Colour::Fixed(172).paint(content_text)
                            ));

                            current_delta = Some(Delta::Conflicted);
                        }
                        Delta::Deleted => {
                            content.push(format!(
                                "{} {}",
                                colorize_origin(line.origin()),
                                Colour::Red.paint(content_text)
                            ));

                            current_delta = Some(Delta::Deleted);
                            deleted_lines += 1;
                        }
                        Delta::Modified => {
                            let colorized_content = match line.origin() {
                                '+' | '>' => {
                                    added_lines += 1;

                                    Colour::Green.paint(content_text).to_string()
                                }
                                '-' | '<' => {
                                    deleted_lines += 1;

                                    Colour::Red.paint(content_text).to_string()
                                }
                                _ => Colour::White.paint(content_text).to_string(),
                            };

                            content.push(format!(
                                "{} {colorized_content}",
                                colorize_origin(line.origin()),
                            ));
                            current_delta = Some(Delta::Modified);
                        }
                        Delta::Renamed | Delta::Typechange => {
                            content.push(format!(
                                "{} {}",
                                colorize_origin(line.origin()),
                                Colour::Green.paint(content_text)
                            ));
                            current_delta = Some(Delta::Renamed);
                        }
                        _ => {}
                    }
                }
            }

            file_mode = if delta.old_file().mode() != delta.new_file().mode() {
                format!(
                    "{} ==> {}",
                    Colour::Red
                        .bold()
                        .paint(format!("{:?}", delta.old_file().mode())),
                    Colour::Green
                        .bold()
                        .paint(format!("{:?}", delta.new_file().mode()))
                )
            } else {
                format!("{:?}", delta.old_file().mode())
            };
            new_old_oids = format!(
                "{}..{}",
                &(delta.old_file().id().to_string())[..7],
                &(delta.new_file().id().to_string())[..7]
            );
        } else {
            if !content.is_empty() {
                let formatted_filename = get_formatted_filename(
                    added_lines,
                    deleted_lines,
                    current_delta.clone(),
                    file_mode.clone(),
                    filename.clone(),
                    new_file.clone(),
                    new_old_oids.clone(),
                    old_file.clone(),
                );

                if let Some(target_items) = &stripped_items {
                    if target_items.contains(&old_file) || target_items.contains(&new_file) {
                        formatted_diffs.push((formatted_filename, content.join("").to_string()));
                    }
                } else {
                    formatted_diffs.push((formatted_filename, content.join("").to_string()));
                }

                let (new_filename, old_filename) = get_new_old_filenames(&delta);
                new_file = new_filename;
                old_file = old_filename;

                filename = if old_file == new_file {
                    new_file.clone()
                } else {
                    format!("{old_file} ==> {new_file}")
                };

                content.clear();
                filename.clear();
            }

            added_lines = 0;
            deleted_lines = 0;

            current_delta = None;

            file_mode.clear();
            new_old_oids.clear();
        }

        true
    })?;

    if !content.is_empty() {
        let formatted_filename = get_formatted_filename(
            added_lines,
            deleted_lines,
            current_delta,
            file_mode,
            filename,
            new_file.clone(),
            new_old_oids,
            old_file.clone(),
        );

        if let Some(target_items) = &stripped_items {
            if target_items.contains(&old_file) || target_items.contains(&new_file) {
                formatted_diffs.push((formatted_filename, content.join("").to_string()));
            }
        } else {
            formatted_diffs.push((formatted_filename, content.join("").to_string()));
        }
    }

    Ok(formatted_diffs)
}

/// Get the new and old filenames from the DiffDelta.
fn get_new_old_filenames(delta: &DiffDelta) -> (String, String) {
    (
        delta
            .new_file()
            .path()
            .unwrap_or(Path::new("?"))
            .to_str()
            .unwrap_or("?")
            .to_string(),
        delta
            .old_file()
            .path()
            .unwrap_or(Path::new("?"))
            .to_str()
            .unwrap_or("?")
            .to_string(),
    )
}

/// Get the formatted filename for a Git diff.
fn get_formatted_filename(
    added_lines: u32,
    deleted_lines: u32,
    current_delta: Option<Delta>,
    file_mode: String,
    filename: String,
    new_file: String,
    new_old_oids: String,
    old_file: String,
) -> String {
    let added = Colour::Green.bold().paint(format!(
        "+{} line{plurality}",
        added_lines,
        plurality = if added_lines != 1 { "s" } else { "" }
    ));
    let deleted = Colour::Red.bold().paint(format!(
        "-{} line{plurality}",
        deleted_lines,
        plurality = if deleted_lines != 1 { "s" } else { "" }
    ));

    if let Some(delta) = current_delta {
        match delta {
            Delta::Added => {
                format!(
                    "| {} | {} | {added} | {} | {} |",
                    filename,
                    Colour::Green.bold().paint("ADDED"),
                    new_old_oids,
                    file_mode
                )
            }
            Delta::Conflicted => {
                format!(
                    "| {} | {} | {} | {} |",
                    filename,
                    Colour::Red.bold().paint("CONFLICTED"),
                    new_old_oids,
                    file_mode
                )
            }
            Delta::Deleted => {
                format!(
                    "| {} | {} | {deleted} | {} | {} |",
                    filename,
                    Colour::Red.bold().paint("DELETED"),
                    new_old_oids,
                    file_mode
                )
            }
            Delta::Modified => {
                format!(
                    "| {} | {} | {added} | {deleted} | {} | {} |",
                    filename,
                    Colour::Fixed(172).bold().paint("MODIFIED"),
                    new_old_oids,
                    file_mode
                )
            }
            Delta::Renamed | Delta::Typechange => {
                format!(
                    "| {} | {added} | {deleted} | {} | {} |",
                    format!("{} ==> {}", old_file, new_file),
                    new_old_oids,
                    file_mode
                )
            }
            _ => {
                format!(
                    "| {} | {added} | {deleted} | {} | {} |",
                    filename, new_old_oids, file_mode
                )
            }
        }
    } else {
        format!(
            "| {} | {added} | {deleted} | {} | {} |",
            filename, new_old_oids, file_mode
        )
    }
}

/// Get Git diff statistics by comparing the HEAD and index.
pub fn get_diff_stats(
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
