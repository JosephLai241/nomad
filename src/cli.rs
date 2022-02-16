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
    #[structopt(help = "Display a tree for this directory")]
    pub directory: Option<String>,

    #[structopt(long = "dirs", help = "Only display directories")]
    pub dirs: bool,

    #[structopt(long = "disrespect", help = "Disrespect all ignore rules")]
    pub disrespect: bool,

    #[structopt(long = "export", help = "Export the tree to a file")]
    pub export: Option<String>,

    #[structopt(
        short = "l",
        long = "label-directories",
        help = "Label directories with characters"
    )]
    pub label_directories: bool,

    #[structopt(long = "hidden", help = "Display hidden files")]
    pub hidden: bool,

    #[structopt(
        short = "m",
        long = "metadata",
        help = "Show item metadata such as file permissions, owner, group, file size, and last modified time"
    )]
    pub metadata: bool,

    #[structopt(long = "mute-git", help = "Do not display Git status markers")]
    pub mute_git: bool,

    #[structopt(long = "mute-icons", help = "Do not display icons")]
    pub mute_icons: bool,

    #[structopt(
        short = "n",
        long = "numbered",
        help = "Show directory contents with numbers"
    )]
    pub numbers: bool,

    #[structopt(
        short = "p",
        long = "pattern",
        help = "Only display files matching this pattern. Supports regex expressions"
    )]
    pub pattern: Option<String>,

    #[structopt(
        long = "plain",
        help = "Mute icons, Git markers, and colors to display a plain tree"
    )]
    pub plain: bool,

    #[structopt(
        short = "s",
        long = "stats",
        help = "Display directory traversal statistics after the tree is displayed"
    )]
    pub statistics: bool,

    #[structopt(subcommand)]
    pub sub_commands: Option<SubCommands>,
}

#[derive(Debug, PartialEq, StructOpt)]
pub enum SubCommands {
    ///`bat` (the Rust alternative to the `cat` command) a file.
    /// This may be used after running nomad in numbered mode.
    Bat { file_number: i32 },
    /// Edit a file with your default $EDITOR or with Neovim, Vim, Vi, or Nano.
    /// This may be used after running nomad in numbered mode.
    Edit { file_number: i32 },
    /// Filter directory items by filetype.
    Filetype(FileTypeOptions),
    /// Run commonly used Git commands.
    Git(GitOptions),
}

/// This enum provides some commonly used Git options.
#[derive(Debug, PartialEq, StructOpt)]
pub enum GitOptions {
    /// The `git add` command.
    /// This may be used after running nomad in numbered mode or with labeled directories.
    /// Enter a single digit or a list of digits delimited by a space.
    Add { file_numbers: Vec<i32> },
    /// The `git commit` command.
    /// Optionally include a message after the command, ie. `git commit "YOUR MESSAGE HERE"`
    /// The default commit message is "Updating" if no message is included.
    Commit { message: Option<String> },
    /// The `git diff` command.
    /// This may be used after running nomad in numbered mode or with labeled directories.
    Diff { file_number: i32 },
    /// The `git status` command. Only display changed/unstaged files in the tree.
    Status,
}

/// This enum provides pattern matching options.
#[derive(Debug, PartialEq, StructOpt)]
pub enum FileTypeOptions {
    /// Only display files matching this filetype. Enter a single filetype or a
    /// list of filetypes delimited by a space.
    /// ie. `nd filetype match rust py go vim`.
    Match { filetypes: Vec<String> },
    /// Do not display files that match this filetype. Enter a single filetype or
    /// a list of filetypes delimited by a space.
    /// ie. `nd filetype negate c cpp java r`.
    Negate { filetypes: Vec<String> },
    /// List the current set of filetype definitions. Optionally search for a filetype.
    /// ie. `nd filetype options rust`.
    Options { filetype: Option<String> },
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
