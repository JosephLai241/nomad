//! Traverse the target directory while respecting rules set in ignore-type files.

use crate::{
    cli::Args,
    utils::{
        meta::get_metadata,
        temp::{create_temp_dir, get_json_file, write_to_json},
    },
};

use ansi_term::*;
use ignore::{self, DirEntry, Walk, WalkBuilder};
use ptree::{item::StringItem, print_tree_with, Color, PrintConfig, Style, TreeBuilder};
use serde_json::json;

use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::read_link,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    time::Instant,
};

/// Build a `Walk` object based on the client's CLI parameters.
pub fn build_walker(args: &Args, target_directory: &str) -> Result<Walk, Error> {
    if Path::new(target_directory).is_dir() {
        Ok(WalkBuilder::new(target_directory)
            .follow_links(true)
            .git_exclude(!args.disrespect)
            .git_global(!args.disrespect)
            .git_ignore(!args.disrespect)
            .ignore(!args.disrespect)
            .hidden(!args.hidden)
            .parents(!args.disrespect)
            .sort_by_file_path(|a, b| a.cmp(b))
            .build())
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("Directory '{target_directory}/' does not exist!"),
        ))
    }
}

//
// TODO: MOVE THIS TO THE UTILS FOLDER?
//
/// Get the absolute path of a directory.
pub fn canonicalize_path(target_directory: &str) -> Result<String, Error> {
    PathBuf::from(target_directory)
        .canonicalize()?
        .into_os_string()
        .into_string()
        .map_or(
            Err(Error::new(ErrorKind::Other, "Could not canonicalize path!")),
            |path| Ok(path),
        )
}

/// Get the file's corresponding icon.
fn get_file_icon(
    extension_icon_map: &HashMap<&str, &str>,
    item: &DirEntry,
    name_icon_map: &HashMap<&str, &str>,
) -> String {
    let icon = extension_icon_map
        .get(
            item.path()
                .extension()
                .unwrap_or(OsStr::new("none"))
                .to_str()
                .unwrap(),
        )
        .map_or_else(
            || {
                name_icon_map
                    .get(&item.file_name().to_str().unwrap())
                    .unwrap_or(&&"\u{f016}") // 
            },
            |icon| icon,
        );

    icon.to_string()
}

/// Format how the item will be displayed in the tree.
fn format_item(
    icon: String,
    is_directory: bool,
    item: &DirEntry,
    include_metadata: bool,
    number: Option<i32>,
) -> String {
    let formatted_item = item
        .path()
        .file_name()
        .unwrap_or(OsStr::new("?"))
        .to_str()
        .unwrap_or("?");

    let metadata = get_metadata(item);

    if is_directory {
        let directory_label = if item.path_is_symlink() {
            let points_to = read_link(item.path()).map_or("?".to_string(), |pathbuf_path| {
                pathbuf_path
                    .canonicalize()
                    .unwrap_or(PathBuf::from("?"))
                    .into_os_string()
                    .into_string()
                    .map_or("?".to_string(), |path_string| path_string)
            });

            Colour::Yellow
                .bold()
                .paint(format!("{formatted_item} ⇒ {points_to}"))
        } else {
            let directory_name = item
                .path()
                .file_name()
                .unwrap_or(OsStr::new("?"))
                .to_str()
                .unwrap_or("?")
                .to_string();

            Colour::Blue.bold().paint(format!("{directory_name}"))
        };

        if include_metadata {
            format!("{metadata} {icon} {directory_label}")
        } else {
            format!("{icon} {directory_label}")
        }
    } else {
        let mut item_string = format!("{icon} {formatted_item}");

        if let Some(number) = number {
            item_string = format!("[{number}] {item_string}");
        }
        if include_metadata {
            item_string = format!("{metadata} {item_string}")
        }

        item_string
    }
}

/// Build a `ptree` object and set the tree's style/configuration.
fn build_tree(target_directory: &str) -> (TreeBuilder, PrintConfig) {
    let directory_icon = &"\u{f115}"; // 
    let directory_name = Colour::Blue.bold().paint(
        PathBuf::from(target_directory)
            .canonicalize()
            .unwrap_or(PathBuf::from("?"))
            .file_name()
            .unwrap_or(OsStr::new("?"))
            .to_str()
            .unwrap_or("?")
            .to_string(),
    );

    let tree = TreeBuilder::new(format!("{directory_icon} {directory_name}"));

    let mut branch_style = Style::default();
    branch_style.bold = true;
    branch_style.foreground = Some(Color::White);

    let mut config = PrintConfig::default();
    config.branch = branch_style;
    config.indent = 4;

    (tree, config)
}

/// Write the directory's contents to a temporary file.
pub fn store_directory_contents(items: Vec<Vec<String>>) -> Result<(), Error> {
    create_temp_dir()?;

    let mut json = json!({ "items": {} });
    for item in items {
        json["items"]
            .as_object_mut()
            .unwrap()
            .insert(item[0].clone(), json!(item[1]));
    }

    let mut json_file = get_json_file(false)?;
    write_to_json(&mut json_file, json)?;

    Ok(())
}

/// Traverse the directory and display files and directories accordingly.
pub fn walk_directory(
    args: &Args,
    extension_icon_map: &HashMap<&str, &str>,
    name_icon_map: &HashMap<&str, &str>,
    target_directory: &str,
    walker: &mut Walk,
) -> Result<Option<(StringItem, PrintConfig)>, Error> {
    let mut current_depth: usize = 0;
    let (mut tree, config) = build_tree(target_directory);

    println!();

    let mut num_directories = 0;
    let mut num_files = 0;
    let mut previous_item = walker
        .next() // Sets the first `previous_item` to the `target_directory`.
        .expect("No items were found in this directory!")
        .unwrap_or_else(|error| panic!("Could not retrieve items in this directory! {error}"));
    let mut items: Vec<Vec<String>> = Vec::new();

    let start = Instant::now();
    while let Some(Ok(item)) = walker.next() {
        if item.depth() < current_depth {
            if previous_item.path().is_dir() {
                let item_parent = item
                    .path()
                    .parent()
                    .expect("Could not get the current item's parent!");
                let previous_parent = previous_item
                    .path()
                    .parent()
                    .expect("Could not get the previous item's parent!");

                if item_parent != previous_parent {
                    tree.end_child();
                }
            }

            for _ in 0..current_depth - item.depth() {
                tree.end_child();
            }
        } else if item.depth() == current_depth && previous_item.path().is_dir() {
            tree.end_child();
        }

        if item.path().is_dir() {
            let icon = "\u{f115}".to_string(); // 
            tree.begin_child(format_item(icon, true, &item, args.metadata, None));

            num_directories += 1;
        } else if item.path().is_file() {
            let number = if args.numbers {
                items.push(vec![
                    num_files.to_string(),
                    item.path()
                        .canonicalize()
                        .unwrap_or(PathBuf::from("?"))
                        .into_os_string()
                        .into_string()
                        .unwrap_or("?".into()),
                ]);

                Some(num_files)
            } else {
                None
            };

            let icon = get_file_icon(extension_icon_map, &item, name_icon_map);
            tree.add_empty_child(format_item(icon, false, &item, args.metadata, number));

            num_files += 1;
        }

        current_depth = item.depth();
        previous_item = item;
    }

    if args.numbers {
        store_directory_contents(items)?;
    }

    if args.export.is_some() {
        return Ok(Some((tree.build(), config)));
    } else {
        print_tree_with(&tree.build(), &config)?;
    }

    println!();

    if args.statistics {
        let duration = start.elapsed().as_millis();
        println!("{num_directories} directories | {num_files} files | {duration} ms\n");
    }

    Ok(None)
}
