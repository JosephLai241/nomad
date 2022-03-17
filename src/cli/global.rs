//! Providing arguments that are used throughout `nomad`.

use structopt::StructOpt;

/// This struct contains general use flags.
#[derive(Debug, PartialEq, StructOpt)]
pub struct GlobalArgs {
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
    pub modifiers: ModifierArgs,

    #[structopt(flatten)]
    pub regex: RegexArgs,

    #[structopt(flatten)]
    pub style: StyleArgs,

    #[structopt(
        short = "s",
        long = "stats",
        help = "Display traversal statistics after the tree is displayed"
    )]
    pub statistics: bool,
}

/// This struct contains flags that label tree items.
#[derive(Debug, PartialEq, StructOpt)]
pub struct LabelArgs {
    #[structopt(
        short = "L",
        long = "all-labels",
        help = "Label both files and directories. Alias for `-n -l`"
    )]
    pub all_labels: bool,

    #[structopt(
        short = "l",
        long = "label-directories",
        help = "Label directories with characters"
    )]
    pub label_directories: bool,

    #[structopt(
        short = "n",
        long = "numbered",
        help = "Label directory items with numbers"
    )]
    pub numbers: bool,
}

/// This struct contains flags that implement `Tokei` or extract file metadata.
#[derive(Debug, PartialEq, StructOpt)]
pub struct MetaArgs {
    #[structopt(
        long = "loc",
        help = "Display code statistics (lines of code, blanks, and comments) for each item"
    )]
    pub loc: bool,

    #[structopt(
        short = "m",
        long = "metadata",
        help = "Show item metadata such as file permissions, owner, group, file size, and last modified time"
    )]
    pub metadata: bool,

    #[structopt(
        long = "no-tree",
        help = "Only display `tokei` (lines of code counter) statistics. This only applies if `--loc` is provided"
    )]
    pub no_tree: bool,

    #[structopt(
        long = "summary",
        help = "Display `tokei` (lines of code counter) statistics. This only applies if `--loc` is provided"
    )]
    pub summarize: bool,
}

/// This struct contains flags that may alter the items that are displayed in the tree.
#[derive(Debug, PartialEq, StructOpt)]
pub struct ModifierArgs {
    #[structopt(long = "dirs", help = "Only display directories")]
    pub dirs: bool,

    #[structopt(long = "disrespect", help = "Disrespect all ignore rules")]
    pub disrespect: bool,

    #[structopt(long = "hidden", help = "Display hidden files")]
    pub hidden: bool,

    #[structopt(long = "max-depth", help = "Set the maximum depth to recurse")]
    pub max_depth: Option<usize>,

    #[structopt(
        long = "max-filesize",
        help = "Set the maximum filesize (in bytes) to include in the tree"
    )]
    pub max_filesize: Option<u64>,
}

/// This struct contains flags that implement regex.
#[derive(Debug, PartialEq, StructOpt)]
pub struct RegexArgs {
    #[structopt(
        short = "p",
        long = "pattern",
        help = "Only display items matching this pattern. Supports regex expressions"
    )]
    pub pattern: Option<String>,
}

/// This struct contains flags that alter how the tree looks.
#[derive(Debug, PartialEq, StructOpt)]
pub struct StyleArgs {
    #[structopt(long = "no-colors", help = "Do not display any colors")]
    pub no_colors: bool,

    #[structopt(long = "no-git", help = "Do not display Git status markers")]
    pub no_git: bool,

    #[structopt(long = "no-icons", help = "Do not display icons")]
    pub no_icons: bool,

    #[structopt(
        long = "plain",
        help = "Mute icons, Git markers, and colors to display a plain tree"
    )]
    pub plain: bool,
}
