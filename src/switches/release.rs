//! Executing release commands.

use crate::{
    cli::releases::ReleaseOptions,
    releases::build_release_list,
    utils::{
        paint::paint_error,
        table::{TableView, TabledItems},
    },
};

pub fn run_releases(release_option: &ReleaseOptions) {
    match release_option {
        ReleaseOptions::All => match build_release_list() {
            Ok(releases) => TabledItems::new(
                releases,
                vec![
                    "Name".into(),
                    "Version".into(),
                    "Release Date".into(),
                    "Description".into(),
                    "Assets".into(),
                ],
                180,
                None,
            )
            .display_table(),
            Err(error) => paint_error(error),
        },
        ReleaseOptions::Info { release_version } => match build_release_list() {
            Ok(releases) => TabledItems::new(
                releases,
                vec![
                    "Name".into(),
                    "Version".into(),
                    "Release Date".into(),
                    "Description".into(),
                    "Assets".into(),
                ],
                180,
                release_version.to_owned(),
            )
            .display_table(),
            Err(error) => paint_error(error),
        },
    }
}
