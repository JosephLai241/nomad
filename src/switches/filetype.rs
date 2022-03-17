//! Executing filetype subcommands.

use ignore::types::TypesBuilder;

use crate::{
    cli::filetype::FileTypeOptions,
    style::models::NomadStyle,
    traverse::{
        modes::NomadMode,
        utils::{build_types, build_walker, TypeOption},
        walk_directory,
    },
    utils::{
        export::{export_tree, ExportMode},
        paint::paint_error,
        table::{TableView, TabledItems},
    },
};

/// `match` the filetype subcommand and execute it.
pub fn run_filetypes(
    filetype_option: &FileTypeOptions,
    nomad_style: &NomadStyle,
    target_directory: &str,
) {
    let mut type_matcher = TypesBuilder::new();
    type_matcher.add_defaults();

    match filetype_option {
        FileTypeOptions::Match(match_options) => {
            match build_types(
                &match_options.filetypes,
                &match_options.globs,
                type_matcher,
                TypeOption::Match,
            ) {
                Ok(types) => {
                    match build_walker(&match_options.general, &target_directory, Some(types)) {
                        Ok(mut walker) => {
                            match walk_directory(
                                &match_options.general,
                                NomadMode::Normal,
                                nomad_style,
                                &target_directory,
                                &mut walker,
                            ) {
                                Ok((tree, config, _)) => {
                                    if let Some(export) = &match_options.general.export {
                                        if let Err(error) = export_tree(
                                            config,
                                            ExportMode::Filetype(
                                                &match_options.filetypes,
                                                &match_options.globs,
                                            ),
                                            &export,
                                            tree,
                                        ) {
                                            paint_error(error);
                                        }
                                    }
                                }
                                Err(error) => paint_error(error),
                            }
                        }
                        Err(error) => paint_error(error),
                    }
                }
                Err(error) => paint_error(error),
            }
        }
        FileTypeOptions::Negate(negate_options) => {
            match build_types(
                &negate_options.filetypes,
                &negate_options.globs,
                type_matcher,
                TypeOption::Negate,
            ) {
                Ok(types) => {
                    match build_walker(&negate_options.general, &target_directory, Some(types)) {
                        Ok(mut walker) => {
                            match walk_directory(
                                &negate_options.general,
                                NomadMode::Normal,
                                nomad_style,
                                &target_directory,
                                &mut walker,
                            ) {
                                Ok((tree, config, _)) => {
                                    if let Some(export) = &negate_options.general.export {
                                        if let Err(error) = export_tree(
                                            config,
                                            ExportMode::Filetype(
                                                &negate_options.filetypes,
                                                &negate_options.globs,
                                            ),
                                            &export,
                                            tree,
                                        ) {
                                            paint_error(error);
                                        }
                                    }
                                }
                                Err(error) => paint_error(error),
                            }
                        }
                        Err(error) => paint_error(error),
                    }
                }
                Err(error) => paint_error(error),
            }
        }
        FileTypeOptions::Options { filetype } => TabledItems::new(
            type_matcher.definitions(),
            vec!["Name".into(), "Globs".into()],
            120,
            filetype.to_owned(),
        )
        .display_table(),
    }
}
