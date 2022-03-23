//! Utilities for counting lines of code.

use std::{collections::BTreeMap, path::PathBuf};

use tokei::{Config, LanguageType, Report};

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
pub fn get_file_report(
    language_children: &BTreeMap<LanguageType, Vec<Report>>,
    path: PathBuf,
) -> Option<&'_ Report> {
    match LanguageType::from_path(&path, &Config::default()) {
        Some(language_type) => match language_children.get(&language_type) {
            Some(reports) => reports.iter().find(|report| report.name == path),
            None => None,
        },
        None => None,
    }
}
