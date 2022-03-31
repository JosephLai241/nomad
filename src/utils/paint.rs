//! Apply colors to a directory's contents.

use crate::errors::NomadError;

use ansi_term::Colour;
use lazy_static::lazy_static;
use syntect::highlighting::Color;

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

/// Converts an `ansi_term::Colour` to a `syntect::highlighting::Color`
pub fn convert_ansi_to_syntect(color: u8) -> Color {
    match color {
        000 => Color {
            r: 000,
            g: 000,
            b: 000,
            a: 001,
        },
        001 => Color {
            r: 080,
            g: 065,
            b: 047,
            a: 001,
        },
        004 => Color {
            r: 020,
            g: 020,
            b: 108,
            a: 001,
        },
        005 => Color {
            r: 092,
            g: 035,
            b: 092,
            a: 001,
        },
        017 => Color {
            r: 008,
            g: 008,
            b: 089,
            a: 001,
        },
        018 => Color {
            r: 034,
            g: 034,
            b: 099,
            a: 000,
        },
        019 => Color {
            r: 036,
            g: 036,
            b: 138,
            a: 000,
        },
        020 => Color {
            r: 054,
            g: 054,
            b: 161,
            a: 000,
        },
        052 => Color {
            r: 095,
            g: 000,
            b: 000,
            a: 000,
        },
        053 => Color {
            r: 095,
            g: 000,
            b: 095,
            a: 001,
        },
        054 => Color {
            r: 095,
            g: 000,
            b: 135,
            a: 001,
        },
        055 => Color {
            r: 095,
            g: 000,
            b: 175,
            a: 001,
        },
        056 => Color {
            r: 102,
            g: 052,
            b: 162,
            a: 001,
        },
        057 => Color {
            r: 113,
            g: 071,
            b: 184,
            a: 001,
        },
        058 => Color {
            r: 095,
            g: 070,
            b: 027,
            a: 001,
        },
        059 => Color {
            r: 095,
            g: 095,
            b: 095,
            a: 001,
        },
        060 => Color {
            r: 095,
            g: 095,
            b: 135,
            a: 001,
        },
        061 => Color {
            r: 095,
            g: 095,
            b: 175,
            a: 001,
        },
        062 => Color {
            r: 095,
            g: 095,
            b: 215,
            a: 001,
        },
        088 => Color {
            r: 135,
            g: 000,
            b: 000,
            a: 001,
        },
        089 => Color {
            r: 135,
            g: 000,
            b: 095,
            a: 001,
        },
        090 => Color {
            r: 135,
            g: 000,
            b: 135,
            a: 001,
        },
        091 => Color {
            r: 135,
            g: 000,
            b: 175,
            a: 001,
        },
        092 => Color {
            r: 135,
            g: 000,
            b: 215,
            a: 001,
        },
        124 => Color {
            r: 175,
            g: 000,
            b: 000,
            a: 001,
        },
        125 => Color {
            r: 175,
            g: 000,
            b: 095,
            a: 000,
        },
        126 => Color {
            r: 175,
            g: 000,
            b: 135,
            a: 001,
        },
        127 => Color {
            r: 175,
            g: 000,
            b: 175,
            a: 000,
        },
        128 => Color {
            r: 175,
            g: 000,
            b: 215,
            a: 001,
        },
        129 => Color {
            r: 175,
            g: 000,
            b: 255,
            a: 001,
        },
        130 => Color {
            r: 175,
            g: 095,
            b: 000,
            a: 001,
        },
        131 => Color {
            r: 175,
            g: 095,
            b: 095,
            a: 001,
        },
        132 => Color {
            r: 175,
            g: 095,
            b: 153,
            a: 001,
        },
        133 => Color {
            r: 175,
            g: 095,
            b: 175,
            a: 001,
        },
        134 => Color {
            r: 175,
            g: 095,
            b: 215,
            a: 001,
        },
        136 => Color {
            r: 175,
            g: 135,
            b: 000,
            a: 001,
        },
        137 => Color {
            r: 175,
            g: 135,
            b: 095,
            a: 001,
        },
        138 => Color {
            r: 175,
            g: 135,
            b: 135,
            a: 001,
        },
        139 => Color {
            r: 175,
            g: 135,
            b: 175,
            a: 001,
        },
        160 => Color {
            r: 215,
            g: 000,
            b: 000,
            a: 001,
        },
        161 => Color {
            r: 215,
            g: 000,
            b: 095,
            a: 001,
        },
        162 => Color {
            r: 216,
            g: 000,
            b: 135,
            a: 001,
        },
        163 => Color {
            r: 216,
            g: 000,
            b: 175,
            a: 001,
        },
        165 => Color {
            r: 215,
            g: 000,
            b: 255,
            a: 001,
        },
        166 => Color {
            r: 215,
            g: 095,
            b: 000,
            a: 001,
        },
        167 => Color {
            r: 215,
            g: 095,
            b: 095,
            a: 001,
        },
        196 => Color {
            r: 255,
            g: 000,
            b: 000,
            a: 001,
        },
        197 => Color {
            r: 255,
            g: 000,
            b: 095,
            a: 001,
        },
        198 => Color {
            r: 255,
            g: 000,
            b: 135,
            a: 001,
        },
        199 => Color {
            r: 255,
            g: 000,
            b: 175,
            a: 001,
        },
        200 => Color {
            r: 255,
            g: 000,
            b: 223,
            a: 001,
        },
        201 => Color {
            r: 255,
            g: 000,
            b: 223,
            a: 001,
        },
        202 => Color {
            r: 255,
            g: 095,
            b: 000,
            a: 001,
        },
        203 => Color {
            r: 255,
            g: 095,
            b: 095,
            a: 001,
        },
        204 => Color {
            r: 255,
            g: 095,
            b: 135,
            a: 001,
        },
        208 => Color {
            r: 255,
            g: 135,
            b: 000,
            a: 001,
        },
        218 => Color {
            r: 255,
            g: 175,
            b: 223,
            a: 001,
        },
        236 => Color {
            r: 048,
            g: 048,
            b: 048,
            a: 001,
        },
        237 => Color {
            r: 058,
            g: 058,
            b: 058,
            a: 001,
        },
        238 => Color {
            r: 068,
            g: 068,
            b: 068,
            a: 001,
        },
        239 => Color {
            r: 078,
            g: 078,
            b: 078,
            a: 001,
        },
        240 => Color {
            r: 088,
            g: 088,
            b: 088,
            a: 001,
        },
        241 => Color {
            r: 098,
            g: 098,
            b: 098,
            a: 001,
        },
        242 => Color {
            r: 108,
            g: 108,
            b: 108,
            a: 001,
        },
        243 => Color {
            r: 118,
            g: 118,
            b: 118,
            a: 001,
        },
        244 => Color {
            r: 128,
            g: 128,
            b: 128,
            a: 001,
        },
        245 => Color {
            r: 138,
            g: 138,
            b: 138,
            a: 001,
        },
        246 => Color {
            r: 148,
            g: 148,
            b: 148,
            a: 001,
        },
        _ => Color {
            r: 000,
            g: 000,
            b: 000,
            a: 001,
        },
    }
}
