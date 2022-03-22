//! Defining command-line interface flags.

pub mod config;
pub mod filetype;
pub mod git;
pub mod global;
pub mod releases;

use structopt::StructOpt;

use self::{
    config::ConfigOptions,
    filetype::FileTypeOptions,
    git::GitOptions,
    global::GlobalArgs,
    releases::{ReleaseOptions, UpgradeOptions},
};

/// This struct contains all flags that are used in this program.
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(
    name = "nomad",
    about = "The customizable next gen tree command with Git integration and TUI",
    author = "Joseph Lai"
)]
pub struct Args {
    #[structopt(help = "Display a tree for this directory")]
    pub directory: Option<String>,

    #[structopt(flatten)]
    pub global: GlobalArgs,

    #[structopt(subcommand)]
    pub sub_commands: Option<SubCommands>,
}

#[derive(Debug, PartialEq, StructOpt)]
pub enum SubCommands {
    ///`bat` (the Rust alternative to the `cat` command) a file.
    /// This may be used after running nomad in a labeled mode.
    Bat { item_labels: Vec<String> },
    /// Customize/configure nomad or view your current configuration.
    ///
    /// Edit or view your settings defined in the self-instantiated configuration
    /// file `nomad.toml`.
    ///
    /// === NOTE ===
    ///
    /// You DO NOT have to create this file yourself. nomad will create
    /// it for you if it does not already exist on your system.
    Config(ConfigOptions),
    /// Edit a file with your default $EDITOR or with Neovim, Vim, Vi, or Nano.
    /// This may be used after running nomad in a labeled mode.
    Edit { item_labels: Vec<String> },
    /// Filter directory items by filetype.
    Ft(FileTypeOptions),
    /// Run commonly used Git commands.
    /// Some commands may be used after running nomad in a labeled mode.
    ///
    /// Use the `-h`/`--help` flags to see the available options for each command.
    Git(GitOptions),
    /// Retrieve releases for this program (retrieved from GitHub).
    Releases(ReleaseOptions),
    /// Enter rootless (interactive) mode.
    Rootless,
    /// Upgrade nomad or just check if there is an upgrade available.
    Upgrade(UpgradeOptions),
}

/// Return the `Args` struct.
pub fn get_args() -> Args {
    Args::from_args()
}

#[cfg(test)]
mod test_cli {
    use assert_cmd::Command;

    #[test]
    fn test_invalid_arg() {
        Command::cargo_bin("nd")
            .unwrap()
            .arg("-q")
            .assert()
            .failure();
    }
}
