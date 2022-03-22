//! Apply colors to a directory's contents.

use crate::errors::NomadError;

use ansi_term::Colour;

/// Format and display a `NomadError`.
pub fn paint_error(error: NomadError) {
    println!("\n{}\n", Colour::Red.bold().paint(error.to_string()));
}
