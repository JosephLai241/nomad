//! `nomad` - The `tree` command, but better.

mod cli;
mod models;
mod traverse;
mod ui;
mod utils;

use tokio;
use ui::spawn_terminal;

use std::{env, io::Result};

use utils::{
    icons::{get_icons_by_extension, get_icons_by_name},
    open::get_file,
};

/// Run `nomad`.
#[tokio::main]
async fn main() -> Result<()> {
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
        let mut walker = traverse::build_walker(&args, &target_directory)?;
        let _ = spawn_terminal(&args, &target_directory, &mut walker)
            .await
            .map_err(|error| format!("UI ERROR: {}", error));
    } else if let Some(target) = args.open {
        let target_file = get_file(target)?;
        utils::open::open_file(target_file)?;
    } else if let Some(target) = args.bat {
        let target_file = get_file(target)?;
        utils::bat::run_bat(target_file)?;
    } else {
        let mut walker = traverse::build_walker(&args, &target_directory)?;
        traverse::walk_directory(&args, &extension_icon_map, &target_directory, &mut walker)?;
    }

    Ok(())
}
