//! Defining command-line interface flags.

use structopt::StructOpt;

/// This struct contains all flags that are used in this program.
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(
    name = "nomad",
    about = "The next gen tree command",
    author = "Joseph Lai"
)]
pub struct Args {
    #[structopt(
        short = "b",
        long = "bat",
        help = "`bat` (the Rust alternative to the `cat` command) a file"
    )]
    pub bat: Option<String>,

    #[structopt(help = "Explore this directory")]
    pub directory: Option<String>,

    #[structopt(long = "disrespect", help = "Disrespect all ignore rules")]
    pub disrespect: bool,

    #[structopt(
        long = "export",
        help = "Export the tree to a file instead of displaying"
    )]
    pub export: Option<String>,

    //#[structopt(short = "g", long = "git", help = "Display Git markers for files")]
    #[structopt(subcommand, help = "Run Git commands")]
    pub git: Option<Git>,

    #[structopt(long = "hidden", help = "Display hidden files")]
    pub hidden: bool,

    #[structopt(
        short = "i",
        long = "interactive",
        help = "Initialize an interactive file/directory explorer"
    )]
    pub interactive: bool,

    #[structopt(
        short = "m",
        long = "metadata",
        help = "Show item metadata such as file permissions, owner, group, file size, and last modified time"
    )]
    pub metadata: bool,

    #[structopt(
        short = "n",
        long = "numbered",
        help = "Show directory contents with numbers"
    )]
    pub numbers: bool,

    #[structopt(
        short = "o",
        long = "open",
        help = "Open a file based on its index within the tree\nThis may be used after running `nomad` in numbered mode (`-n`)"
    )]
    pub open: Option<String>,

    #[structopt(
        short = "s",
        long = "stats",
        help = "Display directory traversal statistics after the tree is displayed"
    )]
    pub statistics: bool,
}

#[derive(Debug, PartialEq, StructOpt)]
pub enum Git {
    /// Run commonly used Git commands.
    Git(GitOptions),
}

/// This enum provides some commonly used Git options.
#[derive(Debug, PartialEq, StructOpt)]
pub enum GitOptions {
    /// Equivalent to the `git add` command.
    Add { file_numbers: Vec<i32> },
    /// Equivalent to the `git commit` command.
    /// Optionally include a message after the command, ie. `git commit "YOUR MESSAGE HERE"`.
    /// The default commit message is "Updating" if no message is included.
    Commit { message: Option<String> },
    /// Equivalent to the `git diff` command.
    Diff { file_number: i32 },
    /// Equivalent to the `git status` command. Only display changed/unstaged files in the tree.
    Status,
}

/// Return the `Args` struct.
pub fn get_args() -> Args {
    Args::from_args()
}

#[cfg(test)]
mod test_cli {
    use super::*;

    use assert_cmd::Command;

    #[test]
    fn test_invalid_arg() {
        //Command::cargo_bin("nd")
        //.unwrap()
        //.arg("-q")
        //.assert()
        //.failure();
    }
}
