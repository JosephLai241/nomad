//! `nomad` - The `tree` command, but better.

mod cli;
mod models;
mod traverse;
mod ui;
mod utils;

use ansi_term::Colour;
use tokio;

use std::io::Result;

use cli::{Git, GitOptions};
use ui::{spawn_terminal, utils::convert_tree};
use utils::{
    git::get_repo,
    icons::{get_icons_by_extension, get_icons_by_name},
    open::get_file,
    paths::get_current_directory,
};

/// Run `nomad`.
#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::get_args();

    // Set the target directory based on whether a directory was passed in.
    let target_directory = if let Some(ref directory) = args.directory {
        directory.clone()
    } else {
        get_current_directory().unwrap_or("?".to_string())
    };

    let extension_icon_map = get_icons_by_extension();
    let name_icon_map = get_icons_by_name();

    let repo = Some(get_repo(&target_directory)).flatten();

    if args.interactive {
        // TODO: RESERVE FOR NOMAD V0.1.1?

        let mut walker = traverse::build_walker(&args, &target_directory)?;
        let tree_items = traverse::walk_directory(
            &args,
            &extension_icon_map,
            &name_icon_map,
            repo,
            &mut walker,
        )?;
        if let Some((tree, config)) = tree_items {
            let tree_parts = convert_tree(config, tree);
            println!("{:?}", tree_parts);
        }

        let _ = spawn_terminal(&args, &target_directory, &mut walker)
            .await
            .map_err(|error| format!("UI ERROR: {error}"));
    } else if let Some(target) = args.open {
        let target_file = get_file(target)?;
        utils::open::open_file(target_file)?;
    } else if let Some(target) = args.bat {
        let target_file = get_file(target)?;
        utils::bat::run_bat(target_file)?;
    } else if args.git.is_some() {
        ////////////////// TODO: REFACTOR?
        if repo.is_some() {
            if let Some(sub_command) = args.git {
                match sub_command {
                    Git::Git(git_command) => match git_command {
                        GitOptions::Add { file_number } => {
                            let target_file = get_file(file_number.to_string())?;
                            println!("\nTARGET FILE FROM GIT ADD IS: {target_file}\n");
                            // TODO: ADD METHOD TO DO A `git add <file_number>`
                        }
                        GitOptions::Diff { file_number } => {
                            let target_file = get_file(file_number.to_string())?;
                            utils::bat::run_bat(target_file)?;
                        }
                        _ => {}
                    },
                }
            }
        } else {
            println!(
                "\n{}\n",
                Colour::Red
                    .bold()
                    .paint("Cannot run Git commands in this directory. Not a Git repository!")
            );
        }
    } else {
        let mut walker = traverse::build_walker(&args, &target_directory)?;
        let tree_items = traverse::walk_directory(
            &args,
            &extension_icon_map,
            &name_icon_map,
            repo,
            &mut walker,
        )?;

        if let (Some(file_name), Some((tree, config))) = (args.export, tree_items) {
            utils::export::export_tree(config, file_name, tree)?;
        }
    }

    Ok(())
}
