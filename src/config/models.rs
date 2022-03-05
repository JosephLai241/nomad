//! Structs used when serializing/deserializing configuration settings from `nomad.toml`.

use serde::{Deserialize, Serialize};

/// Contains all settings specified in `nomad.toml`.
#[derive(Debug, Deserialize, Serialize)]
pub struct NomadConfig {
    /// Contains all settings for all things related to Git.
    pub git: Option<Git>,
    /// Contains the setting for the color of the regex match.
    pub regex: Option<Regex>,
}

/// Contains settings for all things related to Git.
#[derive(Debug, Deserialize, Serialize)]
pub struct Git {
    /// Contains settings for each Git marker.
    pub colors: Option<Colors>,
    /// Contains settings for the color of each Git marker.
    pub markers: Option<Markers>,
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
    /// The string that marks an untracked file.
    pub untracked_marker: Option<String>,
}

/// Contains settings for the color of each Git marker.
#[derive(Debug, Deserialize, Serialize)]
pub struct Colors {
    /// The color of the conflicting file's marker.
    pub conflicted_color: Option<String>,
    /// The color of the deleted file's marker.
    pub deleted_color: Option<String>,
    /// The color of the modified file's marker.
    pub modified_color: Option<String>,
    /// The color of the renamed file's marker.
    pub renamed_color: Option<String>,
    /// The color of the untracked file's marker.
    pub untracked_color: Option<String>,
}

/// Contains the setting for the color of the regex match.
#[derive(Debug, Deserialize, Serialize)]
pub struct Regex {
    /// The color the matched substring.
    pub match_color: Option<String>,
}
