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

/// This enum provides interactions with upgrades and is related to releases.
#[derive(Debug, PartialEq, StructOpt)]
pub struct UpgradeOptions {
    /// Check if there is an upgrade available. Does not actually upgrade nomad.
    #[structopt(
        long,
        help = "Check if there is an upgrade available. Does not actually upgrade nomad"
    )]
    pub check: bool,
}
