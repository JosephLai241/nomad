//! Exposing functionality for the Git branch command.

use std::time::Instant;

use crate::{
    cli::{
        git::BranchOptions,
        global::{GlobalArgs, LabelArgs, MetaArgs, ModifierArgs, RegexArgs, StyleArgs},
    },
    errors::NomadError,
    style::models::NomadStyle,
    traverse::{
        format::highlight_matched,
        models::{DirItem, FoundBranch},
        modes::NomadMode,
        traits::{ToTree, TransformFound},
    },
};

use ansi_term::Colour;
use anyhow::{private, Result};
use git2::{Branch, BranchType, Repository};
use ptree::{item::StringItem, PrintConfig};
use regex::Regex;

use super::utils::get_repo_branch;

/// Get all local branches from the repository and transform them into a `Vec<FoundBranch>`.
pub fn display_branches(
    args: &BranchOptions,
    nomad_style: &NomadStyle,
    repo: &Repository,
    target_directory: &str,
) -> Result<Option<(StringItem, PrintConfig, Option<Vec<DirItem>>)>, NomadError> {
    let regex_expression = if let Some(ref pattern) = args.pattern {
        match Regex::new(&pattern.clone()) {
            Ok(regex) => Some(regex),
            Err(error) => return private::Err(NomadError::RegexError(error)),
        }
    } else {
        None
    };

    let mut branches: Vec<FoundBranch> = Vec::new();
    let current_branch = get_repo_branch(repo);

    // At first I tried to do something like this to iterate over all branches:
    //
    //     ```rust
    //     while let Some(Ok((branch, _branch_type))) = repo.branches(Some(BranchType::Local))?.next() {
    //          // Do stuff to format the branch and shit.
    //     }
    //     ```
    //     * NOTE: This codeblock might not be entirely correct. I just wrote
    //             what I remember.
    //
    // Usually the `while let Some(Ok(thing)) = iterator.next()` syntax would work
    // and would avoid the need to `collect()` iterator items into a `Vec<T>`
    // before iterating, but I ran into some strange issues:
    //
    //     * Calling `next()` would not move to the next found branch - it would
    //       infinitely loop on the same found branch, which is the first branch
    //       that was found.
    //     * If I created a `HashSet` containing visited branches to avoid the
    //       infinite loop, iteration would be broken after only a single repo
    //       branch was found. This means only one branch for a given repo would
    //       be "found".
    //
    // This is why I had to `collect()` the `Branch`es into a `Vec<Branch>` prior
    // to iteration. If anyone sees this and knows why this is happening, please
    // let me know because I have no idea what the hell is causing this behavior.
    // Maybe I had to use the unsafe `from_raw()` method?
    //
    // Documentation for `git2::Branches` is available here:
    //
    //     https://docs.rs/git2/latest/git2/struct.Branches.html
    //
    let repo_branches = repo
        .branches(Some(BranchType::Local))?
        .filter_map(|repo_branch| {
            if let Ok((branch, _branch_type)) = repo_branch {
                Some(branch)
            } else {
                None
            }
        })
        .collect::<Vec<Branch>>();

    if args.flat {
        println!();
    }

    let mut num_branches = 0;
    let start = Instant::now();
    for branch in repo_branches {
        let branch_name = branch.name()?.unwrap_or("?").to_string();

        let mut is_current_branch = false;
        let marker = match current_branch {
            Some(ref current_branch_name) => {
                if &branch_name == current_branch_name {
                    is_current_branch = true;
                    Some(format!("{}", Colour::Green.bold().paint("*")))
                } else {
                    None
                }
            }
            None => None,
        };
        let upstream = if let Ok(upstream) = branch.upstream() {
            let mut upstream_branch = format!(
                " => {}",
                Colour::Blue
                    .bold()
                    .paint(upstream.name()?.unwrap_or("?").to_string())
            );

            if upstream.is_head() {
                upstream_branch.push_str(&format!(" [{}]", Colour::Red.bold().paint("HEAD")));
            }

            Some(upstream_branch)
        } else {
            None
        };
        let number = if args.numbers {
            Some(num_branches)
        } else {
            None
        };

        if let Some(ref regex) = regex_expression {
            if let Some(matched) = regex.find(&branch_name) {
                if args.flat {
                    display_flat_branch(
                        &branch,
                        &branch_name,
                        is_current_branch,
                        marker,
                        Some((matched.start(), matched.end())),
                        nomad_style,
                        number,
                        upstream,
                    );
                } else {
                    branches.push(FoundBranch {
                        full_branch: branch_name.clone(),
                        is_current_branch,
                        is_head: branch.is_head(),
                        marker,
                        matched: Some((matched.start(), matched.end())),
                        upstream,
                    });
                }
            }
        } else if args.flat {
            display_flat_branch(
                &branch,
                &branch_name,
                is_current_branch,
                marker,
                None,
                nomad_style,
                number,
                upstream,
            );
        } else {
            branches.push(FoundBranch {
                full_branch: branch_name.clone(),
                is_current_branch,
                is_head: branch.is_head(),
                marker,
                matched: None,
                upstream,
            });
        }

        num_branches += 1;
    }

    if args.flat {
        println!();

        if args.statistics {
            let duration = start.elapsed().as_millis();
            println!("| {num_branches} branches | {duration} ms |\n");
        }
    }

    // Hm... There is probably a better solution, but fuck it. Leaving it for now.
    let global_args = GlobalArgs {
        export: args.export.clone(),
        labels: LabelArgs {
            all_labels: false,
            label_directories: false,
            numbers: args.numbers,
        },
        meta: MetaArgs {
            loc: false,
            metadata: false,
            no_tree: false,
            summarize: false,
        },
        modifiers: ModifierArgs {
            dirs: false,
            disrespect: false,
            hidden: false,
            max_depth: None,
            max_filesize: None,
        },
        regex: RegexArgs {
            pattern: args.pattern.clone(),
        },
        style: StyleArgs {
            no_colors: false,
            no_git: false,
            no_icons: args.no_icons,
            plain: false,
        },
        statistics: args.statistics,
    };

    Ok(if args.flat {
        None
    } else {
        Some(branches.transform(target_directory)?.to_tree(
            &global_args,
            NomadMode::GitBranch,
            nomad_style,
            target_directory,
        )?)
    })
}

/// Format the branch into a flat view and then display it
/// This is like the standard `git branch` or `git branch --list` commands.
fn display_flat_branch(
    branch: &Branch,
    branch_name: &str,
    is_current_branch: bool,
    marker: Option<String>,
    matched: Option<(usize, usize)>,
    nomad_style: &NomadStyle,
    number: Option<i32>,
    upstream: Option<String>,
) {
    let branch_label = match matched {
        Some(matched) => highlight_matched(
            false,
            nomad_style,
            branch_name.to_string(),
            (matched.0, matched.1),
        ),
        None => branch_name.to_string(),
    };
    let formatted_branch = if is_current_branch {
        Colour::Green.bold().paint(branch_label).to_string()
    } else {
        branch_label
    };

    let number_label = match number {
        Some(number) => format!(
            "[{}] ",
            nomad_style
                .tree
                .label_colors
                .item_labels
                .paint(format!("{number}"))
        ),
        None => "".to_string(),
    };
    let marker_label = match marker {
        Some(marker) => format!("{} ", Colour::Green.bold().paint(marker)),
        None => "".to_string(),
    };
    let head_label = if branch.is_head() {
        format!(" [{}]", Colour::Red.bold().paint("HEAD"))
    } else {
        "".to_string()
    };
    let upstream_label = match upstream {
        Some(upstream_branch) => upstream_branch,
        None => "".to_string(),
    };

    println!(
        "{}{}{}{}{}",
        number_label, marker_label, formatted_branch, head_label, upstream_label
    );
}
