//! Providing pattern matching CLI options.

use structopt::StructOpt;

/// This enum provides pattern matching options.
#[derive(Debug, PartialEq, StructOpt)]
pub enum FileTypeOptions {
    /// Only display files matching the specified filetypes and/or globs.
    Match {
        #[structopt(
            short,
            long,
            help = "Enter a single filetype or a list of filetypes delimited by a space. ie. `nd filetype match -f rust py go vim`"
        )]
        filetypes: Vec<String>,
        #[structopt(
            short,
            long,
            help = "Enter a single glob or a list of globs delimited by a space. ie. `nd filetype match -g *.something *.anotherthing`. You may have to put quotes around globs that include '*'"
        )]
        globs: Vec<String>,
    },
    /// Do not display files that match the specified filetypes and/or globs.
    Negate {
        #[structopt(
            short,
            long,
            help = "Enter a single filetype or a list of filetypes delimited by a space. ie. `nd filetype match -f rust py go vim`"
        )]
        filetypes: Vec<String>,
        #[structopt(
            short,
            long,
            help = "Enter a single glob or a list of globs delimited by a space. ie. `nd filetype match -g *.something *.anotherthing`. You may have to put quotes around globs that include '*'"
        )]
        globs: Vec<String>,
    },
    /// List the current set of filetype definitions. Optionally search for a filetype.
    /// ie. `nd filetype options rust`.
    Options { filetype: Option<String> },
}
