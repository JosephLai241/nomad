//! Configure style settings for `nomad`.

use ansi_term::Colour;
use ptree::print_config::UTF_CHARS;
use tui::style::Color;

use crate::config::models::{IndentCharacters, NomadConfig};

use super::{
    models::NomadStyle,
    paint::{
        convert_to_ansi_style, convert_to_tui_color, process_git_settings,
        process_tui_git_settings, process_tui_style_settings,
    },
};

/// Return a struct containing user-specified or default settings.
pub fn process_settings(nomad_config: NomadConfig) -> NomadStyle {
    let mut nomad_style = NomadStyle::default();

    if let Some(tree_settings) = nomad_config.tree {
        nomad_style.tree.indent = match tree_settings.indent {
            Some(indent) => indent,
            None => 4,
        };
        nomad_style.tree.padding = match tree_settings.padding {
            Some(padding) => padding,
            None => 1,
        };

        if let Some(label_settings) = tree_settings.labels {
            if let Some(color) = label_settings.item_labels {
                nomad_style.tree.label_colors.item_labels =
                    convert_to_ansi_style(&color.to_lowercase())
            }
            if let Some(color) = label_settings.directory_labels {
                nomad_style.tree.label_colors.directory_labels =
                    convert_to_ansi_style(&color.to_lowercase())
            }
        }

        if let Some(indent_chars) = tree_settings.indent_chars {
            process_indent_chars(indent_chars, &mut nomad_style);
        }

        if let Some(git_settings) = tree_settings.git {
            process_git_settings(&mut nomad_style, &git_settings);
        }

        nomad_style.tree.regex.match_color = match tree_settings.regex {
            Some(regex_setting) => match regex_setting.match_color {
                Some(color) => convert_to_ansi_style(&color),
                None => Colour::Fixed(033).bold(),
            },
            None => Colour::Fixed(033).bold(),
        };
    }

    if let Some(tui_settings) = nomad_config.tui {
        if let Some(git_settings) = tui_settings.git {
            process_tui_git_settings(&mut nomad_style, &git_settings);
        }

        if let Some(style_settings) = tui_settings.style {
            process_tui_style_settings(&mut nomad_style, &style_settings);
        }

        nomad_style.tui.regex.match_color = match tui_settings.regex {
            Some(regex_setting) => match regex_setting.match_color {
                Some(color) => convert_to_tui_color(&color),
                None => Color::Indexed(033),
            },
            None => Color::Indexed(033),
        }
    }

    nomad_style
}

/// Set the indent characters for the tree itself.
fn process_indent_chars(indent_chars: IndentCharacters, nomad_style: &mut NomadStyle) {
    nomad_style.tree.indent_chars.down = match indent_chars.down {
        Some(character) => character,
        None => UTF_CHARS.down.to_string(),
    };
    nomad_style.tree.indent_chars.down_and_right = match indent_chars.down_and_right {
        Some(character) => character,
        None => UTF_CHARS.down_and_right.to_string(),
    };
    nomad_style.tree.indent_chars.empty = match indent_chars.empty {
        Some(character) => character,
        None => UTF_CHARS.empty.to_string(),
    };
    nomad_style.tree.indent_chars.right = match indent_chars.right {
        Some(character) => character,
        None => UTF_CHARS.right.to_string(),
    };
    nomad_style.tree.indent_chars.turn_right = match indent_chars.turn_right {
        Some(character) => character,
        None => UTF_CHARS.turn_right.to_string(),
    };
}
