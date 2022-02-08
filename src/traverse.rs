//! Traverse the target directory while respecting rules set in ignore-type files.

use crate::{
    cli::Args,
    git::markers::extend_marker_map,
    utils::{
        meta::get_metadata,
        paint::{paint_directory, paint_symlink_directory},
        paths::{canonicalize_path, get_filename},
        temp::{create_temp_dir, get_json_file, write_to_json, JSONTarget},
    },
};

use ansi_term::*;
use ignore::{self, DirEntry, Walk, WalkBuilder};
use ptree::{item::StringItem, print_tree_with, Color, PrintConfig, Style, TreeBuilder};
use serde_json::json;

use std::{
    collections::HashMap,
    ffi::OsStr,
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
            format!("Directory '{target_directory}' does not exist!"),
        ))
    }
}

/// Get the file's corresponding icon.
pub fn get_file_icon(
    extension_icon_map: &HashMap<&str, &str>,
    item: &DirEntry,
    name_icon_map: &HashMap<&str, &str>,
) -> String {
    if let Some(icon) = extension_icon_map.get(
        item.path()
            .extension()
            .unwrap_or(OsStr::new("none"))
            .to_str()
            .unwrap(),
    ) {
        icon.to_string()
    } else {
        if let Some(icon) = name_icon_map.get(&item.file_name().to_str().unwrap()) {
            icon.to_string()
        } else {
            "\u{f016}".to_string() // 
        }
    }
}

/// Format how directories are displayed in the tree.
pub fn format_directory(
    git_marker: Option<String>,
    icon: String,
    label: Option<String>,
    item: &DirEntry,
    include_metadata: bool,
    mute_icons: bool,
) -> String {
    let metadata = get_metadata(item);

    let directory_label = if item.path_is_symlink() {
        paint_symlink_directory(item)
    } else {
        paint_directory(item)
    };

    let mut formatted = if mute_icons {
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
    item: &DirEntry,
    include_metadata: bool,
    mute_icons: bool,
    number: Option<i32>,
) -> String {
    let filename = get_filename(item);
    let metadata = get_metadata(item);

    let mut item_string = if let Some(marker) = git_marker {
        let staged_deleted = Colour::Red.bold().paint("SD").to_string();
        let staged_modified = Colour::Yellow.bold().paint("SM").to_string();
        let staged_new = Colour::Green.bold().paint("SA").to_string();
        let staged_renamed = Colour::Fixed(172).bold().paint("SR").to_string();
        let conflicted = Colour::Red.bold().paint("CONFLICT").to_string();

        let formatted_filename = match marker {
            _ if marker == staged_deleted => Colour::Red
                .bold()
                .strikethrough()
                .paint(format!("{filename}"))
                .to_string(),
            _ if marker == staged_modified => Colour::Yellow
                .bold()
                .paint(format!("{filename}"))
                .to_string(),
            _ if marker == staged_new => Colour::Green
                .bold()
                .paint(format!("{filename}"))
                .to_string(),
            _ if marker == staged_renamed => Colour::Fixed(172)
                .bold()
                .paint(format!("{filename}"))
                .to_string(),
            _ if marker == conflicted => {
                Colour::Red.bold().paint(format!("{filename}")).to_string()
            }
            _ => filename,
        };

        if mute_icons {
            format!("{marker} {formatted_filename}")
        } else {
            format!("{marker} {icon} {formatted_filename}")
        }
    } else {
        if mute_icons {
            format!("{filename}")
        } else {
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

/// Build a `ptree` object and set the tree's style/configuration.
pub fn build_tree(
    include_metadata: bool,
    target_directory: &DirEntry,
) -> (PrintConfig, TreeBuilder) {
    let directory_icon = &"\u{f115}"; // 
    let directory_name = Colour::Blue.bold().paint(
        target_directory
            .file_name()
            .to_str()
            .unwrap_or("?")
            .to_string(),
    );

    let mut tree_label = format!("{directory_icon} {directory_name}");
    if include_metadata {
        let metadata = get_metadata(target_directory);
        tree_label = format!("{metadata} {tree_label}");
    }

    let tree = TreeBuilder::new(tree_label);

    let mut branch_style = Style::default();
    branch_style.bold = true;
    branch_style.foreground = Some(Color::White);

    let mut config = PrintConfig::default();
    config.branch = branch_style;
    config.indent = 4;

    (config, tree)
}

/// Run checks to ensure tree nesting is correct. Make any corrections if applicable.
pub fn check_nesting(
    current_depth: usize,
    item: &DirEntry,
    previous_item: DirEntry,
    tree: &mut TreeBuilder,
) {
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
}

/// Write the labeled directories or numbered directory contents to a temporary file.
pub fn store_directory_contents(
    items: HashMap<String, String>,
    json_target: JSONTarget,
) -> Result<(), Error> {
    create_temp_dir()?;

    let mut json = json!({ "items": {} });
    for (key, value) in items.iter() {
        json["items"]
            .as_object_mut()
            .unwrap()
            .insert(key.clone(), json!(value.clone()));
    }

    let mut json_file = get_json_file(json_target, false)?;
    write_to_json(&mut json_file, json)?;

    Ok(())
}

/// Traverse the directory and display files and directories accordingly.
pub fn walk_directory(
    args: &Args,
    extension_icon_map: &HashMap<&str, &str>,
    name_icon_map: &HashMap<&str, &str>,
    walker: &mut Walk,
) -> Result<(StringItem, PrintConfig), Error> {
    let mut current_depth: usize = 0;
    let mut num_directories = 0;
    let mut num_files = 0;
    let mut previous_item = walker
        .next() // Sets the first `previous_item` to the `target_directory`.
        .expect("No items were found in this directory!")
        .unwrap_or_else(|error| panic!("Could not retrieve items in this directory! {error}"));

    println!();

    let mut git_markers: HashMap<String, String> = HashMap::new();

    extend_marker_map(
        &mut git_markers,
        previous_item.path().to_str().unwrap_or("?"),
    );
    let (config, mut tree) = build_tree(args.metadata, &previous_item);

    let mut numbered_items: HashMap<String, String> = HashMap::new();
    let mut labeled_items: HashMap<String, String> = HashMap::new();

    let start = Instant::now();
    while let Some(Ok(item)) = walker.next() {
        check_nesting(current_depth, &item, previous_item, &mut tree);

        let git_marker = git_markers
            .get(&canonicalize_path(item.path().to_str().unwrap_or("?")).unwrap_or("?".to_string()))
            .map_or(None, |marker| Some(marker.to_string()));

        if item.path().is_dir() {
            extend_marker_map(&mut git_markers, item.path().to_str().unwrap_or("?"));

            let label = if args.label_directories {
                //labeled_items.insert();
                //
                // TODO: Calculate the label here
                //      If label reaches "z", label becomes double the character, ie. "aa" comes
                //      after "z".

                let temp_label = "a".to_string();
                Some(temp_label)
            } else {
                None
            };

            let icon = "\u{f115}".to_string(); // 
            tree.begin_child(format_directory(
                git_marker,
                icon,
                label,
                &item,
                args.metadata,
                args.mute_icons,
            ));

            num_directories += 1;
        } else if item.path().is_file() {
            let number = if args.numbers {
                numbered_items.insert(
                    format!("{num_files}"),
                    item.path()
                        .canonicalize()
                        .unwrap_or(PathBuf::from("?"))
                        .into_os_string()
                        .into_string()
                        .unwrap_or("?".into()),
                );

                Some(num_files)
            } else {
                None
            };

            let icon = get_file_icon(extension_icon_map, &item, name_icon_map);
            tree.add_empty_child(format_content(
                git_marker,
                icon,
                &item,
                args.metadata,
                args.mute_icons,
                number,
            ));

            num_files += 1;
        }

        current_depth = item.depth();
        previous_item = item;
    }

    if args.numbers {
        store_directory_contents(numbered_items, JSONTarget::Contents)?;
    }
    if args.label_directories {
        store_directory_contents(labeled_items, JSONTarget::Directories)?;
    }

    print_tree_with(&tree.build(), &config)?;
    println!();

    if args.statistics {
        let duration = start.elapsed().as_millis();
        println!("{num_directories} directories | {num_files} files | {duration} ms\n");
    }

    Ok((tree.build(), config))
}
