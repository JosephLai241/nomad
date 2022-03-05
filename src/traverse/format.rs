//! Formatting items in the tree.

use crate::{
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
pub fn format_directory(
    label: Option<String>,
    item: &Path,
    include_metadata: bool,
    mute_icons: bool,
    plain: bool,
) -> String {
    let icon = "\u{f115}".to_string(); // 
    let metadata = get_metadata(item, plain);

    let directory_label = if item.is_symlink() {
        paint_symlink(item)
    } else if plain {
        get_filename(item)
    } else {
        paint_directory(item)
    };

    let mut formatted = if mute_icons || plain {
        format!("{directory_label}")
    } else {
        format!("{icon} {directory_label}")
    };

    if let Some(label) = label {
        formatted = format!("[{label}] {formatted}");
    }

    if include_metadata {
        return format!("{metadata} {formatted}");
    }

    formatted
}

/// Format how directory contents are displayed in the tree.
pub fn format_content(
    git_marker: Option<String>,
    icon: String,
    item: &Path,
    include_metadata: bool,
    matched: Option<(usize, usize)>,
    mute_git: bool,
    mute_icons: bool,
    nomad_style: &NomadStyle,
    number: Option<i32>,
    plain: bool,
) -> String {
    let mut filename = get_filename(item);
    let metadata = get_metadata(item, plain);

    let mut item_string = if let (Some(marker), false, false) = (git_marker, mute_git, plain) {
        let formatted_filename = paint_git_item(&filename, &marker, nomad_style, matched);

        if mute_icons {
            format!("{marker} {formatted_filename}")
        } else {
            format!("{marker} {icon} {formatted_filename}")
        }
    } else {
        if mute_icons || plain {
            format!("{filename}")
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
    if include_metadata {
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
