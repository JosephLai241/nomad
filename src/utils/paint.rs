//! Apply colors to a directory's contents.

use crate::errors::NomadError;

use ansi_term::Colour;
use lazy_static::lazy_static;

lazy_static! {
    /// THE BANNER 🥴.
    static ref BANNER: &'static str = r#"
   ________  ________  ________  ________   _______
  ╱    ╱   ╲╱        ╲╱        ╲╱        ╲_╱       ╲
 ╱         ╱         ╱         ╱         ╱         ╱
╱         ╱         ╱         ╱         ╱         ╱
╲__╱_____╱╲________╱╲__╱__╱__╱╲___╱____╱╲________╱
"#;
}

/// Format and display a `NomadError`.
pub fn paint_error(error: NomadError) {
    println!("\n{}\n", Colour::Red.bold().paint(error.to_string()));
}

/// Display the ASCII art for `nomad`.
pub fn show_banner() {
    println!(
        "{}",
        Colour::Fixed(172).blink().bold().paint(BANNER.to_string())
    );
}
