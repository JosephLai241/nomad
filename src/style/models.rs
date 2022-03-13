//! Struct used to store colors/styles for `nomad`.

use ansi_term::{Colour, Style};
use ptree::print_config::UTF_CHARS;
use tui::style::Color;

/// Contains styles used throughout `nomad`.
#[derive(Debug)]
pub struct NomadStyle {
    /// The color and marker styles for all things Git.
    pub git: GitStyle,
    /// The color styles for all things regex.
    pub regex: RegexStyle,
    /// The styles for the tree.
    pub tree: TreeStyle,
    /// The color styles for all things TUI.
    pub tui: TUIStyle,
}

/// Contains color and marker styles for all things Git.
#[derive(Debug)]
pub struct GitStyle {
    /// The color of the conflicting file's marker.
    pub conflicted_color: Style,
    /// The string that marks a conflicting file.
    pub conflicted_marker: String,
    /// The color of the deleted file's marker.
    pub deleted_color: Style,
    /// The string that marks a deleted file.
    pub deleted_marker: String,
    /// The color of the modified file's marker.
    pub modified_color: Style,
    /// The string that marks a modified file.
    pub modified_marker: String,
    /// The color of the renamed file's marker.
    pub renamed_color: Style,
    /// The string that marks a renamed file.
    pub renamed_marker: String,
    /// The color of the staged added file.
    pub staged_added_color: Style,
    /// The string that marks a staged added file.
    pub staged_added_marker: String,
    /// The color of the staged deleted file.
    pub staged_deleted_color: Style,
    /// The string that marks a staged deleted file.
    pub staged_deleted_marker: String,
    /// The color of the staged modified file.
    pub staged_modified_color: Style,
    /// The string that marks a staged modified file.
    pub staged_modified_marker: String,
    /// The color of the staged renamed file.
    pub staged_renamed_color: Style,
    /// The string that marks a staged renamed file.
    pub staged_renamed_marker: String,
    /// The color of the untracked file's marker.
    pub untracked_color: Style,
    /// The string that marks an untracked file.
    pub untracked_marker: String,
}

/// Contains color and marker styles for all things regex.
#[derive(Debug)]
pub struct RegexStyle {
    /// The color of the matched substring.
    pub match_color: Style,
}

/// Contains styles for the tree itself.
#[derive(Debug)]
pub struct TreeStyle {
    /// Contains the indentation setting.
    pub indent: usize,
    /// Contains indent characters for the tree itself.
    pub indent_chars: IndentStyles,
    /// Contains the padding setting.
    pub padding: usize,
}

/// Contains the indent characters for the tree itself.
#[derive(Debug)]
pub struct IndentStyles {
    /// The character used for pointing straight down.
    pub down: String,
    /// The character used for pointing down and to the right.
    pub down_and_right: String,
    /// The character used for empty sections.
    pub empty: String,
    /// The character used for pointing right.
    pub right: String,
    /// The character used for turning from down to right.
    pub turn_right: String,
}

/// Contains color and marker styles for all things TUI.
#[derive(Debug)]
pub struct TUIStyle {
    /// The color of the conflicting file's marker.
    pub conflicted_color: Color,
    /// The color of the deleted file's marker.
    pub deleted_color: Color,
    /// The color of the modified file's marker.
    pub modified_color: Color,
    /// The color of the renamed file's marker.
    pub renamed_color: Color,
    /// The color of the staged added file's marker.
    pub staged_added_color: Color,
    /// The color of the staged deleted file's marker.
    pub staged_deleted_color: Color,
    /// The color of the staged modified file's marker.
    pub staged_modified_color: Color,
    /// The color of the staged renamed file's marker.
    pub staged_renamed_color: Color,
    /// The color of the untracked file's marker.
    pub untracked_color: Color,
}

impl Default for NomadStyle {
    /// Create a new `NomadStyle` with default values.
    fn default() -> Self {
        Self {
            git: GitStyle {
                conflicted_color: Colour::Red.bold(),
                conflicted_marker: "CONFLICT".to_string(),
                deleted_color: Colour::Red.bold(),
                deleted_marker: "D".to_string(),
                modified_color: Colour::Yellow.bold(),
                modified_marker: "M".to_string(),
                renamed_color: Colour::Fixed(172).bold(),
                renamed_marker: "R".to_string(),
                staged_added_color: Colour::Green.bold(),
                staged_added_marker: "SA".to_string(),
                staged_deleted_color: Colour::Red.bold(),
                staged_deleted_marker: "SD".to_string(),
                staged_modified_color: Colour::Yellow.bold(),
                staged_modified_marker: "SM".to_string(),
                staged_renamed_color: Colour::Fixed(172).bold(),
                staged_renamed_marker: "SR".to_string(),
                untracked_color: Colour::Fixed(243).bold(),
                untracked_marker: "U".to_string(),
            },
            regex: RegexStyle {
                match_color: Colour::Fixed(033).bold(),
            },
            tree: TreeStyle {
                indent: 4,
                indent_chars: IndentStyles {
                    down: UTF_CHARS.down.to_string(),
                    down_and_right: UTF_CHARS.down_and_right.to_string(),
                    empty: UTF_CHARS.empty.to_string(),
                    right: UTF_CHARS.right.to_string(),
                    turn_right: UTF_CHARS.turn_right.to_string(),
                },
                padding: 1,
            },
            tui: TUIStyle {
                conflicted_color: Color::Red,
                deleted_color: Color::Red,
                modified_color: Color::Yellow,
                renamed_color: Color::Indexed(172),
                staged_added_color: Color::Green,
                staged_deleted_color: Color::Red,
                staged_modified_color: Color::Yellow,
                staged_renamed_color: Color::Indexed(172),
                untracked_color: Color::Indexed(243),
            },
        }
    }
}
