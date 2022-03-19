//! Utilities for the TUI.

use std::path::{Component, Path};

use ptree::write_tree_with;
use tui::{
    style::{Color, Style},
    widgets::{Cell, Row},
};

use crate::{
    cli::global::GlobalArgs,
    errors::NomadError,
    style::models::NomadStyle,
    traverse::{models::DirItem, modes::NomadMode, utils::build_walker, walk_directory},
};

/// Return all app settings formatted in `Row`s.
pub fn get_settings<'a>(args: &GlobalArgs) -> Vec<Row<'a>> {
    let assign_boolean_flag = |label: &'a str, flag| -> Row<'a> {
        Row::new(vec![
            Cell::from(label),
            Cell::from(format!("{}", flag)).style(Style::default().fg(if flag {
                Color::Green
            } else {
                Color::Red
            })),
        ])
    };

    vec![
        assign_boolean_flag(" all labels", args.labels.all_labels),
        assign_boolean_flag(" dirs", args.modifiers.dirs),
        assign_boolean_flag(" disrespect", args.modifiers.disrespect),
        assign_boolean_flag(" hidden", args.modifiers.hidden),
        assign_boolean_flag(" label directories", args.labels.label_directories),
        Row::new(vec![
            Cell::from(" max depth"),
            Cell::from(format!(
                "{}",
                if let Some(ref depth) = args.modifiers.max_depth {
                    depth.to_string()
                } else {
                    "None".to_string()
                }
            ))
            .style(Style::default().fg(if args.modifiers.max_depth.is_some() {
                Color::Green
            } else {
                Color::Red
            })),
        ]),
        Row::new(vec![
            Cell::from(" max filesize"),
            Cell::from(format!(
                "{}",
                if let Some(ref size) = args.modifiers.max_filesize {
                    size.to_string()
                } else {
                    "None".to_string()
                }
            ))
            .style(
                Style::default().fg(if args.modifiers.max_filesize.is_some() {
                    Color::Green
                } else {
                    Color::Red
                }),
            ),
        ]),
        assign_boolean_flag(" metadata", args.meta.metadata),
        assign_boolean_flag(" no Git", args.style.no_git),
        assign_boolean_flag(" no icons", args.style.no_icons),
        assign_boolean_flag(" numbered", args.labels.numbers),
        Row::new(vec![
            Cell::from(" pattern"),
            Cell::from(format!(
                "{}",
                if let Some(ref pattern) = args.regex.pattern {
                    pattern.to_string()
                } else {
                    "None".to_string()
                }
            ))
            .style(Style::default().fg(if args.regex.pattern.is_some() {
                Color::Green
            } else {
                Color::Red
            })),
        ]),
        assign_boolean_flag(" plain", args.style.plain),
    ]
}

/// Get the breadcrumbs for the target directory.
pub fn get_breadcrumbs(target_directory: &str) -> Result<Vec<String>, NomadError> {
    let mut breadcrumbs = Vec::new();
    for component in Path::new(target_directory).canonicalize()?.components() {
        match component {
            Component::Normal(section) => {
                breadcrumbs.push(section.to_str().unwrap_or("?").to_string());
            }
            _ => {}
        }
    }

    Ok(breadcrumbs)
}

/// Get the directory tree as a `Vec<String>` and the directory items as an `Option<Vec<String>>`.
pub fn get_tree(
    args: &GlobalArgs,
    nomad_style: &NomadStyle,
    target_directory: &str,
) -> Result<(Vec<String>, Option<Vec<DirItem>>), NomadError> {
    let (tree, config, directory_items) = walk_directory(
        args,
        NomadMode::Rootless,
        nomad_style,
        target_directory,
        &mut build_walker(args, target_directory, None)?,
    )?;

    // Write the tree to a buffer, then convert it to a `Vec<String>`.
    let mut tree_buf = Vec::new();
    write_tree_with(&tree, &mut tree_buf, &config)?;

    Ok((
        String::from_utf8_lossy(&tree_buf)
            .split("\n")
            .map(|line| line.to_string())
            .collect::<Vec<String>>(),
        directory_items,
    ))
}

/// Reset all settings to its original value.
pub fn reset_args(args: &mut GlobalArgs) {
    if args.labels.all_labels {
        args.labels.all_labels = false;
    }
    if args.modifiers.dirs {
        args.modifiers.dirs = false;
    }
    if args.modifiers.disrespect {
        args.modifiers.disrespect = false;
    }
    if args.export.is_some() {
        args.export = None;
    }
    if args.modifiers.hidden {
        args.modifiers.hidden = false;
    }
    if args.labels.label_directories {
        args.labels.label_directories = false;
    }
    if args.modifiers.max_depth.is_some() {
        args.modifiers.max_depth = None;
    }
    if args.modifiers.max_filesize.is_some() {
        args.modifiers.max_filesize = None;
    }
    if args.meta.metadata {
        args.meta.metadata = false;
    }
    if args.style.no_git {
        args.style.no_git = false;
    }
    if args.style.no_icons {
        args.style.no_icons = false;
    }
    if args.labels.numbers {
        args.labels.numbers = false;
    }
    if args.regex.pattern.is_some() {
        args.regex.pattern = None;
    }
    if args.style.plain {
        args.style.plain = false;
    }
    if args.statistics {
        args.statistics = false;
    }
}
