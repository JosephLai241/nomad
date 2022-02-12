//! Displaying items in a neat table.

use ansi_term::Colour;
use ignore::types::FileTypeDef;
use term_table::{row::Row, table_cell::TableCell, Table, TableStyle};

/// Render a table containing filetype definitions.
pub fn display_filetype_definitions(definitions: Vec<FileTypeDef>, filetype: Option<String>) {
    let mut table = Table::new();

    table.max_column_width = 120;
    table.style = TableStyle::rounded();

    let mut found = false;
    for definition in definitions {
        if let Some(ref filetype) = filetype {
            if definition.name() == filetype {
                table.add_row(Row::new(vec![
                    TableCell::new(definition.name()),
                    TableCell::new(definition.globs().join("\n")),
                ]));

                found = true;
                break;
            }
        } else {
            table.add_row(Row::new(vec![
                TableCell::new(definition.name()),
                TableCell::new(definition.globs().join("\n")),
            ]));
        }
    }

    if let (Some(filetype), false) = (filetype, found) {
        println!(
            "{}",
            Colour::Red
                .bold()
                .paint(format!("\nNo globs available for {filetype} filetypes!\n"))
        );

        return;
    }

    println!("\n{}", table.render());
}
