//! Providing configuration read/write CLI options.

use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
pub enum ConfigOptions {
    /// Display the configuration settings in tables.
    Display,
    /// Edit the configuration file.
    Edit,
}
