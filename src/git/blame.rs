//! Exposing functionality for the Git blame command.

use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{errors::NomadError, XTERM_COLORS};

use ansi_term::Colour;
use anyhow::Result;
use bat::{Input, PagingMode, PrettyPrinter, WrappingMode};
use git2::{BlameOptions, Repository};
use rand::{prelude::SliceRandom, thread_rng};

/// Use `bat` to display the Git blame.
pub fn bat_blame(
    filename: String,
    ranges: Vec<usize>,
    repo: &Repository,
    target_directory: &str,
) -> Result<(), NomadError> {
    let blame_meta = get_blame(&filename, ranges, repo, target_directory)?;
    let joined_blame = blame_meta.blame.join("\n");

    if let Err(error) = PrettyPrinter::new()
        .grid(true)
        .header(true)
        .input(Input::from_bytes(joined_blame.as_bytes()).name(format!(
            "| {} | {} {author_label} | {} {email_label} |",
            blame_meta.relative_path,
            Colour::Green.paint(blame_meta.authors.to_string()),
            Colour::Yellow.paint(blame_meta.emails.to_string()),
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
        .line_numbers(true)
        .paging_mode(PagingMode::QuitIfOneScreen)
        .rule(true)
        .true_color(true)
        .wrapping_mode(WrappingMode::Character)
        .print()
    {
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
    /// The relative path of the file.
    pub relative_path: String,
}

/// Traverse the Git blame hunks, format each line, and return a Vec containing the
/// formatted lines in the blame.
fn get_blame(
    filename: &str,
    ranges: Vec<usize>,
    repo: &Repository,
    target_directory: &str,
) -> Result<BlameMeta, NomadError> {
    let mut blame_options = BlameOptions::new();
    blame_options
        .track_copies_same_commit_copies(true)
        .track_copies_same_commit_moves(true)
        .track_copies_same_file(true);

    if !ranges.is_empty() {
        blame_options.min_line(*ranges.get(0).unwrap_or(&0));
        blame_options.max_line(*ranges.get(1).unwrap_or(&usize::MAX));
    }

    let relative_path = Path::new(&filename)
        .strip_prefix(target_directory)
        .unwrap_or(Path::new("?"));

    let blame = repo.blame_file(relative_path, Some(&mut blame_options))?;
    let you = repo.signature()?.name().unwrap_or("?").to_string();

    let object =
        repo.revparse_single(&format!("HEAD:{}", relative_path.to_str().unwrap_or("?")))?;
    let blob = repo.find_blob(object.id())?;
    let reader = BufReader::new(blob.content());

    let mut found_authors: HashMap<String, Option<u8>> = HashMap::new();
    let mut found_emails: HashSet<String> = HashSet::new();
    let mut formatted_blame: Vec<String> = Vec::new();

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

            let commit_id = repo.find_commit(hunk.final_commit_id())?.id().to_string();

            let blame = format!(
                "{} {} {} | {}",
                Colour::Fixed(028).paint(&commit_id[..7]),
                Colour::Fixed(193).paint(&author),
                Colour::Fixed(194).paint(&email),
                match found_authors.get(&author) {
                    Some(assigned_color) => {
                        match assigned_color {
                            Some(color) => Colour::Fixed(*color).paint(line).to_string(),
                            None => line,
                        }
                    }
                    None => line,
                }
            );

            formatted_blame.push(blame);
        }
    }

    Ok(BlameMeta {
        authors: found_authors.len(),
        blame: formatted_blame,
        emails: found_emails.len(),
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
        let mut new_color = false;
        let mut color = XTERM_COLORS.choose(&mut thread_rng()).unwrap_or(&007);

        while !new_color {
            // The Vec<u8> contains dark colors, which could make it hard to read text
            // in the terminal.
            if Vec::from_iter(found_authors.values().filter_map(|color| {
                if color.is_some() {
                    Some(color.unwrap())
                } else {
                    None
                }
            }))
            .contains(color)
                || vec![016, 017, 018, 019, 051, 053, 054, 055, 089, 088].contains(color)
            {
                color = XTERM_COLORS.choose(&mut thread_rng()).unwrap_or(&007);
            } else {
                new_color = true;
            }
        }

        Some(*color)
    }
}
