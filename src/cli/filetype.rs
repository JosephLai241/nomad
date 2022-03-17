//! Providing pattern matching CLI options.

use structopt::StructOpt;

use super::global::GlobalArgs;

/// This enum provides pattern matching options.
#[derive(Debug, PartialEq, StructOpt)]
pub enum FileTypeOptions {
    /// Only display files matching the specified filetypes and/or globs.
    Match(MatchOptions),
    /// Do not display files that match the specified filetypes and/or globs.
    Negate(NegateOptions),
    /// List the current set of filetype definitions. Optionally search for a filetype.
    /// ie. `nd filetype options rust`.
    Options { filetype: Option<String> },
}

/// This struct provides options for filetype matching.
#[derive(Debug, PartialEq, StructOpt)]
pub struct MatchOptions {
    #[structopt(
        short,
        long,
        help = "Enter a single filetype or a list of filetypes delimited by a space. ie. `nd filetype match -f rust py go vim`"
    )]
    pub filetypes: Vec<String>,

    #[structopt(flatten)]
    pub general: GlobalArgs,

    #[structopt(
        short,
        long,
        help = "Enter a single glob or a list of globs delimited by a space. ie. `nd filetype match -g *.something *.anotherthing`. You may have to put quotes around globs that include '*'"
    )]
    pub globs: Vec<String>,
}

/// This struct provides options for filetype negating.
#[derive(Debug, PartialEq, StructOpt)]
pub struct NegateOptions {
    #[structopt(
        short,
        long,
        help = "Enter a single filetype or a list of filetypes delimited by a space. ie. `nd filetype match -f rust py go vim`"
    )]
    pub filetypes: Vec<String>,

    #[structopt(flatten)]
    pub general: GlobalArgs,

    #[structopt(
        short,
        long,
        help = "Enter a single glob or a list of globs delimited by a space. ie. `nd filetype match -g *.something *.anotherthing`. You may have to put quotes around globs that include '*'"
    )]
    pub globs: Vec<String>,
}
