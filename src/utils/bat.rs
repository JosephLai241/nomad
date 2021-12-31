//! Run `bat`.

use std::{
    io::{Error, ErrorKind},
    path::Path,
};

use bat::{PagingMode, PrettyPrinter, WrappingMode};

/// Create a new `PrettyPrinter`, then run it against the file.
pub fn run_bat(file: String) -> Result<(), Error> {
    PrettyPrinter::new()
        .grid(true)
        .header(true)
        .input_file(Path::new(&file))
        .line_numbers(true)
        .paging_mode(PagingMode::QuitIfOneScreen)
        .true_color(true)
        .vcs_modification_markers(true)
        .wrapping_mode(WrappingMode::Character)
        .print()
        .map_or_else(
            |error| Err(Error::new(ErrorKind::Other, error.kind().to_string())),
            |_| Ok(()),
        )
}
