//! Format `tokei` file metadata for tree or table views.

use ansi_term::{Colour, Style};
use tokei::Report;

/// Contains formatted strings for a file's individual tokei metadata.
pub struct TokeiTreeStats {
    /// The formatted string indicating the number of blank lines in this file.
    pub blanks: String,
    /// The formatted string indicating the lines of code in this file.
    pub code: String,
    /// The formatted string indicating the number of comments in this file.
    pub comments: String,
    /// The total number of lines in this file.
    pub lines: String,
}

/// Format the file's complimentary `Report` for normal/tree view.
pub fn tree_stats_from_report(report: Option<&'_ Report>) -> Option<TokeiTreeStats> {
    report.map(|metadata| TokeiTreeStats {
        blanks: format!(
            "{} Blanks   {}",
            Style::new().bold().paint("|"),
            Colour::Fixed(030)
                .bold()
                .paint(format!("{}", metadata.stats.blanks))
        ),
        code: format!(
            "{} Code     {}",
            Style::new().bold().paint("|"),
            Colour::Fixed(030)
                .bold()
                .paint(format!("{}", metadata.stats.code))
        ),
        comments: format!(
            "{} Comments {}",
            Style::new().bold().paint("|"),
            Colour::Fixed(030)
                .bold()
                .paint(format!("{}", metadata.stats.comments))
        ),
        lines: format!(
            "{} Lines    {}",
            Style::new().bold().paint("|"),
            Colour::Fixed(030)
                .bold()
                .paint(format!("{}", metadata.stats.lines()))
        ),
    })
}
