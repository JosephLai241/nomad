//! Helpers for self-updating `nomad`.

use ansi_term::Colour;
use anyhow::Result;
use self_update::{
    backends::github::{ReleaseList, Update},
    cargo_crate_version,
    update::Release,
};

use crate::errors::NomadError;

/// Return a list of `Release` objects containing release information.
pub fn build_release_list() -> Result<Vec<Release>, NomadError> {
    Ok(ReleaseList::configure()
        .repo_name("nomad")
        .repo_owner("JosephLai241")
        .build()?
        .fetch()?)
}

/// Update `nomad`.
pub fn update_self() -> Result<(), NomadError> {
    let current_version = cargo_crate_version!();

    let update_status = Update::configure()
        .bin_name("nd")
        .current_version(cargo_crate_version!())
        .repo_name("nomad")
        .repo_owner("JosephLai241")
        .show_download_progress(true)
        .build()?
        .update()?;

    if update_status.updated() {
        println!(
            "\nSuccessfully updated nomad from {} to {}!\n",
            Colour::Fixed(172)
                .bold()
                .paint(format!("{current_version}")),
            Colour::Green
                .bold()
                .paint(format!("{}", update_status.version()))
        );
    } else {
        println!(
            "\n{}\n",
            Colour::Fixed(172)
                .bold()
                .paint("Already at the newest version.")
        );
    }

    Ok(())
}
