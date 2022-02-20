//! Run `bat`.

use crate::{errors::NomadError, git::diff::colorize_origin};

use ansi_term::Colour;
use anyhow::Result;
use bat::{Input, PagingMode, PrettyPrinter, WrappingMode};
use git2::{Delta, Diff, DiffFormat};

use std::{path::Path, str::from_utf8};

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

/// Display Git diffs with `bat`.
pub fn display_git_diffs<'a>(diff: Diff, file_options: Vec<String>) -> Result<(), NomadError> {
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

                let formatted_filename = if let Some(delta) = current_delta {
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
                };

                formatted_diffs.push((formatted_filename, content.join("").to_string()));

                content.clear();
                filename.clear();

                filename = new_file.clone();
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

        let formatted_filename = if let Some(delta) = current_delta {
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
        };

        formatted_diffs.push((formatted_filename, content.join("").to_string()));
    }

    if let Err(error) = PrettyPrinter::new()
        .grid(true)
        .header(true)
        .inputs(
            formatted_diffs
                .iter()
                .map(|(filename, diff)| Input::from_bytes(diff.as_bytes()).name(filename))
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

    Ok(())
}
