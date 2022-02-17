//! Helpers for self-updating `nomad`.

use anyhow::Result;
use self_update::{backends::github::ReleaseList, update::Release};

use crate::errors::NomadError;

/// Return a list of `Release` objects containing release information.
pub fn build_release_list() -> Result<Vec<Release>, NomadError> {
    Ok(ReleaseList::configure()
        .repo_name("nomad")
        .repo_owner("JosephLai241")
        .build()?
        .fetch()?)
}
