//! Retrieving metadata for files.

use crate::cli::Args;

use ansi_term::Colour;
use chrono::{DateTime, Local, TimeZone, Utc};
use unix_mode::to_string;
use users::{get_group_by_gid, get_user_by_uid};

use std::path::Path;

#[cfg(target_family = "unix")]
use std::os::unix::fs::{MetadataExt, PermissionsExt};
#[cfg(target_family = "windows")]
use std::os::windows::fs::MetadataExt;

/// Convert a UNIX timestamp to a readable format.
pub fn convert_time(timestamp: i64) -> String {
    let utc_time = Utc.timestamp(timestamp, 0);
    let local_time: DateTime<Local> = DateTime::from(utc_time);

    local_time.format("%a %b %e %H:%M:%S %Y").to_string()
}

/// Convert bytes to different units depending on size.
///
/// Petabyte is the largest unit of data that may be converted. Otherwise, file
/// sizes will be displayed in bytes.
fn convert_bytes(bytes: i64) -> String {
    let (convert_by, label): (i64, &str) = match bytes {
        1000..=999999 => (1000, "KB"),
        1000000..=9999999 => (1000000, "MB"),
        1000000000..=999999999999 => (1000000000, "GB"),
        1000000000000..=999999999999999 => (1000000000000, "TB"),
        1000000000000000..=999999999999999999 => (1000000000000000, "PB"),
        _ => (1, "B"),
    };

    let rounded_size = ((bytes as f64 / convert_by as f64) * 100.0).round() / 100.0;

    let final_number = if rounded_size.fract() == 0.0 {
        let int = rounded_size.round() as i64;
        format!("{int:>3}")
    } else {
        if rounded_size > 10.0 {
            let floored_number = rounded_size.floor() as i64;
            format!("{floored_number:>3}")
        } else {
            format!("{:>3}", format!("{:.1}", rounded_size))
        }
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
            's' | 'S' => Colour::Purple.paint(format!("{character}")).to_string(),
            't' | 'T' => Colour::Purple.paint(format!("{character}")).to_string(),
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
pub fn get_metadata(args: &Args, item: &Path) -> String {
    let metadata = item.metadata().ok();

    if let Some(metadata) = metadata {
        let plain_group = format!(
            "{}",
            get_group_by_gid(metadata.gid())
                .expect("None")
                .name()
                .to_str()
                .expect("None")
                .to_string()
        );
        let group = if args.plain || args.no_colors {
            plain_group
        } else {
            Colour::Fixed(193)
                .paint(format!(
                    "{}",
                    get_group_by_gid(metadata.gid())
                        .expect("None")
                        .name()
                        .to_str()
                        .expect("None")
                        .to_string()
                ))
                .to_string()
        };

        let plain_mode = to_string(metadata.permissions().mode());
        let mode = if args.plain || args.no_colors {
            plain_mode
        } else {
            colorize_permission_bits(plain_mode)
        };

        let plain_last_modified = format!("{}", convert_time(metadata.mtime()));
        let last_modified = if args.plain || args.no_colors {
            plain_last_modified
        } else {
            Colour::Fixed(035).paint(plain_last_modified).to_string()
        };

        let plain_size = i64::try_from(metadata.size())
            .map_or("unknown file size".to_string(), |converted_bytes| {
                convert_bytes(converted_bytes)
            });
        let size = if args.plain || args.no_colors {
            plain_size
        } else {
            Colour::Fixed(172).paint(plain_size).to_string()
        };

        let plain_user = format!(
            "{}",
            get_user_by_uid(metadata.uid())
                .expect("None")
                .name()
                .to_str()
                .expect("None")
                .to_string()
        );
        let user = if args.plain || args.no_colors {
            plain_user
        } else {
            Colour::Fixed(194).paint(plain_user).to_string()
        };

        format!("{mode} {user} {group} {size} {last_modified}")
    } else {
        let missing_message = "-- No metadata available for this item --";

        if args.plain || args.no_colors {
            missing_message.to_string()
        } else {
            Colour::Red
                .bold()
                .paint("-- No metadata available for this item --")
                .to_string()
        }
    }
}

/// Get the metadata for a directory or file.
///
/// This is only compiled when on Windows systems.
#[cfg(target_family = "windows")]
pub fn get_metadata(args: &Args, item: &Path) -> String {
    let metadata = item.metadata().ok();

    if let Some(metadata) = metadata {
        let plain_file_attributes = match metadata.file_attributes() {
            1 => "FILE_ATTRIBUTE_READONLY",
            2 => "FILE_ATTRIBUTE_HIDDEN",
            4 => "FILE_ATTRIBUTE_SYSTEM",
            16 => "FILE_ATTRIBUTE_DIRECTORY",
            32 => "FILE_ATTRIBUTE_ARCHIVE",
            64 => "FILE_ATTRIBUTE_DEVICE",
            128 => "FILE_ATTRIBUTE_NORMAL",
            256 => "FILE_ATTRIBUTE_TEMPORARY",
            512 => "FILE_ATTRIBUTE_SPARSE_FILE",
            1024 => "FILE_ATTRIBUTE_REPARSE_POINT",
            2048 => "FILE_ATTRIBUTE_COMPRESSED",
            4096 => "FILE_ATTRIBUTE_OFFLINE",
            8192 => "FILE_ATTRIBUTE_NOT_CONTENT_INDEXED",
            16384 => "FILE_ATTRIBUTE_ENCRYPTED",
            32768 => "FILE_ATTRIBUTE_INTEGRITY_STREAM",
            65536 => "FILE_ATTRIBUTE_VIRTUAL",
            131072 => "FILE_ATTRIBUTE_NO_SCRUB_DATA",
            262144 => "FILE_ATTRIBUTE_RECALL_ON_OPEN",
            4194304 => "FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS",
        };
        let file_attributes = if args.plain || args.no_colors {
            plain_file_attributes.to_string()
        } else {
            Colour::Fixed(193)
                .paint(format!("{}", plain_file_attributes))
                .to_string()
        };

        let plain_last_modified = i64::try_from(metadata.last_write_time()).map_or(
            "unknown last modified time".to_string(),
            |converted_value| convert_time(converted_value),
        );
        let last_modified = if args.plain || args.no_colors {
            plain_last_modified
        } else {
            Colour::Fixed(035).paint(plain_last_modified).to_string()
        };

        let plain_size = i64::try_from(metadata.file_size())
            .map_or("unknown file size".to_string(), |converted_bytes| {
                convert_bytes(converted_bytes)
            });
        let size = if args.plain || args.no_colors {
            plain_size
        } else {
            Colour::Fixed(172).paint(plain_size).to_string()
        };

        format!("{file_attributes} {last_modified} {size}")
    } else {
        Colour::Red
            .bold()
            .paint("-- No metadata available for this item --")
            .to_string()
    }
}
