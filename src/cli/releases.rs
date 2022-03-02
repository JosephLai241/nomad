//! Providing CLI options for `nomad` releases.

use structopt::StructOpt;

/// This enum provides interactions with releases.
#[derive(Debug, PartialEq, StructOpt)]
pub enum ReleaseOptions {
    /// List all releases.
    All,
    /// Display information for a release version. Optionally search for a release version.
    Info { release_version: Option<String> },
}
