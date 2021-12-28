//! Defining command-line interface flags.

use structopt::StructOpt;

/// This struct contains all flags that are used in this program.
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(name = "oak", about = "The `tree` command, but better.")]
pub struct Args {
    #[structopt(help = "Display a tree for this directory")]
    pub directory: Option<String>,
}

/// Return the `Args` struct.
pub fn get_args() -> Args {
    Args::from_args()
}
