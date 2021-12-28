//! `oak` - The `tree` command, but better.

mod cli;

use std::{env, io::Result};

fn main() -> Result<()> {
    let args = cli::get_args();

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

    println!("THE TARGET DIRECTORY IS: {}", target_directory);

    Ok(())
}
