//! Exposing functionality for the Git diff command.

use std::{ffi::OsStr, path::Path, str::from_utf8};

use ansi_term::Colour;
use anyhow::Result;
use bat::{Input, PagingMode, PrettyPrinter, WrappingMode};
use git2::{Delta, Diff, DiffDelta, DiffFormat, Error, Index, ObjectType, Repository, Tree};
use lazy_static::lazy_static;
use syntect::{
    easy::HighlightLines,
    highlighting::{Color, Style, StyleModifier},
    util::as_24_bit_terminal_escaped,
};

use crate::{errors::NomadError, SYNTAX_SET, THEME_SET};

lazy_static! {
    /// A dark green color to indicate added lines.
    static ref GREEN: Color = Color {
        r: 030,
        g: 052,
        b: 024,
        a: 0x1A,
    };
    /// A dark orange color to indicate conflicts in conflicting files.
    static ref ORANGE: Color = Color {
        r: 107,
        g: 068,
        b: 000,
        a: 0x1A,
    };
    /// A dark red color to indicate deleted lines.
    static ref RED: Color = Color {
        r: 075,
        g: 022,
        b: 022,
        a: 0x1A,
    };
}

/// Get the diff between the old Git tree and the working directory using the Git index.
pub fn get_repo_diffs(repo: &Repository) -> Result<Diff<'_>, Error> {
    let previous_head = repo.head()?.peel(ObjectType::Tree)?.id();
    let old_tree = repo.find_tree(previous_head)?;

    let diff = repo.diff_tree_to_workdir_with_index(Some(&old_tree), None)?;

    Ok(diff)
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
                            .unwrap_or_else(|_| Path::new("?"))
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
        let old_syntax = SYNTAX_SET
            .find_syntax_by_extension(
                delta
                    .old_file()
                    .path()
                    .unwrap_or_else(|| Path::new("?"))
                    .extension()
                    .unwrap_or_else(|| OsStr::new("?"))
                    .to_str()
                    .unwrap_or("?"),
            )
            .unwrap_or(SYNTAX_SET.find_syntax_plain_text());
        let new_syntax = SYNTAX_SET
            .find_syntax_by_extension(
                delta
                    .new_file()
                    .path()
                    .unwrap_or_else(|| Path::new("?"))
                    .extension()
                    .unwrap_or_else(|| OsStr::new("?"))
                    .to_str()
                    .unwrap_or("?"),
            )
            .unwrap_or(SYNTAX_SET.find_syntax_plain_text());
        let mut old_highlighter =
            HighlightLines::new(old_syntax, &THEME_SET.themes["base16-eighties.dark"]);
        let mut new_highlighter =
            HighlightLines::new(new_syntax, &THEME_SET.themes["base16-eighties.dark"]);

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

                    let highlighted_line = match delta.status() {
                        Delta::Added => {
                            current_delta = Some(Delta::Added);
                            added_lines += 1;

                            highlight_line(Some(*GREEN), content_text, &mut new_highlighter, true)
                        }
                        Delta::Conflicted => {
                            current_delta = Some(Delta::Conflicted);

                            highlight_line(Some(*ORANGE), content_text, &mut old_highlighter, true)
                        }
                        Delta::Deleted => {
                            current_delta = Some(Delta::Deleted);
                            deleted_lines += 1;

                            highlight_line(Some(*RED), content_text, &mut old_highlighter, true)
                        }
                        Delta::Modified => {
                            let (background_color, mut modified_highlighter, paint_background) =
                                match line.origin() {
                                    '+' | '>' => {
                                        added_lines += 1;

                                        (Some(*GREEN), new_highlighter, true)
                                    }
                                    '-' | '<' => {
                                        deleted_lines += 1;

                                        (Some(*RED), old_highlighter, true)
                                    }
                                    _ => (None, old_highlighter, false),
                                };

                            current_delta = Some(Delta::Modified);

                            highlight_line(
                                background_color,
                                content_text,
                                &mut modified_highlighter,
                                paint_background,
                            )
                        }
                        Delta::Renamed | Delta::Typechange => {
                            current_delta = Some(Delta::Renamed);

                            highlight_line(Some(*GREEN), content_text, &mut new_highlighter, true)
                        }
                        _ => highlight_line(None, content_text, &mut old_highlighter, false),
                    };

                    content.push(format!(
                        "{} {highlighted_line}",
                        colorize_origin(line.origin())
                    ))
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
                    current_delta,
                    file_mode.clone(),
                    filename.clone(),
                    new_file.clone(),
                    new_old_oids.clone(),
                    old_file.clone(),
                );

                if let Some(target_items) = &stripped_items {
                    if target_items.contains(&old_file) || target_items.contains(&new_file) {
                        formatted_diffs.push((formatted_filename, content.join("")));
                    }
                } else {
                    formatted_diffs.push((formatted_filename, content.join("")));
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
                formatted_diffs.push((formatted_filename, content.join("")));
            }
        } else {
            formatted_diffs.push((formatted_filename, content.join("")));
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
            .unwrap_or_else(|| Path::new("?"))
            .to_str()
            .unwrap_or("?")
            .to_string(),
        delta
            .old_file()
            .path()
            .unwrap_or_else(|| Path::new("?"))
            .to_str()
            .unwrap_or("?")
            .to_string(),
    )
}

/// Colorize the origin of the `DiffLine`.
fn colorize_origin(marker: char) -> String {
    match marker {
        '+' | '>' => Colour::Green.bold().paint(format!("{marker}")).to_string(),
        '-' | '<' => Colour::Red.bold().paint(format!("{marker}")).to_string(),
        _ => Colour::White.bold().paint(format!("{marker}")).to_string(),
    }
}

/// Add syntax highlighting to a line and set its background color based on the diff status.
fn highlight_line(
    color: Option<Color>,
    content_text: &str,
    highlighter: &mut HighlightLines,
    paint_background: bool,
) -> String {
    let ranges = highlighter
        .highlight(content_text, &SYNTAX_SET)
        .iter()
        .map(|(style, line)| {
            let style = style.apply(StyleModifier {
                background: color,
                font_style: None,
                foreground: None,
            });

            (style, *line)
        })
        .collect::<Vec<(Style, &str)>>();

    format!(
        "{}\u{001b}[0m", // Have to reset the style at the end of each line, otherwise it gets applied to the next line.
        as_24_bit_terminal_escaped(&ranges[..], paint_background)
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
    if let Ok(diff) = repo.diff_tree_to_index(Some(old_tree), Some(index), None) {
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
