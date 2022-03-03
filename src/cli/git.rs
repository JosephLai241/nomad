//! Providing Git CLI options.

use structopt::StructOpt;

/// This enum provides some commonly used Git options.
#[derive(Debug, PartialEq, StructOpt)]
pub enum GitOptions {
    /// The `git add` command.
    /// This may be used after running nomad in numbered mode or with labeled directories.
    /// Enter a single or a list of numbers/labels delimited by a space.
    Add { item_labels: Vec<String> },
    /// The `git blame` command.
    /// This may be used after running monad in numbered mode or with labeled directories.
    /// You can only call `git blame` on a single file.
    Blame(BlameOptions),
    /// The `git branch` command.
    Branch,
    /// The `git commit` command.
    /// Optionally include a message after the command, ie. `git commit "YOUR MESSAGE HERE"`
    /// The default commit message is "Updating" if no message is included.
    Commit { message: Option<String> },
    /// The `git diff` command.
    /// This may be used after running nomad in numbered mode or with labeled directories.
    Diff { item_labels: Vec<String> },
    /// The `git restore` command.
    Restore(RestoreOptions),
    /// The `git status` command. Only display changed/unstaged files in the tree.
    Status,
}

/// This struct provides options for the `git blame` command.
#[derive(Debug, PartialEq, StructOpt)]
pub struct BlameOptions {
    #[structopt(help = "Display a blame for this file")]
    pub file_number: String,
    #[structopt(
        short,
        long,
        help = "Restrict a range of lines to display in the blame"
    )]
    pub lines: Vec<usize>,
}

/// This struct provides options for the `git restore` command.
#[derive(Debug, PartialEq, StructOpt)]
pub struct RestoreOptions {
    #[structopt(
        help = "Restore these items to its clean Git state. Restores in the working tree by default"
    )]
    pub item_labels: Vec<String>,
    #[structopt(short, long, help = "Restore these items in the index")]
    pub staged: bool,
}