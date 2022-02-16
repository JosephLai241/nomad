//! Display the Git status command in tree form.

use std::{collections::HashMap, path::Path, time::Instant};

use ansi_term::Colour;
use git2::Repository;
use ignore::Walk;
use ptree::{print_tree, TreeBuilder};

use crate::{
    cli::Args, errors::NomadError, traverse::utils::store_directory_contents,
    utils::temp::JSONTarget,
};

use super::{
    markers::get_status_markers,
    utils::{add_marker_depths, strip_prefixes},
};

/// Build a tree that only contains items that are tracked in Git.
pub fn display_status_tree(
    args: &Args,
    extension_icon_map: &HashMap<&str, &str>,
    name_icon_map: &HashMap<&str, &str>,
    repo: &Repository,
    target_directory: &str,
    walker: &mut Walk,
) -> Result<(), NomadError> {
    get_status_markers(&repo, target_directory).map_or_else(
        |error| {
            //Err(std::io::Error::new(
            //ErrorKind::Other,
            //format!("Could not get Git status markers for this directory! {error}"),
            //))
            Err(error)
        },
        |marker_map| {
            if marker_map.is_empty() {
                println!(
                    "\n{}\n",
                    Colour::Fixed(172)
                        .paint(format!("No Git changes found in {target_directory}.\n"))
                );

                Ok(())
            } else {
                let sliced_markers = strip_prefixes(&target_directory, marker_map);

                build_status_tree(
                    args,
                    extension_icon_map,
                    name_icon_map,
                    sliced_markers,
                    target_directory,
                    walker,
                )?;

                Ok(())
            }
        },
    )
}

/// Traverse the repo and build the status tree.
fn build_status_tree(
    args: &Args,
    extension_icon_map: &HashMap<&str, &str>,
    name_icon_map: &HashMap<&str, &str>,
    mut sliced_markers: HashMap<String, String>,
    target_directory: &str,
    walker: &mut Walk,
) -> Result<(), NomadError> {
    //let (config, mut tree) = build_tree(args.metadata, &previous_item);

    let mut modified_items = add_marker_depths(sliced_markers);

    if modified_items.is_empty() {
        println!(
            "\n{}\n",
            Colour::Fixed(172)
                .bold()
                .paint("No modified items to display.")
        );
    } else {
        let mut tree = TreeBuilder::new(
            Path::new(target_directory)
                .to_str()
                .unwrap_or("?")
                .to_string(),
        );

        let mut numbered_items: HashMap<String, String> = HashMap::new();
        let mut labeled_items: HashMap<String, String> = HashMap::new();

        let current_directory = Path::new(target_directory);

        let mut current_depth: i32 = 0;
        let mut num_directories = 0;
        let mut num_files = 0;

        //let first_item = modified_items.remove(0);
        //let mut current_nesting = first_item.components;

        //let mut current_nesting = modified_items
        //.get(0)
        //.expect("Could not get the first modified item in this Git repository!")
        //.components
        //.clone();
        let mut current_nesting: Vec<String> = Vec::new();

        let start = Instant::now();
        for item in modified_items {
            if item.depth < current_depth {
                for _ in 0..current_depth - item.depth {
                    tree.end_child();
                }
            }

            println!("\nCURRENT ITEM: {}\n", item.path);
            for (index, section) in item.components.iter().enumerate() {
                println!("CHECKING: {section}");

                if current_nesting.is_empty() {
                    current_nesting.push(section.clone());
                }

                if let Some(nested_section) = current_nesting.get(index) {
                    // If the new directory level matches the current level,
                    // check if they are the same directory.
                    //
                    // If they are not identical directories, clear the current
                    // list of directories and overwrite it with the new nesting
                    // while also creating new tree.begin_childs for the new
                    // branches
                    //
                    // Otherwise just add the new file to the current branch.

                    //if !nested_section.eq(section) && !section.contains(".") {
                    if !nested_section.eq(section) && index != item.components.len() - 1 {
                        println!("NOT THE SAME DIR");

                        current_nesting.drain(index..);
                        current_nesting.push(section.clone());

                        //tree.end_child();
                        tree.begin_child(section.clone());
                    //} else if !nested_section.eq(section) && section.contains(".") {
                    } else if !nested_section.eq(section) && index == item.components.len() - 1 {
                        println!("ADDING NEW FILE TO THE SAME BRANCH.");
                        tree.add_empty_child(section.clone());
                    }
                } else {
                    // If the new item's nesting is deeper than the current nesting,
                    // extend the list of directories and create new tree.begin_childs
                    // for the new directory.
                    if index != item.components.len() - 1 {
                        println!("NEW LEVEL");

                        current_nesting.push(section.clone());
                        tree.begin_child(section.clone());
                    } else {
                        tree.add_empty_child(section.clone());
                    }
                }
            }
            //println!("{:?}", item);
            println!("\nCURRENT NESTING: {current_nesting:?}\n");

            current_depth = item.depth;
        }

        if args.numbers {
            store_directory_contents(numbered_items, JSONTarget::Contents)?;
        }
        if args.label_directories {
            store_directory_contents(labeled_items, JSONTarget::Directories)?;
        }

        println!();
        print_tree(&tree.build()).expect("COULDN'T PRINT TREE");

        //print_tree_with(&tree.build(), &config).expect(&format!(
        //"\n{}\n",
        //Colour::Red
        //.bold()
        //.paint("Could not display the Git status tree!")
        //));
        println!();

        if args.statistics {
            let duration = start.elapsed().as_millis();
            println!("{num_directories} directories | {num_files} files | {duration} ms\n");
        }
    }

    let start = Instant::now();
    for _ in 0..1 {
        //for (path, marker) in sliced_markers.iter().sorted() {
        //let item = Path::new(path);
        //println!("CHECKING: {}", item.display());
        //let mut depth = 0;

        //for component in item.components() {
        //match component {
        //Component::Normal(section) => {
        //let section_path = Path::new(section);

        //if let Some(_) = section_path.extension() {
        //if current_depth > depth {
        //println!(
        //"CURRENT DEPTH IS: {current_depth} BUT THIS DEPTH IS: {depth}"
        //);
        ////for _ in 0..current_depth - depth {
        ////tree.end_child();
        ////}
        //}
        //tree.add_empty_child(format!(
        //"{marker} {section_string}",
        //section_string = section_path.to_str().unwrap_or("?").to_string(),
        //));
        //} else {
        //let section_string = section_path.to_str().unwrap_or("?").to_string();
        //let icon = "\u{f115}".to_string(); // 

        //if current_nesting.is_empty() {
        //tree.begin_child(format!("{icon} {section_string}"));

        //current_nesting.push(section_string);
        //} else if !current_nesting.contains(&section_string) {
        //tree.begin_child(format!("{icon} {section_string}"));

        //if current_nesting.len() - 1 == depth
        //&& current_nesting.get(depth).unwrap().to_string() == section_string
        //{
        //current_nesting.pop();
        //}

        //current_nesting.push(section_string);
        //}
        //}
        //println!("CURRENT NESTING: {:?}", current_nesting);
        //depth += 1;
        //}
        //_ => {}
        //}
        //}

        //current_depth = depth;

        //if item.is_dir() {
        //check_nesting(current_depth, &item, previous_item, &mut tree);

        //let icon = "\u{f115}".to_string(); // 
        //tree.begin_child(format_directory(
        //icon,
        //None, // TODO: ADD DIRECTORY LABEL FOR LABELED-DIRECTORIES FLAG
        //&item,
        //args.metadata,
        //args.mute_icons,
        //));

        //num_directories += 1;

        //current_depth = item.depth();
        //previous_item = item;
        //} else if item.is_file() {
        //check_nesting(current_depth, &item, previous_item, &mut tree);

        //let number = if args.numbers {
        //numbered_items.insert(
        //format!("{num_files}"),
        //item.path()
        //.canonicalize()
        //.unwrap_or(PathBuf::from("?"))
        //.into_os_string()
        //.into_string()
        //.unwrap_or("?".into()),
        //);

        //Some(num_files)
        //} else {
        //None
        //};

        //let icon = get_file_icon(extension_icon_map, &item, name_icon_map);
        //tree.add_empty_child(format_content(
        //Some(git_marker.to_owned()),
        //icon,
        //&item,
        //args.metadata,
        //args.mute_icons,
        //number,
        //));

        //num_files += 1;

        //current_depth = item.depth();
        //previous_item = item;
        //}
    }

    Ok(())
}
