//! Providing CLI options for `nomad` releases.

use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
pub enum ReleaseOptions {
    /// List all releases.
    All,
    /// Display information for a release version. Optionally search for a release version.
    Info { release_version: Option<String> },
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct UpgradeOptions {
    /// Check if there is an upgrade available. Does not actually upgrade nomad.
    #[structopt(
        long,
        help = "Check if there is an upgrade available. Does not actually upgrade nomad"
    )]
    pub check: bool,
}
