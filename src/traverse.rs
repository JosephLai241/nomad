//! Traverse the target directory while respecting rules set in ignore-type files.

use std::{env, io::Error};

use crate::cli::Args;

use ignore::{DirEntry, Walk, WalkBuilder};

pub fn build_walker(args: Args) -> Result<Walk, Error> {
    // Set the target directory based on whether a directory was passed into `oak`.
    let target_directory;
    if let Some(directory) = args.directory {
        target_directory = directory;
    } else {
        // Get the current directory as the target for oak if no target was entered.
        // Panic if Rust can't get the current directory.
        let directory = env::current_dir()?;
        target_directory = directory.to_str().unwrap().into();
    }

    println!("\nTHE TARGET DIRECTORY IS: {}\n", target_directory);

    Ok(WalkBuilder::new(target_directory)
        .git_exclude(!args.disrespect)
        .git_global(!args.disrespect)
        .git_ignore(!args.disrespect)
        .ignore(!args.disrespect)
        .hidden(!args.hidden)
        .parents(!args.disrespect)
        .build())
}

fn display_directory(entry: DirEntry) {
    println!("DIRECTORY IS: {:?}", entry.path().display());
}

fn display_file(entry: DirEntry) {
    println!("FILE IS: {:?}", entry.file_name().to_str());
}

pub fn walk_directory(mut walker: Walk) -> Result<(), Error> {
    while let Some(Ok(entry)) = walker.next() {
        if entry.path().is_dir() {
            display_directory(entry);
        } else if entry.path().is_file() {
            display_file(entry);
        } else {
            unimplemented!();
        }
    }

    Ok(())
}
