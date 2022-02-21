//! Helpers for self-updating `nomad`.

use crate::errors::NomadError;

use ansi_term::Colour;
use anyhow::Result;
use indicatif::{ProgressFinish, ProgressStyle};
use self_update::{
    backends::github::{ReleaseList, Update},
    cargo_crate_version,
    update::Release,
};

use std::borrow::Cow;

/// Check for updates. An update is only displayed if there is a working internet
/// connection, if checking the GitHub repository is successful, and if there is
/// an update available.
pub fn check_for_update() -> Result<(), NomadError> {
    let releases = ReleaseList::configure()
        .repo_name("nomad")
        .repo_owner("JosephLai241")
        .build()?
        .fetch();

    if let Ok(mut releases) = releases {
        let latest_release = releases.pop();

        if let Some(latest) = latest_release {
            if latest.version != cargo_crate_version!().to_string() {
                println!(
                    "\nNew release available! {} ==> {}\nRun `nd upgrade` to upgrade to the newest version.\n",
                    Colour::Red.bold().paint(cargo_crate_version!()),
                    Colour::Green.bold().paint(latest.version)
                );
            }
        }
    }

    Ok(())
}

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
        .set_progress_style(
            ProgressStyle::default_bar().on_finish(ProgressFinish::WithMessage(Cow::from("💯"))),
        )
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
