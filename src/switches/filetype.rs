//! Executing filetype subcommands.

use ignore::types::TypesBuilder;

use crate::{
    cli::{filetype::FileTypeOptions, Args},
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
    args: &Args,
    filetype_option: &FileTypeOptions,
    nomad_style: &NomadStyle,
    target_directory: &str,
) {
    let mut type_matcher = TypesBuilder::new();
    type_matcher.add_defaults();

    match filetype_option {
        FileTypeOptions::Match { filetypes, globs } => {
            match build_types(filetypes, globs, type_matcher, TypeOption::Match) {
                Ok(types) => match build_walker(&args, &target_directory, Some(types)) {
                    Ok(mut walker) => {
                        match walk_directory(
                            &args,
                            NomadMode::Normal,
                            nomad_style,
                            &target_directory,
                            &mut walker,
                        ) {
                            Ok((tree, config, _)) => {
                                if let Some(export) = &args.export {
                                    if let Err(error) = export_tree(
                                        config,
                                        ExportMode::Filetype(filetypes, globs),
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
                },
                Err(error) => paint_error(error),
            }
        }
        FileTypeOptions::Negate { filetypes, globs } => {
            match build_types(filetypes, globs, type_matcher, TypeOption::Negate) {
                Ok(types) => match build_walker(&args, &target_directory, Some(types)) {
                    Ok(mut walker) => {
                        match walk_directory(
                            &args,
                            NomadMode::Normal,
                            nomad_style,
                            &target_directory,
                            &mut walker,
                        ) {
                            Ok((tree, config, _)) => {
                                if let Some(export) = &args.export {
                                    if let Err(error) = export_tree(
                                        config,
                                        ExportMode::Filetype(filetypes, globs),
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
                },
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
