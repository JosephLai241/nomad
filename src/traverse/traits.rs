//! Exposing traits for directory traversal/item parsing.

use super::models::{FoundItem, TransformedItem};
use crate::{
    cli::Args,
    errors::NomadError,
    traverse::{
        format::{format_content, format_directory},
        utils::{build_tree, check_nesting, get_file_icon, store_directory_contents},
    },
    ALPHABET,
};

use ptree::{item::StringItem, print_tree_with, PrintConfig};

use std::{
    collections::{HashMap, HashSet},
    path::{Component, Path},
    time::Instant,
};

/// Converts a `Vec<FoundItem>` into a `Vec<TransformedItem>`.
pub trait TransformFound {
    /// Converts a `Vec<FoundItem>` into a `Vec<TransformedItem>`.
    fn transform(self, target_directory: &str) -> Vec<TransformedItem>;
}

impl TransformFound for Vec<FoundItem> {
    fn transform(self, target_directory: &str) -> Vec<TransformedItem> {
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

        transformed
    }
}

/// Converts a `Vec<TransformedItem>` into a `ptree` `StringItem` with its corresponding
/// `PrintConfig`
pub trait ToTree {
    /// Convert the `Vec<TransformedItem` to a `StringItem` and its corresponding `PrintConfig`.
    fn to_tree(
        self,
        args: &Args,
        target_directory: &str,
    ) -> Result<(StringItem, PrintConfig), NomadError>;
}

impl ToTree for Vec<TransformedItem> {
    fn to_tree(
        self,
        args: &Args,
        target_directory: &str,
    ) -> Result<(StringItem, PrintConfig), NomadError> {
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
            path: target_directory.to_string(),
        };

        let (config, mut tree) = build_tree(args.metadata, args.plain, Path::new(target_directory));

        let start = Instant::now();
        for item in self.iter() {
            check_nesting(
                current_depth,
                Path::new(&target_directory)
                    .join(Path::new(&item.components.join("/").to_string()))
                    .as_path(),
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

                tree.begin_child(format_directory(
                    label,
                    Path::new(&item.path),
                    args.metadata,
                    args.no_icons,
                    args.plain,
                ));

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
                    item.marker.clone(),
                    icon,
                    Path::new(&item.path),
                    args.metadata,
                    args.no_git,
                    args.no_icons,
                    number,
                    args.plain,
                ));

                num_files += 1;
            }

            current_depth = item.depth as usize;
            previous_item = item;
        }

        store_directory_contents(labeled_items, numbered_items)?;

        let final_tree = tree.build();

        println!();
        print_tree_with(&final_tree, &config)?;
        println!();

        if args.statistics {
            let duration = start.elapsed().as_millis();
            println!("| {num_directories} directories | {num_files} files | {duration} ms |\n");
        }

        Ok((final_tree, config))
    }
}
