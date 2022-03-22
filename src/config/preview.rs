//! Create a dummy tree to preview the current settings.

use ansi_term::{Colour, Style};
use anyhow::Result;
use ptree::{print_tree_with, TreeBuilder};

use crate::{
    errors::NomadError,
    style::models::NomadStyle,
    traverse::{format::highlight_matched, utils::build_tree_style},
};

/// Build a dummy tree with the current tree settings.
pub fn display_preview_tree(nomad_style: &NomadStyle) -> Result<(), NomadError> {
    let mut tree = TreeBuilder::new(format!(
        "\u{e615} {}{}{}", // ""
        Style::new().bold().paint("["),
        Colour::Fixed(172).bold().paint("PREVIEW"),
        Style::new().bold().paint("]"),
    ));
    let config = build_tree_style(nomad_style);

    // Begin Git configuration branch. Doing these in alphabetical order.
    tree.begin_child(format!(
        "\u{f1d3} {}{}{}", // ""
        Style::new().bold().paint("["),
        Colour::Fixed(172).bold().paint("GIT"),
        Style::new().bold().paint("]"),
    ));

    // Working directory Git changes.
    tree.add_empty_child(format!(
        "{} \u{e204} conflicting file", // ""
        nomad_style
            .git
            .conflicted_color
            .paint(nomad_style.git.conflicted_marker.to_string())
    ));
    tree.add_empty_child(format!(
        "{} \u{e61d} deleted file", // ""
        nomad_style
            .git
            .deleted_color
            .paint(nomad_style.git.deleted_marker.to_string())
    ));
    tree.add_empty_child(format!(
        "{} \u{e7a8} modified file", // ""
        nomad_style
            .git
            .modified_color
            .paint(nomad_style.git.modified_marker.to_string())
    ));
    tree.add_empty_child(format!(
        "{} \u{f48a} renamed file", // ""
        nomad_style
            .git
            .renamed_color
            .paint(nomad_style.git.renamed_marker.to_string())
    ));

    // Staged (index) Git changes.
    tree.add_empty_child(format!(
        "{} \u{e606} {}", // ""
        nomad_style
            .git
            .staged_added_color
            .paint(nomad_style.git.staged_added_marker.to_string()),
        nomad_style
            .git
            .staged_added_color
            .paint("staged added file")
    ));
    tree.add_empty_child(format!(
        "{} \u{e61d} {}", // ""
        nomad_style
            .git
            .staged_deleted_color
            .paint(nomad_style.git.staged_deleted_marker.to_string()),
        nomad_style
            .git
            .staged_deleted_color
            .strikethrough()
            .paint("staged deleted file")
    ));
    tree.add_empty_child(format!(
        "{} \u{e7a8} {}", // ""
        nomad_style
            .git
            .staged_modified_color
            .paint(nomad_style.git.staged_modified_marker.to_string()),
        nomad_style
            .git
            .staged_modified_color
            .paint("staged modified file")
    ));
    tree.add_empty_child(format!(
        "{} \u{f48a} {}", // ""
        nomad_style
            .git
            .staged_renamed_color
            .paint(nomad_style.git.staged_renamed_marker.to_string()),
        nomad_style
            .git
            .staged_renamed_color
            .paint("staged deleted file")
    ));

    // Last working directory Git change.
    tree.add_empty_child(format!(
        "{} \u{e74e} renamed file", // ""
        nomad_style
            .git
            .untracked_color
            .paint(nomad_style.git.untracked_marker.to_string())
    ));

    tree.end_child();

    // Begin regex match branch.
    tree.begin_child(format!(
        "\u{e60b} {}{}{}", // ""
        Style::new().bold().paint("["),
        Colour::Fixed(172).bold().paint("REGEX"),
        Style::new().bold().paint("]"),
    ));
    tree.begin_child(format!(
        "\u{f115} {}", // 
        highlight_matched(true, nomad_style, "directory match".to_string(), (5, 8),)
    ));
    tree.add_empty_child(format!(
        "\u{e7a8} {}", // ""
        highlight_matched(false, nomad_style, "item match".to_string(), (5, 8))
    ));

    tree.end_child();

    println!();
    print_tree_with(&tree.build(), &config)?;

    Ok(())
}
