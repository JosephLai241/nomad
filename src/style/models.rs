//! Struct used to store colors/styles for `nomad`.

use ansi_term::{Colour, Style};
use ptree::print_config::UTF_CHARS;
use tui::style::Color;

/// Contains styles used throughout `nomad`.
#[derive(Debug)]
pub struct NomadStyle {
    /// The color and marker styles for all things Git.
    pub git: GitStyle,
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
    /// The color that marks a staged typechanged file.
    pub staged_typechanged_color: Style,
    /// The string that marks a staged typechanged file.
    pub staged_typechanged_marker: String,
    /// The color that marks a typechanged file.
    pub typechanged_color: Style,
    /// The string that marks a typechanged file.
    pub typechanged_marker: String,
    /// The color of the untracked file's marker.
    pub untracked_color: Style,
    /// The string that marks an untracked file.
    pub untracked_marker: String,
}

/// Contains styles for the tree itself.
#[derive(Debug)]
pub struct TreeStyle {
    /// Contains the indentation setting.
    pub indent: usize,
    /// Contains indent characters for the tree itself.
    pub indent_chars: IndentStyles,
    /// Contains the colors for items in the tree.
    pub item_colors: ItemColors,
    /// Contains colors for the tree labels.
    pub label_colors: LabelColors,
    /// Contains the padding setting.
    pub padding: usize,
    /// The color styles for all things regex.
    pub regex: TreeRegexStyle,
}

/// Contains the colors for items in the tree.
#[derive(Debug)]
pub struct ItemColors {
    /// The color for directories.
    pub directory_color: Style,
}

/// Contains colors for the tree labels.
#[derive(Debug)]
pub struct LabelColors {
    /// The color for item labels.
    pub item_labels: Style,
    /// The color for directory labels.
    pub directory_labels: Style,
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

/// Contains color and marker styles for regex matches in the standard tree.
#[derive(Debug)]
pub struct TreeRegexStyle {
    /// The color of the matched substring.
    pub match_color: Style,
}

/// Contains color and marker styles for all things TUI.
#[derive(Debug)]
pub struct TUIStyle {
    /// The color of all widget borders.
    pub border_color: Color,
    /// Contains the Git styles for the TUI.
    pub git: TUIGitStyle,
    /// The color styles for all things regex.
    pub regex: TUIRegexStyle,
    /// The color of the tree item if it does not contain any Git changes.
    pub standard_item_highlight_color: Color,
}

/// Contains the Git styles for the TUI.
#[derive(Debug)]
pub struct TUIGitStyle {
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

/// Contains color and marker styles for regex matches in the TUI.
#[derive(Debug)]
pub struct TUIRegexStyle {
    /// The color of the matched substring.
    pub match_color: Color,
}

impl Default for NomadStyle {
    /// Create a new `NomadStyle` with default values.
    fn default() -> Self {
        Self {
            git: GitStyle {
                conflicted_color: Colour::Red.bold(),
                conflicted_marker: "!".to_string(),
                deleted_color: Colour::Red.bold(),
                deleted_marker: "D".to_string(),
                modified_color: Colour::Fixed(172).bold(),
                modified_marker: "M".to_string(),
                renamed_color: Colour::Fixed(172).bold(),
                renamed_marker: "R".to_string(),
                staged_added_color: Colour::Green.bold(),
                staged_added_marker: "SA".to_string(),
                staged_deleted_color: Colour::Red.bold(),
                staged_deleted_marker: "SD".to_string(),
                staged_modified_color: Colour::Fixed(172).bold(),
                staged_modified_marker: "SM".to_string(),
                staged_renamed_color: Colour::Fixed(172).bold(),
                staged_renamed_marker: "SR".to_string(),
                staged_typechanged_color: Colour::Purple.bold(),
                staged_typechanged_marker: "STC".to_string(),
                typechanged_color: Colour::Purple.bold(),
                typechanged_marker: "TC".to_string(),
                untracked_color: Colour::Fixed(243).bold(),
                untracked_marker: "U".to_string(),
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
                item_colors: ItemColors {
                    directory_color: Colour::Blue.bold(),
                },
                label_colors: LabelColors {
                    item_labels: Colour::Fixed(068).bold(),
                    directory_labels: Colour::Fixed(068).bold(),
                },
                padding: 1,
                regex: TreeRegexStyle {
                    match_color: Colour::Fixed(033).bold(),
                },
            },
            tui: TUIStyle {
                border_color: Color::Indexed(033),
                git: TUIGitStyle {
                    conflicted_color: Color::Red,
                    deleted_color: Color::Red,
                    modified_color: Color::Indexed(172),
                    renamed_color: Color::Indexed(172),
                    staged_added_color: Color::Green,
                    staged_deleted_color: Color::Red,
                    staged_modified_color: Color::Indexed(172),
                    staged_renamed_color: Color::Indexed(172),
                    untracked_color: Color::Indexed(243),
                },
                regex: TUIRegexStyle {
                    match_color: Color::Indexed(033),
                },
                standard_item_highlight_color: Color::Indexed(033),
            },
        }
    }
}
