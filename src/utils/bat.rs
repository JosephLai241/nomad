//! Run `bat`.

use crate::{errors::NomadError, git::diff::colorize_origin};

use ansi_term::Colour;
use anyhow::Result;
use bat::{Input, PagingMode, PrettyPrinter, WrappingMode};
use git2::{Delta, Diff, DiffFormat};

use std::path::Path;

/// Create a new `PrettyPrinter`, then run it against the file.
pub fn run_bat(file: String) -> Result<(), NomadError> {
    PrettyPrinter::new()
        .grid(true)
        .header(true)
        .input_file(Path::new(&file))
        .line_numbers(true)
        .paging_mode(PagingMode::QuitIfOneScreen)
        .true_color(true)
        .vcs_modification_markers(true)
        .wrapping_mode(WrappingMode::Character)
        .print()
        .map_or_else(|error| Err(NomadError::BatError(error)), |_| Ok(()))
}

/// Contains Git diff data.
struct GitDiff {
    /// The number of lines added in the file.
    added_lines: u32,
    /// The number of lines deleted in the file.
    deleted_lines: u32,
    /// The Git Delta status.
    delta: Option<Delta>,
    /// The name of the file.
    filename: String,
    /// The name of the new file.
    new_file: String,
    /// The name of the old file.
    old_file: String,
    /// The Git diff text.
    text: Vec<String>,
}

/// Display Git diffs with `bat`.
pub fn display_git_diffs<'a>(diff: Diff, file_options: Vec<String>) -> Result<(), NomadError> {
    let mut inputs: Vec<GitDiff> = Vec::new();
    let mut content: Vec<String> = Vec::new();

    let mut current_delta: Option<Delta> = None;
    let mut filename = String::new();
    let mut new_file = String::new();
    let mut old_file = String::new();

    let mut added_lines: u32 = 0;
    let mut deleted_lines: u32 = 0;

    diff.print(DiffFormat::Patch, |delta, hunk, line| {
        old_file = delta
            .old_file()
            .path()
            .unwrap_or(Path::new("?"))
            .to_str()
            .unwrap_or("?")
            .to_string()
            .clone();
        new_file = delta
            .new_file()
            .path()
            .unwrap_or(Path::new("?"))
            .to_str()
            .unwrap_or("?")
            .to_string()
            .clone();

        if filename.is_empty() {
            filename = if old_file == new_file {
                new_file.clone()
            } else {
                format!("{old_file} ==> {new_file}")
            };
        }

        println!(
            "{filename} | OLD LN: {:?} | NEW LN: {:?} | OFFSET: {}\n{}",
            line.old_lineno(),
            line.new_lineno(),
            line.content_offset(),
            std::str::from_utf8(line.content()).unwrap_or("?")
        );

        if let Some(_) = hunk {
            match delta.status() {
                Delta::Added => {
                    content.push(format!(
                        "{} {}",
                        colorize_origin(line.origin()),
                        Colour::Green.paint(std::str::from_utf8(line.content()).unwrap_or("?"))
                    ));

                    current_delta = Some(Delta::Added);
                    added_lines += 1;
                }
                Delta::Conflicted => {
                    // TODO: MAYBE IMPLEMENT THIS?
                    //      DISPLAY "CONFLICTED" IN THE HEADER AND WHATEVER USEFUL INFO FOR A
                    //      CONFLICTING GIT FILE. NOT SURE WHAT THAT WOULD BE BUT I GUESS WE'LL FUCK
                    //      AROUND AND FIND OUT.
                    content.push("CONFLICTING FILE".to_string());
                    current_delta = Some(Delta::Conflicted);
                }
                Delta::Deleted => {
                    content.push(format!(
                        "{} {}",
                        colorize_origin(line.origin()),
                        Colour::Red.paint(std::str::from_utf8(line.content()).unwrap_or("?"))
                    ));

                    current_delta = Some(Delta::Deleted);
                    deleted_lines += 1;
                }
                Delta::Modified => {
                    let colorized_content = match line.origin() {
                        '+' | '>' => {
                            added_lines += 1;

                            Colour::Green
                                .paint(std::str::from_utf8(line.content()).unwrap_or("?"))
                                .to_string()
                        }
                        '-' | '<' => {
                            deleted_lines += 1;

                            Colour::Red
                                .paint(std::str::from_utf8(line.content()).unwrap_or("?"))
                                .to_string()
                        }
                        _ => Colour::White
                            .paint(std::str::from_utf8(line.content()).unwrap_or("?"))
                            .to_string(),
                    };

                    content.push(format!(
                        "{} {}",
                        colorize_origin(line.origin()),
                        colorized_content
                    ));
                    current_delta = Some(Delta::Modified);
                }
                Delta::Renamed | Delta::Typechange => {
                    content.push(format!(
                        "{} {}",
                        colorize_origin(line.origin()),
                        Colour::Green.paint(std::str::from_utf8(line.content()).unwrap_or("?"))
                    ));
                    current_delta = Some(Delta::Renamed);
                }
                _ => {}
            }
        } else {
            if !content.is_empty() {
                inputs.push(GitDiff {
                    added_lines,
                    deleted_lines,
                    delta: current_delta,
                    filename: filename.clone(),
                    new_file: new_file.to_string(),
                    old_file: old_file.to_string(),
                    text: content.to_owned(),
                });

                content.clear();
                filename.clear();

                filename = new_file.clone();
            }

            added_lines = 0;
            deleted_lines = 0;
            current_delta = None;
        }

        true
    })?;

    if !content.is_empty() {
        inputs.push(GitDiff {
            added_lines,
            deleted_lines,
            delta: current_delta,
            filename: filename.clone(),
            new_file: new_file.to_string(),
            old_file: old_file.to_string(),
            text: content.to_owned(),
        });
    }

    let mut _joined_text = String::new();

    for diff in inputs {
        let formatted_filename = if let Some(delta) = diff.delta {
            match delta {
                Delta::Added => {
                    format!(
                        " {} {} | {}",
                        diff.filename,
                        Colour::Green.bold().paint("[ADDED]"),
                        Colour::Green
                            .bold()
                            .paint(format!("+{} lines", diff.added_lines))
                    )
                }
                Delta::Conflicted => diff.filename.clone(),
                Delta::Deleted => {
                    format!(
                        " {} {} | {}",
                        diff.filename,
                        Colour::Red.bold().paint("[DELETED]"),
                        Colour::Red
                            .bold()
                            .paint(format!("-{} lines", diff.deleted_lines))
                    )
                }
                Delta::Modified => {
                    format!(
                        " {} {} | {} | {}",
                        diff.filename,
                        Colour::Fixed(172).bold().paint("[MODIFIED]"),
                        Colour::Green
                            .bold()
                            .paint(format!("+{} lines", diff.added_lines)),
                        Colour::Red
                            .bold()
                            .paint(format!("-{} lines", diff.deleted_lines))
                    )
                }
                Delta::Renamed | Delta::Typechange => {
                    format!(
                        "{} | {} | {}",
                        format!("{} ==> {}", diff.old_file, diff.new_file),
                        Colour::Green
                            .bold()
                            .paint(format!("+{} lines", diff.added_lines)),
                        Colour::Red
                            .bold()
                            .paint(format!("-{} lines", diff.deleted_lines))
                    )
                }
                _ => diff.filename.clone(),
            }
        } else {
            diff.filename
        };

        _joined_text = diff.text.join("").to_string();
        let input = Input::from_bytes(_joined_text.as_bytes()).name(formatted_filename);

        if let Err(error) = PrettyPrinter::new()
            .grid(true)
            .header(true)
            .input(input)
            .paging_mode(PagingMode::Never)
            .true_color(true)
            .wrapping_mode(WrappingMode::Character)
            .print()
        {
            return Err(NomadError::BatError(error));
        }
    }

    Ok(())
}
