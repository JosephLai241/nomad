//! `nomad` - The `tree` command, but better.

mod cli;
mod models;
mod traverse;
mod utils;

use std::{env, io::Result};

use utils::icons::{get_icons_by_extension, get_icons_by_name};

/// Run `nomad`.
fn main() -> Result<()> {
    let args = cli::get_args();

    // Set the target directory based on whether a directory was passed in.
    let target_directory;
    if let Some(ref directory) = args.directory {
        target_directory = directory.clone();
    } else {
        // Get the current directory as the target if no target was entered.
        let directory = env::current_dir()?;
        target_directory = directory
            .into_os_string()
            .into_string()
            .expect("Could not get the current directory!")
            .clone();
    }

    let extension_icon_map = get_icons_by_extension();
    let name_icon_map = get_icons_by_name();

    if args.interactive {
        unimplemented!()
    } else if let Some(target_file) = args.open {
        utils::open::open_file(target_file)?;
    } else if let Some(file) = args.bat {
        utils::bat::run_bat(file)?;
    } else {
        let mut walker = traverse::build_walker(&args, &target_directory)?;
        traverse::walk_directory(&args, &extension_icon_map, &target_directory, &mut walker)?;
    }

    Ok(())
}
