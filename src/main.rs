//! `oak` - The `tree` command, but better.

mod cli;

use std::{env, io::Result};

fn main() -> Result<()> {
    let args = cli::get_args();

    if let Some(directory) = args.directory {
        println!("The directory passed into the `d` flag is: {}", directory);
        Ok(())
    } else {
        // Get the current directory as the target for oak if no target was entered.
        let directory = env::current_dir()?;
        println!("The current directory is: {}", directory.display());
        Ok(())
    }
}
