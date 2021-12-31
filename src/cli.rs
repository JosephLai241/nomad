//! Defining command-line interface flags.

use structopt::StructOpt;

/// This struct contains all flags that are used in this program.
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(name = "nomad", about = "The `tree` command, but better.")]
pub struct Args {
    #[structopt(help = "Explore this directory.")]
    pub directory: Option<String>,

    #[structopt(long = "disrespect", help = "Disrespect ignore rules.")]
    pub disrespect: bool,

    #[structopt(long = "hidden", help = "Do not display hidden files.")]
    pub hidden: bool,

    #[structopt(
        short = "i",
        long = "interactive",
        help = "Initialize an interactive file/directory explorer"
    )]
    pub interactive: bool,

    #[structopt(
        short = "n",
        long = "numbered",
        help = "Show directory contents with numbers"
    )]
    pub numbers: bool,

    #[structopt(
        short = "o",
        long = "open",
        help = "Open a file based on its index within the tree"
    )]
    pub open: Option<String>,

    #[structopt(
        short = "s",
        long = "stats",
        help = "Display directory traversal statistics after the tree is displayed"
    )]
    pub statistics: bool,
}

/// Return the `Args` struct.
pub fn get_args() -> Args {
    Args::from_args()
}
