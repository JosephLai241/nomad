//! Structs used when serializing/deserializing configuration settings from `nomad.toml`.

use serde::{Deserialize, Serialize};

/// Contains all settings specified in `nomad.toml`.
#[derive(Debug, Deserialize, Serialize)]
pub struct NomadConfig {
    /// Contains settings for the standard tree.
    pub tree: Option<TreeSettings>,
    /// Contains settings for the TUI.
    pub tui: Option<TUISettings>,
}

/// Contains settings for the standard tree.
#[derive(Debug, Deserialize, Serialize)]
pub struct TreeSettings {
    /// Contains settings for all things related to Git in the standard tree.
    pub git: Option<TreeGit>,
    /// Contains settings for the color of tree labels (items and directories).
    pub labels: Option<LabelColors>,
    /// Contains the indentation setting.
    pub indent: Option<usize>,
    /// Contains settings for the tree items' appearance.
    pub items: Option<TreeItems>,
    /// Contains indent characters for the tree itself.
    pub indent_chars: Option<IndentCharacters>,
    /// Contains the padding setting.
    pub padding: Option<usize>,
    /// Contains the setting for the color of the regex match.
    pub regex: Option<Regex>,
}

/// Contains settings for the TUI.
#[derive(Debug, Deserialize, Serialize)]
pub struct TUISettings {
    /// Contains settings for all things related to Git in the TUI.
    pub git: Option<TUIGit>,
    /// Contains settings for the TUI's style.
    pub style: Option<TUIStyle>,
    /// Contains the setting for the color of the regex match in the text view.
    pub regex: Option<Regex>,
}

/// Contains settings for the tree items' appearance.
#[derive(Debug, Deserialize, Serialize)]
pub struct TreeItems {
    /// The colors for items in the tree.
    pub colors: Option<TreeItemColor>,
}

/// Contains settings for tree items' appearance.
#[derive(Debug, Deserialize, Serialize)]
pub struct TreeItemColor {
    /// The color for directories.
    pub directory_color: Option<String>,
}

/// Contains settings for the color of tree labels (items and directories).
#[derive(Debug, Deserialize, Serialize)]
pub struct LabelColors {
    /// The color for item labels.
    pub item_labels: Option<String>,
    /// The color for directory labels.
    pub directory_labels: Option<String>,
}

/// Contains indent characters for the tree itself.
#[derive(Debug, Deserialize, Serialize)]
pub struct IndentCharacters {
    /// The character used for pointing straight down.
    pub down: Option<String>,
    /// The character used for pointing down and to the right.
    pub down_and_right: Option<String>,
    /// The character used for empty sections.
    pub empty: Option<String>,
    /// The character used for pointing right.
    pub right: Option<String>,
    /// The character used for turning from down to right.
    pub turn_right: Option<String>,
}

/// Contains settings for all things related to Git in the standard tree.
#[derive(Debug, Deserialize, Serialize)]
pub struct TreeGit {
    /// Contains settings for the color of each Git marker.
    pub colors: Option<Colors>,
    /// Contains settings for each Git marker.
    pub markers: Option<Markers>,
}

/// Contains the setting for the color of the regex match.
#[derive(Debug, Deserialize, Serialize)]
pub struct Regex {
    /// The color the matched substring.
    pub match_color: Option<String>,
}

/// Contains settings for the TUI's style.
#[derive(Debug, Deserialize, Serialize)]
pub struct TUIStyle {
    /// The color of the borders.
    pub border_color: Option<String>,
    /// The color of the tree item if it does not contain any Git changes.
    pub standard_item_highlight_color: Option<String>,
}

/// Contains settings for all things related to Git in the TUI.
#[derive(Debug, Deserialize, Serialize)]
pub struct TUIGit {
    /// Contains settings for the color of each Git marker.
    pub colors: Option<Colors>,
}

/// Contains settings for each Git marker.
#[derive(Debug, Deserialize, Serialize)]
pub struct Markers {
    /// The string that marks a conflicting file.
    pub conflicted_marker: Option<String>,
    /// The string that marks a deleted file.
    pub deleted_marker: Option<String>,
    /// The string that marks a modified file.
    pub modified_marker: Option<String>,
    /// The string that marks a renamed file.
    pub renamed_marker: Option<String>,
    /// The string that marks a staged added file.
    pub staged_added_marker: Option<String>,
    /// The string that marks a staged deleted file.
    pub staged_deleted_marker: Option<String>,
    /// The string that marks a staged modified file.
    pub staged_modified_marker: Option<String>,
    /// The string that marks a staged renamed file.
    pub staged_renamed_marker: Option<String>,
    /// The string that marks a staged typechanged file.
    pub staged_typechanged_marker: Option<String>,
    /// The string that marks a typechanged file.
    pub typechanged_marker: Option<String>,
    /// The string that marks an untracked file.
    pub untracked_marker: Option<String>,
}

/// Contains settings for the color of each Git marker.
#[derive(Debug, Deserialize, Serialize)]
pub struct Colors {
    /// The color associated with conflicting files.
    pub conflicted_color: Option<String>,
    /// The color associated with deleted files.
    pub deleted_color: Option<String>,
    /// The color associated with modified files.
    pub modified_color: Option<String>,
    /// The color associated with renamed files.
    pub renamed_color: Option<String>,
    /// The color associated with staged added files.
    pub staged_added_color: Option<String>,
    /// The color associated with staged deleted files.
    pub staged_deleted_color: Option<String>,
    /// The color associated with staged modified files.
    pub staged_modified_color: Option<String>,
    /// The color associated with staged renamed files.
    pub staged_renamed_color: Option<String>,
    /// The color associated with staged typechanged files.
    pub staged_typechanged_color: Option<String>,
    /// The color associated with typechanged files.
    pub typechanged_marker: Option<String>,
    /// The color associated with untracked files.
    pub untracked_color: Option<String>,
}
