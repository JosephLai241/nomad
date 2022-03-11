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

use std::path::Path;

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
            format!("{marker} {icon} {filename}")
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
        .match_color
        .paint(format!("{}", filename[ranges.0..ranges.1].to_string()))
        .to_string();

    filename.replace(&filename[ranges.0..ranges.1].to_string(), &matched_section)
}
