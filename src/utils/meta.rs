//! Retrieving metadata for files.

use ansi_term::Colour;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use ignore::DirEntry;
use unix_mode::to_string;
use users::{get_group_by_gid, get_user_by_uid};

#[cfg(target_family = "unix")]
use std::os::unix::{fs::PermissionsExt, prelude::MetadataExt};
#[cfg(target_family = "windows")]
use std::os::windows::{fs::PermissionsExt, prelude::MetadataExt};

/// Convert a UNIX timestamp to a readable format.
fn convert_time(timestamp: i64) -> String {
    let utc_time = Utc.timestamp(timestamp, 0);
    let local_time: DateTime<Local> = DateTime::from(utc_time);

    local_time.format("%a %b %e %H:%M:%S %Y").to_string()
}

/// Convert bytes to different units depending on size.
///
/// Petabyte is the largest unit of data that may be converted. Otherwise, file
/// sizes will be displayed in bytes.
fn convert_bytes(bytes: u64) -> String {
    let (convert_by, label) = match bytes {
        1000..=999999 => (1000 as i64, "KB"),
        1000000..=9999999 => (1000000 as i64, "MB"),
        1000000000..=999999999999 => (1000000000 as i64, "GB"),
        1000000000000..=999999999999999 => (1000000000000 as i64, "TB"),
        1000000000000000..=999999999999999999 => (1000000000000000 as i64, "PB"),
        _ => (1, "B"),
    };

    let rounded_size = ((bytes as f64 / convert_by as f64) * 100.0).round() / 100.0;

    let final_number = if rounded_size.fract() == 0.0 {
        let int = rounded_size.round() as i64;
        format!("{int:>4}")
    } else {
        format!("{:>4}", format!("{:.1}", rounded_size))
    };

    format!("{final_number} {label:<2}")
}

/// Colorize the permission bits for a file.
fn colorize_permission_bits(permissions: String) -> String {
    let mut colored_chars: Vec<String> = vec![];

    for character in permissions.chars() {
        colored_chars.push(match character {
            'd' => Colour::Blue.paint(format!("{character}")).to_string(),
            'r' => Colour::Yellow.paint(format!("{character}")).to_string(),
            's' | 'T' => Colour::Purple.paint(format!("{character}")).to_string(),
            'w' => Colour::Fixed(172).paint(format!("{character}")).to_string(), // Orange.
            'x' => Colour::Red.paint(format!("{character}")).to_string(),
            _ => Colour::White
                .dimmed()
                .paint(format!("{character}"))
                .to_string(),
        })
    }

    colored_chars.into_iter().collect::<String>()
}

/// Get the metadata for a directory or file.
///
/// This is only compiled when on UNIX systems.
#[cfg(target_family = "unix")]
pub fn get_metadata(item: &DirEntry) -> String {
    let metadata = item
        .metadata()
        .expect("Could not retrieve metadata for a directory item!");

    let group = Colour::Fixed(193).paint(format!(
        "{}",
        get_group_by_gid(metadata.gid())
            .expect("None")
            .name()
            .to_str()
            .expect("None")
            .to_string()
    ));
    let mode = colorize_permission_bits(to_string(metadata.permissions().mode()));
    let last_modified = Colour::Fixed(035).paint(format!("{}", convert_time(metadata.mtime())));
    let size = convert_bytes(metadata.size());
    let user = Colour::Fixed(194).paint(format!(
        "{}",
        get_user_by_uid(metadata.uid())
            .expect("None")
            .name()
            .to_str()
            .expect("None")
            .to_string()
    ));

    format!("{mode} {user} {group} {size} {last_modified}")
}

/// Get the metadata for a directory or file.
/// This function merely returns an empty string.
///
/// This is only compiled when on Windows systems.
#[cfg(target_family = "windows")]
pub fn get_metadata(item: &dirEntry) -> String {
    "".to_string()
}
