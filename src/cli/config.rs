//! Providing configuration read/write CLI options.

use structopt::StructOpt;

/// This enum provides options to read or edit the configuration file.
#[derive(Debug, PartialEq, StructOpt)]
pub enum ConfigOptions {
    /// Display the configuration settings in tables.
    Display,
    /// Edit the configuration file.
    Edit,
}
