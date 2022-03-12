//! Exposing traits for directory traversal/item parsing.

use super::{
    format::format_branch,
    models::{FoundBranch, FoundItem, TransformedBranch, TransformedItem},
    modes::NomadMode,
};
use crate::{
    cli::Args,
    errors::NomadError,
    style::models::NomadStyle,
    traverse::{
        format::{format_content, format_directory},
        utils::{build_tree, check_nesting, get_file_icon, store_directory_contents},
    },
    ALPHABET,
};

use ansi_term::Colour;
use ptree::{item::StringItem, print_tree_with, PrintConfig};

use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    path::{Component, Path},
    time::Instant,
};

/// Transform found items into a new struct containing useful metadata for tree building.
pub trait TransformFound<T> {
    /// Transforms a `Vec<U>` containing found items into a `Vec<T>` for tree building.
    fn transform(self, target_directory: &str) -> Result<Vec<T>, NomadError>;
}

impl TransformFound<TransformedItem> for Vec<FoundItem> {
    /// Transforms a `Vec<FoundItem>` into a `Vec<TransformedItem>`.
    fn transform(self, target_directory: &str) -> Result<Vec<TransformedItem>, NomadError> {
        if self.is_empty() {
            return Err(NomadError::NothingFound);
        }

        let mut transformed: Vec<TransformedItem> = Vec::new();
        let mut directories: HashSet<String> = HashSet::new();

        for found_item in self.iter() {
            let item = Path::new(&found_item.path)
                .strip_prefix(target_directory)
                .unwrap_or(Path::new("?"));

            let mut components = Vec::new();
            let mut depth = 0;
            for (index, component) in item.components().enumerate() {
                match component {
                    Component::Normal(section) => {
                        components.push(section.to_str().unwrap_or("?").to_string());
                        depth += 1;

                        let joined_path = components.join("/").to_string();

                        if index < item.components().count() - 1
                            && !directories.contains(&joined_path)
                        {
                            transformed.push(TransformedItem {
                                components: components.clone(),
                                depth,
                                is_dir: true,
                                is_file: false,
                                marker: None,
                                matched: None,
                                path: Path::new(target_directory)
                                    .join(joined_path)
                                    .to_str()
                                    .unwrap_or("?")
                                    .to_string(),
                            });

                            directories.insert(components.join("/").to_string());
                        } else if index == item.components().count() - 1 {
                            transformed.push(TransformedItem {
                                components: components.clone(),
                                depth,
                                is_dir: false,
                                is_file: true,
                                marker: found_item.marker.clone(),
                                matched: found_item.matched,
                                path: Path::new(target_directory)
                                    .join(joined_path)
                                    .to_str()
                                    .unwrap_or("?")
                                    .to_string(),
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(transformed)
    }
}

impl TransformFound<TransformedBranch> for Vec<FoundBranch> {
    /// Transforms a `Vec<FoundBranch>` into a `Vec<TransformedBranch>`.
    fn transform(self, _: &str) -> Result<Vec<TransformedBranch>, NomadError> {
        if self.is_empty() {
            return Err(NomadError::NothingFound);
        }

        let mut transformed: Vec<TransformedBranch> = Vec::new();
        let mut branch_parents: HashSet<String> = HashSet::new();

        for found_branch in self.iter() {
            let item = Path::new(&found_branch.full_branch);

            let mut components = Vec::new();
            let mut depth = 0;
            for (index, component) in item.components().enumerate() {
                match component {
                    Component::Normal(section) => {
                        components.push(section.to_str().unwrap_or("?").to_string());
                        depth += 1;

                        let joined_branch_name = components.join("/").to_string();

                        if index < item.components().count() - 1
                            && !branch_parents.contains(&joined_branch_name)
                        {
                            transformed.push(TransformedBranch {
                                components: components.clone(),
                                depth,
                                full_branch: Path::new(&joined_branch_name)
                                    .to_str()
                                    .unwrap_or("?")
                                    .to_string(),
                                is_current_branch: found_branch.is_current_branch,
                                is_end: false,
                                is_head: found_branch.is_head,
                                is_parent: true,
                                marker: None,
                                matched: None,
                                upstream: found_branch.upstream.clone(),
                            });

                            branch_parents.insert(components.join("/").to_string());
                        } else if index == item.components().count() - 1 {
                            transformed.push(TransformedBranch {
                                components: components.clone(),
                                depth,
                                full_branch: Path::new(&joined_branch_name)
                                    .to_str()
                                    .unwrap_or("?")
                                    .to_string(),
                                is_current_branch: found_branch.is_current_branch,
                                is_end: true,
                                is_head: found_branch.is_head,
                                is_parent: false,
                                marker: found_branch.marker.clone(),
                                matched: found_branch.matched,
                                upstream: found_branch.upstream.clone(),
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(transformed)
    }
}

/// Converts a `Vec<TransformedItem>` into a `ptree` `StringItem` with its corresponding
/// `PrintConfig`
pub trait ToTree {
    /// Convert the `Vec<TransformedItem>` to a `StringItem` and its corresponding `PrintConfig`.
    /// May also return a `Vec` containing all directory items depending on the `NomadMode`.
    fn to_tree(
        self,
        args: &Args,
        nomad_mode: NomadMode,
        nomad_style: &NomadStyle,
        target_directory: &str,
    ) -> Result<(StringItem, PrintConfig, Option<Vec<String>>), NomadError>;
}

impl ToTree for Vec<TransformedItem> {
    /// Build a tree from the `Vec<TransformedItem>`.
    fn to_tree(
        self,
        args: &Args,
        nomad_mode: NomadMode,
        nomad_style: &NomadStyle,
        target_directory: &str,
    ) -> Result<(StringItem, PrintConfig, Option<Vec<String>>), NomadError> {
        let mut numbered_items: HashMap<String, String> = HashMap::new();
        let mut labeled_items: HashMap<String, String> = HashMap::new();

        let mut current_depth = 0;
        let mut letter_index = 0; // The index pointing to a letter in the alphabet.
        let mut loop_count = 0; // Count the number of times the alphabet has been looped.
        let mut num_directories = 0;
        let mut num_files = 0;
        let mut previous_item = &TransformedItem {
            components: vec![],
            depth: 0,
            is_dir: true,
            is_file: false,
            marker: None,
            matched: None,
            path: target_directory.to_string(),
        };

        // This holds every single item in the directory and is only returned in
        // NomadMode::Interactive.
        let mut directory_items = Vec::new();
        match nomad_mode {
            NomadMode::Interactive => directory_items.push(target_directory.to_string()),
            _ => {}
        }

        let (config, mut tree) = build_tree(&args, &nomad_mode, Path::new(target_directory));

        let start = Instant::now();
        for item in self.iter() {
            check_nesting(
                current_depth,
                Path::new(&target_directory)
                    .join(Path::new(&item.components.join("/").to_string()))
                    .as_path(),
                &nomad_mode,
                Path::new(&target_directory)
                    .join(Path::new(&previous_item.components.join("/").to_string()))
                    .as_path(),
                target_directory,
                &mut tree,
            );

            if item.is_dir {
                if letter_index == 26 {
                    loop_count += 1;
                    letter_index = 0;
                }

                let mut directory_label = ALPHABET.get(letter_index).unwrap_or(&'?').to_string();

                if loop_count > 0 {
                    directory_label.push_str(&loop_count.to_string());
                }

                labeled_items.insert(format!("{directory_label}"), item.path.to_string());

                letter_index += 1;

                let label = if args.label_directories || args.all_labels {
                    Some(directory_label)
                } else {
                    None
                };

                tree.begin_child(format_directory(&args, label, Path::new(&item.path)));

                num_directories += 1;
            } else if item.is_file && !args.dirs {
                numbered_items.insert(format!("{num_files}"), item.path.to_string());

                let number = if args.numbers || args.all_labels {
                    Some(num_files)
                } else {
                    None
                };

                let icon = get_file_icon(Path::new(&item.path));
                tree.add_empty_child(format_content(
                    &args,
                    item.marker.clone(),
                    icon,
                    Path::new(&item.path),
                    item.matched,
                    nomad_style,
                    number,
                ));

                num_files += 1;
            }

            current_depth = item.depth as usize;
            previous_item = item;

            match nomad_mode {
                NomadMode::Interactive => directory_items.push(
                    Path::new(&item.path)
                        .canonicalize()?
                        .to_str()
                        .unwrap_or("?")
                        .to_string(),
                ),
                _ => {}
            }
        }

        store_directory_contents(labeled_items, numbered_items)?;

        let final_tree = tree.build();

        match nomad_mode {
            NomadMode::Normal => {
                println!();
                print_tree_with(&final_tree, &config)?;
                println!();
            }
            _ => {}
        }

        if args.statistics {
            let duration = start.elapsed().as_millis();
            println!("| {num_directories} directories | {num_files} files | {duration} ms |\n");
        }

        Ok((
            final_tree,
            config,
            match nomad_mode {
                NomadMode::Interactive => Some(directory_items),
                _ => None,
            },
        ))
    }
}

impl ToTree for Vec<TransformedBranch> {
    /// Build a tree from the `Vec<TransformedBranch>`.
    fn to_tree(
        self,
        args: &Args,
        nomad_mode: NomadMode,
        nomad_style: &NomadStyle,
        target_directory: &str,
    ) -> Result<(StringItem, PrintConfig, Option<Vec<String>>), NomadError> {
        let _labeled_items: HashMap<String, String> = HashMap::new();
        let mut numbered_items: HashMap<String, String> = HashMap::new();

        let mut current_depth = 0;
        let mut num_branches = 0;
        let mut previous_item = &TransformedBranch {
            components: vec![],
            depth: 0,
            full_branch: target_directory.to_string(),
            is_current_branch: false,
            is_end: false,
            is_head: false,
            is_parent: true,
            marker: Some("\u{f1d3}".to_string()),
            matched: None,
            upstream: None,
        };

        let (config, mut tree) = build_tree(args, &nomad_mode, Path::new(target_directory));

        for item in self.iter() {
            check_nesting(
                current_depth,
                Path::new(&item.components.join("/")),
                &nomad_mode,
                Path::new(&previous_item.components.join("/")),
                target_directory,
                &mut tree,
            );

            if item.is_parent {
                tree.begin_child(format!(
                    "{}",
                    Colour::Blue.bold().paint(
                        Path::new(&item.full_branch)
                            .file_name()
                            .unwrap_or(&OsStr::new("?"))
                            .to_str()
                            .unwrap_or("?")
                    )
                ));
            } else if item.is_end {
                numbered_items.insert(format!("{num_branches}"), item.full_branch.to_string());

                let number = if args.numbers || args.all_labels {
                    Some(num_branches)
                } else {
                    None
                };

                tree.add_empty_child(format_branch(&item, nomad_style, number));

                num_branches += 1;
            }

            current_depth = item.depth as usize;
            previous_item = item;
        }

        store_directory_contents(_labeled_items, numbered_items)?;

        let final_tree = tree.build();

        println!();
        print_tree_with(&final_tree, &config)?;
        println!();

        Ok((final_tree, config, None))
    }
}
