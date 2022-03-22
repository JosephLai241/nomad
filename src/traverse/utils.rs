//! Directory traversal utilities.

use crate::{
    cli::global::GlobalArgs,
    errors::NomadError,
    style::models::NomadStyle,
    utils::{
        cache::{get_json_file, write_to_json},
        meta::get_metadata,
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

use super::modes::NomadMode;

/// Contains options for `Types` building.
pub enum TypeOption {
    /// Build a `Types` that matches a filetype.
    Match,
    /// Build a `Types` that negates a filetype.
    Negate,
}

/// Build an `ignore` `Types` depending on the
pub fn build_types(
    filetypes: &[String],
    globs: &[String],
    mut type_matcher: TypesBuilder,
    type_option: TypeOption,
) -> Result<Types, NomadError> {
    for filetype in filetypes {
        match type_option {
            TypeOption::Match => {
                type_matcher.select(filetype);
            }
            TypeOption::Negate => {
                type_matcher.negate(filetype);
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

    type_matcher
        .build()
        .map_or_else(|error| Err(NomadError::IgnoreError(error)), Ok)
}

/// Build a `Walk` object based on the client's CLI parameters.
pub fn build_walker(
    args: &GlobalArgs,
    target_directory: &str,
    types: Option<Types>,
) -> Result<Walk, NomadError> {
    if Path::new(target_directory).is_dir() {
        let mut walk = WalkBuilder::new(target_directory);

        walk.follow_links(true)
            .git_exclude(!args.modifiers.disrespect)
            .git_global(!args.modifiers.disrespect)
            .git_ignore(!args.modifiers.disrespect)
            .hidden(!args.modifiers.hidden)
            .ignore(!args.modifiers.disrespect)
            .max_depth(args.modifiers.max_depth)
            .max_filesize(args.modifiers.max_filesize)
            .parents(!args.modifiers.disrespect)
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
            .unwrap_or_else(|| OsStr::new("none"))
            .to_str()
            .unwrap(),
    ) {
        icon.to_string()
    } else if let Some(icon) = NAME_ICON_MAP.get(
        &item_path
            .file_name()
            .unwrap_or_else(|| OsStr::new("?"))
            .to_str()
            .unwrap_or("?"),
    ) {
        icon.to_string()
    } else {
        "\u{f016}".to_string() // 
    }
}

/// Build a `ptree` object and set the tree's style/configuration.
pub fn build_tree(
    args: &GlobalArgs,
    nomad_mode: &NomadMode,
    nomad_style: &NomadStyle,
    target_directory: &Path,
) -> (PrintConfig, TreeBuilder) {
    let directory_icon = &"\u{f115}"; // 

    let plain_name = target_directory
        .file_name()
        .unwrap_or_else(|| OsStr::new("?"))
        .to_str()
        .unwrap_or("?")
        .to_string();
    let directory_name = match nomad_mode {
        NomadMode::GitBranch => format!(
            "{}{} [{}]",
            match args.style.no_icons {
                true => "",
                false => "\u{f1d3} ",
            },
            Colour::Blue.bold().paint(plain_name),
            Colour::Fixed(172).bold().paint("BRANCHES")
        ),
        _ => {
            if args.style.plain {
                plain_name
            } else if args.style.no_colors {
                format!("{directory_icon} {plain_name}")
            } else {
                format!("{directory_icon} {}", Colour::Blue.bold().paint(plain_name))
            }
        }
    };

    let mut tree_label = directory_name;
    match nomad_mode {
        NomadMode::GitBranch => {}
        _ => {
            if args.meta.metadata {
                let metadata = get_metadata(args, target_directory);
                tree_label = format!("{metadata} {tree_label}");
            }
        }
    }

    let tree = TreeBuilder::new(tree_label);
    let config = build_tree_style(nomad_style);

    (config, tree)
}

/// Build a new `Style` based on the settings in `NomadStyle`.
pub fn build_tree_style(nomad_style: &NomadStyle) -> PrintConfig {
    let mut branch_style = Style::default();

    branch_style.bold = true;
    branch_style.foreground = Some(Color::White);

    let mut config = PrintConfig::default();

    config.branch = branch_style;
    config.indent = nomad_style.tree.indent;
    config.padding = nomad_style.tree.padding;

    config.characters.down = nomad_style.tree.indent_chars.down.to_string();
    config.characters.down_and_right = nomad_style.tree.indent_chars.down_and_right.to_string();
    config.characters.empty = nomad_style.tree.indent_chars.empty.to_string();
    config.characters.right = nomad_style.tree.indent_chars.right.to_string();
    config.characters.turn_right = nomad_style.tree.indent_chars.turn_right.to_string();

    config
}

/// Run checks to ensure tree nesting is correct. Make any corrections if applicable.
pub fn check_nesting(
    current_depth: usize,
    item: &Path,
    nomad_mode: &NomadMode,
    previous_item: &Path,
    target_directory: &str,
    tree: &mut TreeBuilder,
) {
    let mut item_depth = 0;
    let item_components = match nomad_mode {
        NomadMode::GitBranch => item.components(),
        _ => item
            .strip_prefix(target_directory)
            .unwrap_or_else(|_| Path::new("?"))
            .components(),
    };

    for component in item_components {
        if let Component::Normal(_) = component {
            item_depth += 1;
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
