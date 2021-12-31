//! Traverse the target directory while respecting rules set in ignore-type files.

use crate::cli::Args;

use ansi_term::*;
use ignore::{self, DirEntry, Walk, WalkBuilder};
use ptree::{print_tree, TreeBuilder};

use std::{collections::HashMap, ffi::OsStr, io::Error, path::PathBuf, time::Instant};

/// Build a `Walk` object based on the client's CLI parameters.
pub fn build_walker(args: &Args, target_directory: &str) -> Walk {
    WalkBuilder::new(target_directory)
        .git_exclude(!args.disrespect)
        .git_global(!args.disrespect)
        .git_ignore(!args.disrespect)
        .ignore(!args.disrespect)
        .hidden(!args.hidden)
        .parents(!args.disrespect)
        .sort_by_file_path(|a, b| a.cmp(b))
        .build()
}

/// Format how the item will be displayed in the tree.
fn format_item(
    extension_icon_map: &HashMap<&str, &str>,
    is_dir: bool,
    item: &DirEntry,
    number: Option<i32>,
) -> String {
    let icon = if is_dir {
        &"\u{f115}"
    } else {
        extension_icon_map
            .get(
                item.path()
                    .extension()
                    .unwrap_or(OsStr::new("markdown"))
                    .to_str()
                    .unwrap(),
            )
            .unwrap_or(&&"\u{f016}") // 
    };

    let formatted_item = item
        .path()
        .file_name()
        .unwrap_or(OsStr::new("?"))
        .to_str()
        .unwrap_or("?");

    if is_dir {
        format!(
            "{} {}",
            icon,
            Colour::Blue.bold().paint(
                item.path()
                    .file_name()
                    .unwrap_or(OsStr::new("?"))
                    .to_str()
                    .unwrap_or("?")
            )
        )
    } else {
        let item_string = format!("{} {}", icon, formatted_item);

        if let Some(number) = number {
            let numbered_string = format!("[{}] ", number);
            numbered_string + &item_string
        } else {
            item_string
        }
    }
}

/// Traverse the directory and display files and directories accordingly.
pub fn walk_directory(
    args: &Args,
    extension_icon_map: &HashMap<&str, &str>,
    target_directory: &str,
    walker: &mut Walk,
) -> Result<(), Error> {
    let mut current_depth: usize = 0;
    let mut tree = TreeBuilder::new(format!(
        "{} {}",
        &"\u{f115}",
        Colour::Blue.bold().paint(
            PathBuf::from(target_directory)
                .canonicalize()
                .unwrap_or(PathBuf::from("?"))
                .file_name()
                .unwrap_or(OsStr::new("?"))
                .to_str()
                .unwrap_or("?")
        )
    ));
    walker.next(); // Skip the target directory.
    println!();

    let mut num_directories = 0;
    let mut num_files = 0;
    let start = Instant::now();
    while let Some(Ok(item)) = walker.next() {
        if item.depth() < current_depth {
            for _ in 0..current_depth - item.depth() {
                tree.end_child();
            }
        }

        if item.path().is_dir() {
            tree.begin_child(format_item(extension_icon_map, true, &item, None));
            num_directories += 1;
        } else if item.path().is_file() {
            let number = if args.numbers { Some(num_files) } else { None };
            tree.add_empty_child(format_item(extension_icon_map, false, &item, number));
            num_files += 1;
        }

        current_depth = item.depth();
    }

    print_tree(&tree.build())?;
    println!();

    if args.statistics {
        println!(
            "{} directories | {} files | {} ms\n",
            num_directories,
            num_files,
            start.elapsed().as_millis()
        );
    }

    Ok(())
}
