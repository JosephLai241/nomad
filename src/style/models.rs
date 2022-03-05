//! Struct used to store colors/styles for `nomad`.

use ansi_term::{Colour, Style};

/// Contains styles used throughout `nomad`.
#[derive(Debug)]
pub struct NomadStyle {
    /// The color of the conflicting file's marker.
    pub conflicted_color: Style,
    /// The string that marks a conflicting file.
    pub conflicted_marker: String,
    /// The color of the deleted file's marker.
    pub deleted_color: Style,
    /// The string that marks a deleted file.
    pub deleted_marker: String,
    /// The color of the matched substring.
    pub match_color: Style,
    /// The color of the modified file's marker.
    pub modified_color: Style,
    /// The string that marks a modified file.
    pub modified_marker: String,
    /// The color of the renamed file's marker.
    pub renamed_color: Style,
    /// The string that marks a renamed file.
    pub renamed_marker: String,
    /// The color of the untracked file's marker.
    pub untracked_color: Style,
    /// The string that marks an untracked file.
    pub untracked_marker: String,
}

impl Default for NomadStyle {
    /// Create a new `NomadStyle` with default values.
    fn default() -> Self {
        Self {
            conflicted_color: Colour::Red.bold(),
            conflicted_marker: "CONFLICT".to_string(),
            deleted_color: Colour::Red.bold(),
            deleted_marker: "D".to_string(),
            match_color: Colour::Fixed(033).bold(),
            modified_color: Colour::Yellow.bold(),
            modified_marker: "M".to_string(),
            renamed_color: Colour::Fixed(172).bold(),
            renamed_marker: "R".to_string(),
            untracked_color: Colour::Fixed(243).bold(),
            untracked_marker: "U".to_string(),
        }
    }
}
