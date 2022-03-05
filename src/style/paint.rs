//! Set the styles for `nomad`.

use super::models::NomadStyle;
use crate::config::models::{Git, NomadConfig};

use ansi_term::{Colour, Style};

/// Return a struct containing user-specified or default settings.
pub fn process_settings(nomad_config: NomadConfig) -> NomadStyle {
    let mut nomad_style = NomadStyle::default();

    if let Some(git_settings) = nomad_config.git {
        process_git_settings(&mut nomad_style, &git_settings);
    }

    if let Some(regex_config) = nomad_config.regex {
        nomad_style.match_color = set_color(&regex_config.match_color, Colour::Fixed(033).bold());
    }

    nomad_style
}

/// Process the Git markers from `NomadConfig`.
fn process_git_settings(nomad_style: &mut NomadStyle, git_settings: &Git) {
    if let Some(colors) = &git_settings.colors {
        nomad_style.conflicted_color = set_color(&colors.conflicted_color, Colour::Red.bold());
        nomad_style.deleted_color = set_color(&colors.deleted_color, Colour::Red.bold());
        nomad_style.modified_color = set_color(&colors.modified_color, Colour::Fixed(172).bold());
        nomad_style.renamed_color = set_color(&colors.renamed_color, Colour::Red.bold());
        nomad_style.untracked_color = set_color(&colors.untracked_color, Colour::Fixed(243).bold());
    }

    if let Some(markers) = &git_settings.markers {
        nomad_style.conflicted_marker =
            set_marker(&markers.conflicted_marker, "CONFLICT".to_string());
        nomad_style.deleted_marker = set_marker(&markers.deleted_marker, "D".to_string());
        nomad_style.modified_marker = set_marker(&markers.modified_marker, "M".to_string());
        nomad_style.renamed_marker = set_marker(&markers.renamed_marker, "R".to_string());
        nomad_style.untracked_marker = set_marker(&markers.untracked_marker, "U".to_string());
    }
}

/// A helper function to set the color to a custom color or the default color.
fn set_color(color: &Option<String>, default_color: Style) -> Style {
    if let Some(color) = color {
        convert_to_style(color)
    } else {
        default_color
    }
}

/// A helper function to set the Git marker to a custom marker or the default marker.
fn set_marker(marker: &Option<String>, default_marker: String) -> String {
    if let Some(marker) = marker {
        marker.to_string()
    } else {
        default_marker
    }
}

/// Parse the default or 256 Xterm color into a `Style`.
fn convert_to_style(color: &str) -> Style {
    let style = match color {
        "black" => Colour::Black.bold(),
        "blue" => Colour::Blue.bold(),
        "cyan" => Colour::Cyan.bold(),
        "green" => Colour::Green.bold(),
        "purple" => Colour::Purple.bold(),
        "red" => Colour::Red.bold(),
        "white" => Colour::White.bold(),
        "yellow" => Colour::Yellow.bold(),
        _ => Colour::Fixed(convert_hex_code(color)).bold(),
    };

    style
}

/// Convert a hex code into a `u8`.
fn convert_hex_code(hex_code: &str) -> u8 {
    let eight_bit = match hex_code {
        "000000" => 000,
        "800000" => 001,
        "008000" => 002,
        "808000" => 003,
        "000080" => 004,
        "800080" => 005,
        "008080" => 006,
        "c0c0c0" => 007,
        "808080" => 008,
        "ff0000" => 009,
        "00ff00" => 010,
        "ffff00" => 011,
        "0000ff" => 012,
        "ff00ff" => 013,
        "00ffff" => 014,
        "ffffff" => 015,

        "00005f" => 017,
        "000087" => 018,
        "0000af" => 019,
        "0000d7" => 020,

        "005f00" => 022,
        "005f5f" => 023,
        "005f87" => 024,
        "005faf" => 025,
        "005fd7" => 026,
        "005fff" => 027,
        "008700" => 028,
        "00875f" => 029,
        "008787" => 030,
        "0087fa" => 031,
        "0087d7" => 032,
        "0087ff" => 033,
        "00fa00" => 034,
        "00fa5f" => 035,
        "00fa87" => 036,
        "00afaf" => 037,
        "00afd7" => 038,
        "00afff" => 039,
        "00d700" => 040,
        "00d75f" => 041,
        "00d787" => 042,
        "00d7af" => 043,
        "00d7d7" => 044,
        "00d7ff" => 045,

        "00ff5f" => 047,
        "00ff87" => 048,
        "00ffaf" => 049,
        "00ffd7" => 050,

        "5f0000" => 052,
        "5f005f" => 053,
        "5f0087" => 054,
        "5f00af" => 055,
        "5f00d7" => 056,
        "5f00ff" => 057,
        "5f5f00" => 058,
        "5f5f5f" => 059,
        "5f5f87" => 060,
        "5f5faf" => 061,
        "5f5fd7" => 062,
        "5f5fff" => 063,
        "5f8700" => 064,
        "5f875f" => 065,
        "5f8787" => 066,
        "5f87af" => 067,
        "5f87d7" => 068,
        "5f87ff" => 069,
        "5faf00" => 070,
        "5faf5f" => 071,
        "5faf87" => 072,
        "5fafaf" => 073,
        "5fafd7" => 074,
        "5fafff" => 075,
        "5fd700" => 076,
        "5fd75f" => 077,
        "5fd787" => 078,
        "5fd7af" => 079,
        "5fd7d7" => 080,
        "5fd7ff" => 081,
        "5fff00" => 082,
        "5fff5f" => 083,
        "5fff87" => 084,
        "5fffaf" => 085,
        "5fffd7" => 086,
        "5fffff" => 087,
        "870000" => 088,
        "87005f" => 089,
        "870087" => 090,
        "8700af" => 091,
        "8700d7" => 092,
        "8700ff" => 093,
        "875f00" => 094,
        "875f5f" => 095,
        "875f87" => 096,
        "875faf" => 097,
        "875fd7" => 098,
        "875fff" => 099,
        "878700" => 100,
        "87875f" => 101,
        "878787" => 102,
        "8787af" => 103,
        "8787d7" => 104,
        "8787ff" => 105,
        "87af00" => 106,
        "87af5f" => 107,
        "87af87" => 108,
        "87afaf" => 109,
        "87afd7" => 110,
        "87afff" => 111,
        "87d700" => 112,
        "87d75f" => 113,
        "87d787" => 114,
        "87d7af" => 115,
        "87d7d7" => 116,
        "87d7ff" => 117,
        "87ff00" => 118,
        "87ff5f" => 119,
        "87ff87" => 120,
        "87ffaf" => 121,
        "87ffd7" => 122,
        "87ffff" => 123,
        "af0000" => 124,
        "af005f" => 125,
        "af0087" => 126,
        "af00af" => 127,
        "af00d7" => 128,
        "af00ff" => 129,
        "af5f00" => 130,
        "af5f5f" => 131,
        "af5f87" => 132,
        "af5faf" => 133,
        "af5fd7" => 134,
        "af5fff" => 135,
        "af8700" => 136,
        "af875f" => 137,
        "af8787" => 138,
        "af87af" => 139,
        "af87d7" => 140,
        "af87ff" => 141,
        "afaf00" => 142,
        "afaf5f" => 143,
        "afaf87" => 144,
        "afafaf" => 145,
        "afafd7" => 146,
        "afafff" => 147,
        "afd700" => 148,
        "afd75f" => 149,
        "afd787" => 150,
        "afd7af" => 151,
        "afd7d7" => 152,
        "afd7ff" => 153,
        "afff00" => 154,
        "afff5f" => 155,
        "afff87" => 156,
        "afffaf" => 157,
        "afffd7" => 158,
        "afffff" => 159,
        "d70000" => 160,
        "d7005f" => 161,
        "d70087" => 162,
        "d700af" => 163,
        "d700d7" => 164,
        "d700ff" => 165,
        "d75f00" => 166,
        "d75f5f" => 167,
        "d75f87" => 168,
        "d75faf" => 169,
        "d75fd7" => 170,
        "d75fff" => 171,
        "d78700" => 172,
        "d7875f" => 173,
        "d78787" => 174,
        "d787af" => 175,
        "d787d7" => 176,
        "d787ff" => 177,
        "dfaf00" => 178,
        "dfaf5f" => 179,
        "dfaf87" => 180,
        "dfafaf" => 181,
        "dfafdf" => 182,
        "dfafff" => 183,
        "dfdf00" => 184,
        "dfdf5f" => 185,
        "dfdf87" => 186,
        "dfdfaf" => 187,
        "dfdfdf" => 188,
        "dfdfff" => 189,
        "dfff00" => 190,
        "dfff5f" => 191,
        "dfff87" => 192,
        "dfffaf" => 193,
        "dfffdf" => 194,
        "dfffff" => 195,

        "ff005f" => 197,
        "ff0087" => 198,
        "ff00af" => 199,
        "ff00df" => 200,

        "ff5f00" => 202,
        "ff5f5f" => 203,
        "ff5f87" => 204,
        "ff5faf" => 205,
        "ff5fdf" => 206,
        "ff5fff" => 207,
        "ff8700" => 208,
        "ff875f" => 209,
        "ff8787" => 210,
        "ff87af" => 211,
        "ff87df" => 212,
        "ff87ff" => 213,
        "ffaf00" => 214,
        "ffaf5f" => 215,
        "ffaf87" => 216,
        "ffafaf" => 217,
        "ffafdf" => 218,
        "ffafff" => 219,
        "ffdf00" => 220,
        "ffdf5f" => 221,
        "ffdf87" => 222,
        "ffdfaf" => 223,
        "ffdfdf" => 224,
        "ffdfff" => 225,

        "ffff5f" => 227,
        "ffff87" => 228,
        "ffffaf" => 229,
        "ffffdf" => 230,

        "080808" => 232,
        "121212" => 233,
        "1c1c1c" => 234,
        "262626" => 235,
        "303030" => 236,
        "3a3a3a" => 237,
        "444444" => 238,
        "4e4e4e" => 239,
        "585858" => 240,
        "626262" => 241,
        "6c6c6c" => 242,
        "767676" => 243,

        "8a8a8a" => 245,
        "949494" => 246,
        "9e9e9e" => 247,
        "a8a8a8" => 248,
        "b2b2b2" => 249,
        "bcbcbc" => 250,
        "c6c6c6" => 251,
        "d0d0d0" => 252,
        "dadada" => 253,
        "e4e4e4" => 254,
        "eeeeee" | _ => 255,
    };

    eight_bit
}
