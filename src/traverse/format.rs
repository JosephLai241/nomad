//! Formatting items in the tree.

use crate::{
    cli::Args,
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
pub fn format_directory(args: &Args, label: Option<String>, item: &Path) -> String {
    let icon = "\u{f115}".to_string(); // 
    let metadata = get_metadata(args, item);

    let directory_label = if item.is_symlink() {
        paint_symlink(item)
    } else if args.plain || args.no_colors {
        get_filename(item)
    } else {
        paint_directory(item)
    };

    let mut formatted = if args.no_icons || args.plain {
        format!("{directory_label}")
    } else {
        format!("{icon} {directory_label}")
    };

    if let Some(label) = label {
        formatted = format!("[{label}] {formatted}");
    }

    if args.metadata {
        return format!("{metadata} {formatted}");
    }

    formatted
}

/// Format how directory contents are displayed in the tree.
pub fn format_content(
    args: &Args,
    git_marker: Option<String>,
    icon: String,
    item: &Path,
    matched: Option<(usize, usize)>,
    nomad_style: &NomadStyle,
    number: Option<i32>,
) -> String {
    let mut filename = get_filename(item);
    let metadata = get_metadata(args, item);

    let mut item_string = if let (Some(marker), false) = (git_marker, args.no_git || args.plain) {
        if args.no_colors {
            if args.no_icons {
                format!("{marker} {filename}")
            } else {
                format!("{marker} {icon} {filename}")
            }
        } else {
            let formatted_filename = paint_git_item(&filename, &marker, nomad_style, matched);

            if args.no_icons {
                format!("{marker} {formatted_filename}")
            } else {
                format!("{marker} {icon} {formatted_filename}")
            }
        }
    } else {
        if args.no_icons || args.plain {
            format!("{filename}")
        } else if args.no_colors {
            format!("{icon} {filename}")
        } else {
            filename = if let Some(ranges) = matched {
                highlight_matched(filename, nomad_style, ranges)
            } else {
                filename
            };
            format!("{icon} {filename}")
        }
    };

    if let Some(number) = number {
        item_string = format!("[{number}] {item_string}");
    }
    if args.metadata {
        item_string = format!("{metadata} {item_string}")
    }

    item_string
}

/// Reformat the filename if a pattern was provided and matched.
pub fn highlight_matched(
    filename: String,
    nomad_style: &NomadStyle,
    ranges: (usize, usize),
) -> String {
    let matched_section = nomad_style
        .regex
        .match_color
        .paint(format!("{}", filename[ranges.0..ranges.1].to_string()))
        .to_string();

    filename.replace(&filename[ranges.0..ranges.1].to_string(), &matched_section)
}

/// Format how the branch looks depending on its metadata.
pub fn format_branch(
    item: &TransformedBranch,
    nomad_style: &NomadStyle,
    number: Option<i32>,
) -> String {
    let mut branch_name = Path::new(&item.full_branch)
        .file_name()
        .unwrap_or(&OsStr::new("?"))
        .to_str()
        .unwrap_or("?")
        .to_string();

    if item.is_current_branch {
        branch_name = Colour::Green
            .bold()
            .paint(format!("{branch_name}"))
            .to_string();
    }

    if let Some(ranges) = item.matched {
        branch_name = highlight_matched(branch_name, nomad_style, ranges);
    }

    if let Some(marker) = &item.marker {
        branch_name = format!("{marker} {branch_name}");
    }
    if let Some(number) = number {
        branch_name = format!("[{number}] {branch_name}");
    }
    if item.is_head {
        branch_name.push_str(&format!(" [{}]", Colour::Red.bold().paint("HEAD")));
    }
    if let Some(upstream) = &item.upstream {
        branch_name.push_str(&upstream);
    }

    branch_name
}
