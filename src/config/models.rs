//! Structs used when serializing/deserializing configuration settings from `nomad.toml`.

use serde::{Deserialize, Serialize};

/// Contains all settings specified in `nomad.toml`.
#[derive(Debug, Deserialize, Serialize)]
pub struct NomadConfig {
    /// Contains all settings for all things related to Git.
    git: Option<Git>,
    /// Contains the setting for the color of the regex match.
    regex: Option<Regex>,
}

/// Contains settings for all things related to Git.
#[derive(Debug, Deserialize, Serialize)]
pub struct Git {
    /// Contains settings for each Git marker.
    colors: Option<Colors>,
    /// Contains settings for the color of each Git marker.
    markers: Option<Markers>,
}

/// Contains settings for each Git marker.
#[derive(Debug, Deserialize, Serialize)]
pub struct Markers {
    /// The string that marks a conflicting file.
    conflicted_marker: Option<String>,
    /// The string that marks a deleted file.
    deleted_marker: Option<String>,
    /// The string that marks a modified file.
    modified_marker: Option<String>,
    /// The string that marks a renamed file.
    renamed_marker: Option<String>,
    /// The string that marks an untracked file.
    untracked_marker: Option<String>,
}

/// Contains settings for the color of each Git marker.
#[derive(Debug, Deserialize, Serialize)]
pub struct Colors {
    /// The color of the conflicting file's marker.
    conflicted_color: Option<String>,
    /// The color of the deleted file's marker.
    deleted_color: Option<String>,
    /// The color of the modified file's marker.
    modified_color: Option<String>,
    /// The color of the renamed file's marker.
    renamed_color: Option<String>,
    /// The color of the untracked file's marker.
    untracked_color: Option<String>,
}

/// Contains the setting for the color of the regex match.
#[derive(Debug, Deserialize, Serialize)]
pub struct Regex {
    /// The color the matched substring.
    match_color: Option<String>,
}
