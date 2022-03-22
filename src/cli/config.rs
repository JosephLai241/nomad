//! Providing configuration read/write CLI options.

use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
pub enum ConfigOptions {
    /// Edit the configuration file.
    Edit,
    /// Preview the configuration settings in a tree.
    Preview,
}
