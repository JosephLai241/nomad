//! Exposing functionality for the Git blame command.

use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{cli, errors::NomadError, utils::meta::convert_time, XTERM_COLORS};

use ansi_term::Colour;
use anyhow::Result;
use bat::{Input, PagingMode, PrettyPrinter, WrappingMode};
use git2::{BlameOptions, Repository};
use lazy_static::lazy_static;
use rand::{prelude::SliceRandom, thread_rng};

lazy_static! {
    /// Blacklisted colors for Git blame. These colors are darker and may be difficult
    /// to read or are too similar to the default shade of white.
    static ref BLACKLIST: Vec<u8> = vec![
        000, 001, 004, 005, 015, 016, 017, 018, 019, 020, 049, 051, 052, 053, 054,
        055, 056, 057, 058, 059, 060, 061, 062, 081, 085, 086, 087, 088, 089, 090,
        091, 092, 121, 122, 123, 124, 125, 126, 157, 158, 159, 192, 193, 194, 195,
        218, 224, 225, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239,
        240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254,
        255
    ];
}

/// Use `bat` to display the Git blame.
pub fn bat_blame(
    filename: String,
    blame_options: &cli::git::BlameOptions,
    repo: &Repository,
    target_directory: &str,
) -> Result<(), NomadError> {
    let blame_meta = get_blame(blame_options, &filename, repo, target_directory)?;
    let joined_blame = blame_meta.blame.join("\n");

    let mut printer = PrettyPrinter::new();
    printer
        .grid(true)
        .header(true)
        .input(Input::from_bytes(joined_blame.as_bytes()).name(format!(
            "| {} | {} {author_label} | {} {email_label} |{}",
            blame_meta.relative_path,
            Colour::Green.paint(blame_meta.authors.to_string()),
            Colour::Yellow.paint(blame_meta.emails.to_string()),
            if let Some(ranges) = blame_meta.lines {
                format!(
                    " Lines {} to {} |",
                    Colour::Fixed(193).paint(format!("{}", ranges.0)),
                    Colour::Fixed(193).paint(format!("{}", ranges.1))
                )
            } else {
                "".to_string()
            },
            author_label = if blame_meta.authors > 1 {
                "authors"
            } else {
                "author"
            },
            email_label = if blame_meta.emails > 1 {
                "emails"
            } else {
                "email"
            },
        )))
        .paging_mode(PagingMode::QuitIfOneScreen)
        .rule(true)
        .true_color(true)
        .wrapping_mode(WrappingMode::Character);

    if blame_options.lines.is_empty() {
        printer.line_numbers(true);
    }

    if let Err(error) = printer.print() {
        return Err(NomadError::BatError(error));
    }

    Ok(())
}

/// Contains metadata for the Git blame.
struct BlameMeta {
    /// The total number of different authors for this file.
    pub authors: usize,
    /// The formatted lines of the blame.
    pub blame: Vec<String>,
    /// The total number of different emails for this file.
    pub emails: usize,
    /// The line numbers that were targeted in the blame.
    pub lines: Option<(usize, usize)>,
    /// The relative path of the file.
    pub relative_path: String,
}

/// Traverse the Git blame hunks, format each line, and return a Vec containing the
/// formatted lines in the blame.
fn get_blame(
    cli_blame_options: &cli::git::BlameOptions,
    filename: &str,
    repo: &Repository,
    target_directory: &str,
) -> Result<BlameMeta, NomadError> {
    let mut blame_options = BlameOptions::new();
    blame_options
        .track_copies_same_commit_copies(true)
        .track_copies_same_commit_moves(true)
        .track_copies_same_file(true);

    if !cli_blame_options.lines.is_empty() {
        blame_options.min_line(*cli_blame_options.lines.get(0).unwrap_or(&0));
        blame_options.max_line(*cli_blame_options.lines.get(1).unwrap_or(&usize::MAX));
    }

    let relative_path = Path::new(&filename)
        .strip_prefix(target_directory)
        .unwrap_or_else(|_| Path::new("?"));

    let blame = repo.blame_file(relative_path, Some(&mut blame_options))?;
    let you = repo.signature()?.name().unwrap_or("?").to_string();

    let object =
        repo.revparse_single(&format!("HEAD:{}", relative_path.to_str().unwrap_or("?")))?;
    let blob = repo.find_blob(object.id())?;
    let reader = BufReader::new(blob.content());

    let mut found_authors: HashMap<String, Option<u8>> = HashMap::new();
    let mut found_emails: HashSet<String> = HashSet::new();
    let mut formatted_blame: Vec<String> = Vec::new();

    let mut final_line_num: usize = 0;
    for (index, line) in reader.lines().enumerate() {
        if let (Ok(line), Some(hunk)) = (line, blame.get_line(index + 1)) {
            let author = hunk
                .final_signature()
                .name()
                .unwrap_or("Unknown author")
                .to_string();
            if !found_authors.contains_key(&author) {
                found_authors.insert(
                    author.clone(),
                    get_random_color(&author, &found_authors, &you),
                );
            }

            let email = hunk
                .final_signature()
                .email()
                .unwrap_or("Unknown email")
                .to_string();
            if !found_emails.contains(&email) {
                found_emails.insert(email.clone());
            }

            let mut formatted_author = if author.len() > 12 {
                format!("{}..", &author[..11])
            } else {
                author.clone()
            };

            let formatted_meta = if cli_blame_options.emails {
                let formatted_email = if email.len() > 26 {
                    format!("{}..", &email[..25])
                } else {
                    email.clone()
                };

                format!("{:<27}", formatted_email)
            } else {
                let timestamp = convert_time(hunk.final_signature().when().seconds());

                format!("{:<24}", timestamp)
            };

            formatted_author = format!("{:<13}", formatted_author);

            let commit_id = repo.find_commit(hunk.final_commit_id())?.id().to_string();

            formatted_blame.push(format!(
                "{} {} {} | {}",
                Colour::Fixed(028).paint(&commit_id[..7]),
                Colour::Fixed(193).paint(&formatted_author),
                Colour::Fixed(194).paint(&formatted_meta),
                match found_authors.get(&author) {
                    Some(Some(color)) => Colour::Fixed(*color).paint(line).to_string(),
                    _ => line,
                }
            ));

            final_line_num = index + 1;
        }
    }

    Ok(BlameMeta {
        authors: found_authors.len(),
        blame: formatted_blame,
        emails: found_emails.len(),
        lines: if !cli_blame_options.lines.is_empty() {
            Some((
                cli_blame_options
                    .lines
                    .get(0)
                    .unwrap_or(&usize::MIN)
                    .to_owned(),
                final_line_num,
            ))
        } else {
            None
        },
        relative_path: relative_path.to_str().unwrap_or("?").to_string(),
    })
}

/// Randomly pick a color within the `XTERM_COLORS` Vec. Pick another color if
/// the selected color is already assigned to an author.
fn get_random_color(
    author: &str,
    found_authors: &HashMap<String, Option<u8>>,
    you: &str,
) -> Option<u8> {
    if author == you {
        None
    } else {
        let taken_colors = Vec::from_iter(
            found_authors
                .values()
                .filter_map(|color| color.as_ref().map(|color| color.to_owned())),
        );
        let mut color = XTERM_COLORS.choose(&mut thread_rng()).unwrap_or(&007);

        let mut new_color = false;
        while !new_color {
            if taken_colors.contains(color) || BLACKLIST.contains(color) {
                color = XTERM_COLORS.choose(&mut thread_rng()).unwrap_or(&007);
            } else {
                new_color = true;
            }
        }

        Some(*color)
    }
}
