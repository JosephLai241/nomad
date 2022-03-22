//! Providing Git CLI options.

use structopt::StructOpt;

use super::global::{LabelArgs, MetaArgs, RegexArgs, StyleArgs};

#[derive(Debug, PartialEq, StructOpt)]
pub enum GitOptions {
    /// The `git add` command.
    /// This may be used after running nomad in a labeled mode.
    Add(AddOptions),
    /// The `git blame` command.
    /// This may be used after running nomad in a labeled mode.
    /// You can only call `git blame` on a single file.
    Blame(BlameOptions),
    /// The `git branch` command. Displays branches in tree form by default (this behavior may be
    /// disabled).
    Branch(BranchOptions),
    /// The `git commit` command.
    /// Optionally include a message after the command, ie. `git commit "YOUR MESSAGE HERE"`
    /// The default commit message is "Updating" if no message is included.
    Commit { message: Option<String> },
    /// The `git diff` command.
    /// This may be used after running nomad in a labeled mode.
    Diff { item_labels: Vec<String> },
    /// The `git restore` command. This may be used after running nomad in a labeled mode.
    Restore(RestoreOptions),
    /// The `git status` command. Only display changed/unstaged files in the tree.
    Status(StatusOptions),
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct AddOptions {
    #[structopt(help = "The item labels to add")]
    pub item_labels: Vec<String>,

    #[structopt(
        short = "A",
        long,
        help = "Add changes from all tracked and untracked files"
    )]
    pub all: bool,
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct BlameOptions {
    #[structopt(
        long,
        help = "Display emails for each blame line instead of timestamps"
    )]
    pub emails: bool,

    #[structopt(help = "Display a blame for this file")]
    pub file_number: String,

    #[structopt(
        short,
        long,
        help = "Restrict a range of lines to display in the blame"
    )]
    pub lines: Vec<usize>,
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct BranchOptions {
    #[structopt(
        long = "export",
        help = "Export the tree to a file. Optionally include a target filename"
    )]
    pub export: Option<Option<String>>,

    #[structopt(short, long, help = "Display branches in a normal list")]
    pub flat: bool,

    #[structopt(short = "n", long = "numbered", help = "Label branches with numbers")]
    pub numbers: bool,

    #[structopt(
        short = "p",
        long = "pattern",
        help = "Only display branches matching this pattern. Supports regex expressions"
    )]
    pub pattern: Option<String>,

    #[structopt(short, long, help = "Display the total number of branches")]
    pub statistics: bool,

    #[structopt(long = "no-icons", help = "Do not display icons")]
    pub no_icons: bool,
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct RestoreOptions {
    #[structopt(
        help = "Restore these items to its clean Git state. Restores in the working tree by default"
    )]
    pub item_labels: Vec<String>,
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct StatusOptions {
    #[structopt(
        long = "export",
        help = "Export the tree to a file. Optionally include a target filename"
    )]
    pub export: Option<Option<String>>,

    #[structopt(flatten)]
    pub labels: LabelArgs,

    #[structopt(flatten)]
    pub meta: MetaArgs,

    #[structopt(flatten)]
    pub regex: RegexArgs,

    #[structopt(
        short = "s",
        long = "stats",
        help = "Display traversal statistics after the tree is displayed"
    )]
    pub statistics: bool,

    #[structopt(flatten)]
    pub style: StyleArgs,
}
