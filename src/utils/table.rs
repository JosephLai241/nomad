//! Displaying items in a neat table.

use ansi_term::Colour;
use ignore::types::FileTypeDef;
use self_update::update::Release;
use term_table::{row::Row, table_cell::TableCell, Table, TableStyle};

/// Contains information used to build a table.
pub struct TabledItems<T> {
    /// Items of type `T` to iterate.
    items: Vec<T>,
    /// Table labels.
    labels: Vec<String>,
    /// The max width of the table when displayed in the terminal.
    table_width: usize,
    /// An optional target to search for in the Vec of `items`.
    target: Option<String>,
}

impl<T> TabledItems<T> {
    /// Create a new `TabledItems` object.
    pub fn new(
        items: Vec<T>,
        labels: Vec<String>,
        table_width: usize,
        target: Option<String>,
    ) -> Self {
        Self {
            items,
            labels,
            table_width,
            target,
        }
    }
}

/// Enables a table view for a particular type.
pub trait TableView {
    /// Create and then display a table for this type of item.
    fn display_table(self)
    where
        Self: Sized,
    {
    }
}

impl TableView for TabledItems<FileTypeDef> {
    fn display_table(self) {
        let mut table = Table::new();

        table.max_column_width = self.table_width;
        table.style = TableStyle::rounded();

        table.add_row(Row::new(vec![
            TableCell::new(Colour::White.bold().paint("Name").to_string()),
            TableCell::new(Colour::White.bold().paint("Globs").to_string()),
        ]));

        let mut found = false;
        for definition in self.items {
            if let Some(ref filetype) = self.target {
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

        if let (Some(filetype), false) = (self.target, found) {
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
}

impl TableView for TabledItems<Release> {
    /// List all releases for `nomad`. Or optionally search for a version number.
    fn display_table(self) {
        let mut table = Table::new();

        table.max_column_width = self.table_width;
        table.style = TableStyle::rounded();

        table.add_row(Row::new(self.labels.into_iter().map(|label| {
            TableCell::new(Colour::White.bold().paint(label).to_string())
        })));

        let mut found = false;
        for release in self.items {
            if let Some(ref version) = self.target {
                if version == &release.version {
                    let mut assets_table = Table::new();
                    for asset in release.assets {
                        assets_table.add_row(Row::new(vec![TableCell::new(asset.name)]));
                        assets_table.add_row(Row::new(vec![TableCell::new(asset.download_url)]));
                    }

                    table.add_row(Row::new(vec![
                        TableCell::new(release.name),
                        TableCell::new(release.version),
                        TableCell::new(release.date),
                        TableCell::new(match release.body {
                            Some(body) => body,
                            None => "".to_string(),
                        }),
                        TableCell::new(assets_table),
                    ]));

                    found = true;
                    break;
                }
            } else {
                let mut assets_table = Table::new();
                for asset in release.assets {
                    assets_table.add_row(Row::new(vec![TableCell::new(asset.name)]));
                    assets_table.add_row(Row::new(vec![TableCell::new(asset.download_url)]));
                }

                table.add_row(Row::new(vec![
                    TableCell::new(release.name),
                    TableCell::new(release.version),
                    TableCell::new(release.date),
                    TableCell::new(match release.body {
                        Some(body) => body,
                        None => "".to_string(),
                    }),
                    TableCell::new(assets_table),
                ]));
            }
        }

        if let (Some(version), false) = (self.target, found) {
            println!(
                "{}",
                Colour::Red
                    .bold()
                    .paint(format!("\nDid not find a version matching {version}!\n"))
            );

            return;
        }

        println!("\n{}", table.render());
    }
}
