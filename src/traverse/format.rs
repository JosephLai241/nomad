//! Formatting items in the tree.

use crate::{
    cli::global::GlobalArgs,
    git::utils::paint_git_item,
    style::models::NomadStyle,
    utils::{
        meta::get_metadata,
        paint::{paint_directory, paint_symlink},
        paths::get_filename,
    },
};

use ansi_term::Colour;

use std::{ffi::OsStr, path::Path};

use super::models::TransformedBranch;

/// Format how directories are displayed in the tree.
pub fn format_directory(
    args: &GlobalArgs,
    item: &Path,
    label: Option<String>,
    matched: Option<(usize, usize)>,
    nomad_style: &NomadStyle,
    target_directory: &str,
) -> String {
    let icon = "\u{f115}".to_string(); // 
    let metadata = get_metadata(args, item);

    let directory_label = if item.is_symlink() {
        paint_symlink(item)
    } else if args.style.plain || args.style.no_colors {
        get_filename(item)
    } else {
        match matched {
            Some(ranges) => Colour::Blue
                .bold()
                .paint(highlight_matched(
                    true,
                    nomad_style,
                    item.strip_prefix(target_directory)
                        .unwrap_or_else(|_| Path::new("?"))
                        .to_str()
                        .unwrap_or("?")
                        .to_string(),
                    ranges,
                ))
                .to_string(),
            None => paint_directory(item),
        }
    };

    let mut formatted = if args.style.no_icons || args.style.plain {
        directory_label
    } else {
        format!("{icon} {directory_label}")
    };

    if let Some(label) = label {
        formatted = format!(
            "[{}] {formatted}",
            nomad_style.tree.label_colors.directory_labels.paint(label)
        );
    }

    if args.meta.metadata {
        return format!("{metadata} {formatted}");
    }

    formatted
}

/// Format how directory contents are displayed in the tree.
pub fn format_content(
    args: &GlobalArgs,
    git_marker: Option<String>,
    icon: String,
    item: &Path,
    matched: Option<(usize, usize)>,
    nomad_style: &NomadStyle,
    number: Option<i32>,
    target_directory: &str,
) -> String {
    let mut filename = get_filename(item);
    let metadata = get_metadata(args, item);

    let mut item_string =
        if let (Some(marker), false) = (git_marker, args.style.no_git || args.style.plain) {
            if args.style.no_colors {
                if args.style.no_icons {
                    format!("{marker} {filename}")
                } else {
                    format!("{marker} {icon} {filename}")
                }
            } else {
                let formatted_filename = paint_git_item(
                    item.strip_prefix(target_directory)
                        .unwrap_or_else(|_| Path::new("?"))
                        .to_str()
                        .unwrap_or("?"),
                    &marker,
                    nomad_style,
                    matched,
                );

                if args.style.no_icons {
                    format!("{marker} {formatted_filename}")
                } else {
                    format!("{marker} {icon} {formatted_filename}")
                }
            }
        } else if args.style.no_icons || args.style.plain {
            filename
        } else if args.style.no_colors {
            format!("{icon} {filename}")
        } else {
            filename = if let Some(ranges) = matched {
                highlight_matched(
                    false,
                    nomad_style,
                    item.strip_prefix(target_directory)
                        .unwrap_or_else(|_| Path::new("?"))
                        .to_str()
                        .unwrap_or("?")
                        .to_string(),
                    ranges,
                )
            } else {
                filename
            };

            format!("{icon} {filename}")
        };

    if let Some(number) = number {
        item_string = format!(
            "[{}] {item_string}",
            nomad_style
                .tree
                .label_colors
                .item_labels
                .paint(format!("{number}"))
        );
    }
    if args.meta.metadata {
        item_string = format!("{metadata} {item_string}")
    }

    item_string
}

/// Reformat the filename if a pattern was provided and matched.
pub fn highlight_matched(
    for_dir: bool,
    nomad_style: &NomadStyle,
    path: String,
    ranges: (usize, usize),
) -> String {
    if (0..path.len()).contains(&ranges.0) && (0..path.len() + 1).contains(&ranges.1) {
        let mut prefix = path[..ranges.0]
            .chars()
            .into_iter()
            .map(|character| {
                if for_dir {
                    Colour::Blue.bold().paint(character.to_string()).to_string()
                } else {
                    format!("{character}")
                }
            })
            .collect::<Vec<String>>();
        let mut painted_matched = path[ranges.0..ranges.1]
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
        let mut suffix = path[ranges.1..]
            .chars()
            .into_iter()
            .map(|character| {
                if for_dir {
                    Colour::Blue.bold().paint(character.to_string()).to_string()
                } else {
                    format!("{character}")
                }
            })
            .collect::<Vec<String>>();

        prefix.append(&mut painted_matched);
        prefix.append(&mut suffix);

        let highlighted_path = prefix.join("");

        Path::new(&highlighted_path)
            .file_name()
            .unwrap_or_else(|| OsStr::new("?"))
            .to_str()
            .unwrap_or("?")
            .to_string()
    } else {
        Path::new(&path)
            .file_name()
            .unwrap_or_else(|| OsStr::new("?"))
            .to_str()
            .unwrap_or("?")
            .to_string()
    }
}

/// Format how the branch looks depending on its metadata.
pub fn format_branch(
    item: &TransformedBranch,
    nomad_style: &NomadStyle,
    number: Option<i32>,
) -> String {
    let mut branch_name = Path::new(&item.full_branch)
        .file_name()
        .unwrap_or_else(|| OsStr::new("?"))
        .to_str()
        .unwrap_or("?")
        .to_string();

    if item.is_current_branch {
        branch_name = Colour::Green.bold().paint(branch_name).to_string();
    }

    if let Some(ranges) = item.matched {
        branch_name = highlight_matched(false, nomad_style, item.full_branch.to_string(), ranges);
    }

    if let Some(marker) = &item.marker {
        branch_name = format!("{marker} {branch_name}");
    }
    if let Some(number) = number {
        branch_name = format!(
            "[{}] {branch_name}",
            nomad_style
                .tree
                .label_colors
                .item_labels
                .paint(format!("{number}"))
        );
    }
    if item.is_head {
        branch_name.push_str(&format!(" [{}]", Colour::Red.bold().paint("HEAD")));
    }
    if let Some(upstream) = &item.upstream {
        branch_name.push_str(upstream);
    }

    branch_name
}
