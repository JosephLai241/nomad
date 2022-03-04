//! Providing configuration read/write CLI options.

use structopt::StructOpt;

/// This enum provides options to read or edit the configuration file.
#[derive(Debug, PartialEq, StructOpt)]
pub enum ConfigOptions {
    /// Edit the configuration file.
    Edit,
    /// View the configuration file.
    View,
}
