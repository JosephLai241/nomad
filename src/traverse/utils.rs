//! Directory traversal utilities.

use crate::{
    cli::Args,
    errors::NomadError,
    utils::{
        meta::get_metadata,
        temp::{create_temp_dir, get_json_file, write_to_json},
    },
    EXTENSION_ICON_MAP, NAME_ICON_MAP,
};

use ansi_term::Colour;
use anyhow::Result;
use ignore::{
    types::{Types, TypesBuilder},
    Walk, WalkBuilder,
};
use ptree::{Color, PrintConfig, Style, TreeBuilder};
use serde_json::{json, Value};

use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Component, Path},
};

/// Contains options for `Types` building.
pub enum TypeOption {
    /// Build a `Types` that matches a filetype.
    Match,
    /// Build a `Types` that negates a filetype.
    Negate,
}

/// Build an `ignore` `Types` depending on the
pub fn build_types(
    filetypes: &Vec<String>,
    globs: &Vec<String>,
    mut type_matcher: TypesBuilder,
    type_option: TypeOption,
) -> Result<Types, NomadError> {
    for filetype in filetypes {
        match type_option {
            TypeOption::Match => {
                type_matcher.select(&filetype);
            }
            TypeOption::Negate => {
                type_matcher.negate(&filetype);
            }
        };
    }

    for (index, glob) in globs.iter().enumerate() {
        let glob_label = index.to_string();

        type_matcher.add(&glob_label, glob)?;

        match type_option {
            TypeOption::Match => {
                type_matcher.select(&glob_label);
            }
            TypeOption::Negate => {
                type_matcher.negate(&glob_label);
            }
        }
    }

    type_matcher.build().map_or_else(
        |error| Err(NomadError::IgnoreError(error)),
        |types| Ok(types),
    )
}

/// Build a `Walk` object based on the client's CLI parameters.
pub fn build_walker(
    args: &Args,
    target_directory: &str,
    types: Option<Types>,
) -> Result<Walk, NomadError> {
    if Path::new(target_directory).is_dir() {
        let mut walk = WalkBuilder::new(target_directory);

        walk.follow_links(true)
            .git_exclude(!args.disrespect)
            .git_global(!args.disrespect)
            .git_ignore(!args.disrespect)
            .hidden(!args.hidden)
            .ignore(!args.disrespect)
            .max_depth(args.max_depth)
            .max_filesize(args.max_filesize)
            .parents(!args.disrespect)
            .sort_by_file_path(|a, b| a.cmp(b));

        if let Some(types) = types {
            walk.types(types);
        }

        Ok(walk.build())
    } else {
        Err(NomadError::NotADirectory(target_directory.into()))
    }
}

/// Get the file's corresponding icon.
pub fn get_file_icon(item_path: &Path) -> String {
    if let Some(icon) = EXTENSION_ICON_MAP.get(
        item_path
            .extension()
            .unwrap_or(OsStr::new("none"))
            .to_str()
            .unwrap(),
    ) {
        icon.to_string()
    } else {
        if let Some(icon) = NAME_ICON_MAP.get(
            &item_path
                .file_name()
                .unwrap_or(OsStr::new("?"))
                .to_str()
                .unwrap_or("?"),
        ) {
            icon.to_string()
        } else {
            "\u{f016}".to_string() // 
        }
    }
}

/// Build a `ptree` object and set the tree's style/configuration.
pub fn build_tree(
    include_metadata: bool,
    plain: bool,
    target_directory: &Path,
) -> (PrintConfig, TreeBuilder) {
    let directory_icon = &"\u{f115}"; // 

    let plain_directory_name = target_directory
        .file_name()
        .unwrap_or(OsStr::new("?"))
        .to_str()
        .unwrap_or("?")
        .to_string();
    let directory_name = if plain {
        plain_directory_name
    } else {
        format!(
            "{directory_icon} {}",
            Colour::Blue.bold().paint(plain_directory_name).to_string()
        )
    };

    let mut tree_label = directory_name;
    if include_metadata {
        let metadata = get_metadata(target_directory, plain);
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
    item: &Path,
    previous_item: &Path,
    target_directory: &str,
    tree: &mut TreeBuilder,
) {
    let mut item_depth = 0;
    for component in item
        .strip_prefix(target_directory)
        .unwrap_or(Path::new("?"))
        .components()
    {
        match component {
            Component::Normal(_) => item_depth += 1,
            _ => {}
        }
    }

    if item_depth < current_depth {
        if previous_item.is_dir() {
            let item_parent = item
                .parent()
                .expect("Could not get the current item's parent!");
            let previous_parent = previous_item
                .parent()
                .expect("Could not get the previous item's parent!");

            if item_parent != previous_parent {
                tree.end_child();
            }
        }

        for _ in 0..current_depth - item_depth {
            tree.end_child();
        }
    } else if item_depth == current_depth && previous_item.is_dir() {
        tree.end_child();
    }
}

/// Write the labeled directories or numbered directory contents to a temporary file.
pub fn store_directory_contents(
    labeled_items: HashMap<String, String>,
    numbered_items: HashMap<String, String>,
) -> Result<(), NomadError> {
    create_temp_dir()?;

    let mut json = json!({ "labeled": {}, "numbered": {} });

    write_map(labeled_items, &mut json, "labeled");
    write_map(numbered_items, &mut json, "numbered");

    let mut json_file = get_json_file(false)?;
    write_to_json(&mut json_file, json)?;

    Ok(())
}

/// Write each key, value within a HashMap to JSON `Value` object.
fn write_map(items: HashMap<String, String>, json: &mut Value, target_key: &str) {
    for (key, value) in items.iter() {
        json[target_key]
            .as_object_mut()
            .unwrap()
            .insert(key.clone(), json!(value.clone()));
    }
}
