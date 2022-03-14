//! Utilities for counting lines of code.

use std::{collections::BTreeMap, path::PathBuf};

use tokei::{Config, Language, LanguageType, Report};

/// Get the `Report` for a specific file.
///
/// ```rust
/// tokei::Report {
///     /// The code statistics found in the file.
///     pub stats: CodeStats,
///     /// File name.
///     pub name: PathBuf,
/// }
/// ```
pub fn get_file_report<'a>(
    language_children: &'a BTreeMap<LanguageType, Vec<Report>>,
    path: PathBuf,
) -> Option<&'a Report> {
    match LanguageType::from_path(&path, &Config::default()) {
        Some(language_type) => match language_children.get(&language_type) {
            Some(reports) => reports.iter().find(|report| report.name == path),
            None => None,
        },
        None => None,
    }
}