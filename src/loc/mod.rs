//! Exposing functionality to `Tokei` - counts lines of code and other file metadata.

pub mod format;
pub mod utils;

use std::path::PathBuf;

use ansi_term::{Colour, Style};
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};
use tokei::{Config, Language, Languages, Sort};

use crate::cli::global::GlobalArgs;

use self::{format::tree_stats_from_report, utils::get_file_report};

// FUTURE: Add a table in `nomad.toml` called `[tokei]` to set the `Config`
//         and ignored paths.

/// Get the `Language` struct for a directory.
pub fn loc_in_dir(target_directory: &str) -> Language {
    let mut languages = Languages::new();
    languages.get_statistics(&[target_directory], &[], &Config::default());

    languages.total()
}

/// Get the `CodeStats` for a single file from the `Language` struct.
pub fn loc_in_file(args: &GlobalArgs, file_path: &str, tokei: &Language) -> Vec<String> {
    let report = get_file_report(&tokei.children, PathBuf::from(file_path));

    let mut formatted_stats = Vec::new();

    match tree_stats_from_report(args, report) {
        Some(stats) => {
            formatted_stats.push(stats.blanks);
            formatted_stats.push(stats.code);
            formatted_stats.push(stats.comments);
            formatted_stats.push(stats.lines);
        }
        None => formatted_stats.push(if args.style.no_colors || args.style.plain {
            "| No tokei data available".to_string()
        } else {
            format!(
                "{} {}",
                Style::new().bold().paint("|"),
                Colour::Fixed(172).bold().paint("No tokei data available")
            )
        }),
    }

    formatted_stats
}

/// Summarize the `Tokei` stats for this directory.
/// Sort summary by lines of code, descending.
pub fn run_tokei(target_directory: &str) {
    let mut languages = Languages::new();
    let config = Config {
        sort: Some(Sort::Code), // Why doesn't this fucking work?
        ..Config::default()
    };

    languages.get_statistics(&[target_directory], &[], &config);

    let mut summary = languages.total();

    if summary.is_empty() {
        println!(
            "\n{}\n",
            Colour::Red
                .bold()
                .paint("No tokei data available for this directory.")
        );
    } else {
        if summary.inaccurate {
            println!(
                "{}",
                Colour::Fixed(172).bold().paint(
                    "Tokei encountered issues during parsing.\nThis data may not be accurate.\n"
                )
            );
        }

        summary.sort_by(Sort::Code); // This doesn't work either?? I'm triggered it won't sort by code.

        display_summary_table(summary);
    }
}

/// Create a table containing `Tokei` data.
fn display_summary_table(summary: Language) {
    let mut table = Table::new();

    table.max_column_width = 300;
    table.separate_rows = false;
    table.style = TableStyle::empty();

    let mut headers = vec![TableCell::new(
        Colour::White.bold().paint("Language").to_string(),
    )];
    headers.extend(
        vec!["Files", "Lines", "Code", "Comments", "Blanks"]
            .iter()
            .map(|header| {
                let mut cell = TableCell::new(
                    Colour::White
                        .bold()
                        .paint(&(*header).to_string())
                        .to_string(),
                );
                cell.alignment = Alignment::Right;

                cell
            })
            .collect::<Vec<TableCell>>(),
    );

    let header_row = Row::new(headers);
    table.add_row(header_row);
    table.add_row(Row::new(vec![" ", " ", " ", " ", " ", " "]));

    let (mut total_blanks, mut total_code, mut total_comments, mut total_files, mut total_lines) =
        (0, 0, 0, 0, 0);
    for (language_type, reports) in summary.children {
        let (mut blanks, mut code, mut comments, mut files, mut lines) = (0, 0, 0, 0, 0);

        for report in reports {
            blanks += report.stats.blanks;
            code += report.stats.code;
            comments += report.stats.comments;
            lines += report.stats.lines();
            files += 1;
        }

        let mut data = vec![TableCell::new(language_type.to_string())];
        data.extend(
            vec![
                format!("{files}"),
                format!("{lines}"),
                code.to_string(),
                format!("{comments}"),
                format!("{blanks}"),
            ]
            .iter()
            .map(|data| {
                let mut cell = TableCell::new(data);
                cell.alignment = Alignment::Right;

                cell
            })
            .collect::<Vec<TableCell>>(),
        );

        table.add_row(Row::new(data));

        total_blanks += blanks;
        total_code += code;
        total_comments += comments;
        total_files += files;
        total_lines += lines;
    }

    table.add_row(Row::new(vec![" ", " ", " ", " ", " ", " "]));

    let mut totals = vec![TableCell::new(
        Colour::White.bold().paint("Total").to_string(),
    )];
    totals.extend(
        vec![
            format!("{total_files}"),
            format!("{total_lines}"),
            format!("{total_code}"),
            format!("{total_comments}"),
            format!("{total_blanks}"),
        ]
        .iter()
        .map(|total| {
            let mut cell = TableCell::new(Colour::White.bold().paint(total).to_string());
            cell.alignment = Alignment::Right;

            cell
        })
        .collect::<Vec<TableCell>>(),
    );

    table.add_row(Row::new(totals));

    println!("\n{}", table.render());
}
