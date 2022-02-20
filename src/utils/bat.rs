//! Run `bat`.

use crate::errors::NomadError;

use anyhow::Result;
use bat::{PagingMode, PrettyPrinter, WrappingMode};

use std::path::Path;

/// Create a new `PrettyPrinter`, then run it against the file.
pub fn run_bat(file: String) -> Result<(), NomadError> {
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
        .map_or_else(|error| Err(NomadError::BatError(error)), |_| Ok(()))
}
