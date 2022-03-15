//! Widgets for the TUI.

use itertools::Itertools;
use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs, Wrap},
};

use super::app::{App, PopupMode, UIMode};

/// The help text displayed in the help menu after pressing '?'.
pub const HELP_TEXT: &'static str = r#"
 Use the directional or Vim directional keys [j, k] to scroll.
 Press <ESC> to exit this screen.

 Table of Contents
 =================

 - Keybindings
 - Modes


 Views
 =====

 Interactive mode has 4 views:

     * Normal/breadcrumbs
     *

 Keybindings
 ===========

 Navigation
 ----------

 Use the directional keys or Vim directional keys [h, j, k, l] to navigate the TUI.

 Commands
 --------

 0           In general, scroll to the top of the widget you are in

 1 - 9       Scroll `n` lines/items down. `shift` + `n` scrolls `n` lines/items up.

 d           Toggle only displaying directories

 e           Open the selected file in a text editor (Neovim, Vim, Vi, or Nano)
             This only applies if the selected item is a file

 g           Toggle Git markers

 h           Toggle displaying hidden items

 i           Toggle icons

 l           Toggle directory labels

 m           Toggle metadata

 n           Toggle item labels

 p           Toggle plain mode

 q           Quit interactive mode

 r           Refresh the tree

 s           Display the settings for the current tree

 x           Export the tree to a file. Optionally provide a filename

 D           Toggle disrespecting all rules specified in ignore-type files

 L           Toggle all labels (directories and items)

 R           Reset all options to its default value and refresh the current tree

 /           Search for a pattern. Supports regex expressions

 ?           Display this help message

 <ENTER>     If a file is selected       Enter scroll mode
             If a directory is selected  Enter the selected directory and redraw
                                         the tree
 <ESC>       Cycle through modes/widgets


 Modes
 =====

 This UI has four modes:

 * Breadcrumbs   Move focus to the breadcrumbs at the top of the TUI and
                 select a different directory to inspect
 * Normal        This is the default mode when interactive mode is instantiated
 * Scroll        This mode may be entered if the currently highlighted item is a file

"#;

/// Set the breadcrumbs displayed at the top of the TUI.
pub fn get_breadcrumbs<'a>(app: &App) -> Tabs<'a> {
    let breadcrumb_labels: Vec<Spans> = app
        .breadcrumbs
        .items
        .iter()
        .map(|title| Spans::from(vec![Span::raw(title.to_string())]))
        .collect();

    Tabs::new(breadcrumb_labels)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(match app.ui_mode {
                    UIMode::Breadcrumbs => Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(app.nomad_style.tui.border_color)
                        .bg(Color::Black),
                    _ => Style::default(),
                })
                .border_type(BorderType::Rounded),
        )
        .highlight_style(match app.ui_mode {
            UIMode::Breadcrumbs => Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(app.nomad_style.tui.border_color)
                .fg(Color::Black),
            _ => Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black)
                .fg(app.nomad_style.tui.border_color),
        })
        .select(
            app.breadcrumbs
                .state
                .selected()
                .map_or(app.breadcrumbs.items.len() - 1, |index| index),
        )
        .style(match app.ui_mode {
            UIMode::Breadcrumbs => Style::default().add_modifier(Modifier::BOLD),
            _ => Style::default().add_modifier(Modifier::DIM),
        })
}

/// Display the directory tree and any `cat`ted files. This is the default (normal) TUI mode.
pub fn normal_view<'a>(app: &App) -> List<'a> {
    let directory_tree = app
        .directory_tree
        .items
        .iter()
        .map(|item| ListItem::new(item.clone()))
        .collect::<Vec<ListItem>>();

    let text_color = app.get_git_color();
    let text_style = match text_color {
        Color::Reset => Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(app.nomad_style.tui.standard_item_highlight_color),
        _ => Style::default().add_modifier(Modifier::BOLD).fg(text_color),
    };

    List::new(directory_tree)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(match app.ui_mode {
                    UIMode::Normal => Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(app.nomad_style.tui.border_color),
                    _ => Style::default(),
                })
                .border_type(BorderType::Rounded),
        )
        .highlight_style(text_style)
        .style(match app.ui_mode {
            UIMode::Normal => match app.popup_mode {
                PopupMode::Disabled => Style::default(),
                _ => Style::default().add_modifier(Modifier::DIM),
            },
            _ => Style::default().add_modifier(Modifier::DIM),
        })
}

/// Display the `cat`ed file.
pub fn cat_view<'a>(app: &'a App) -> Option<Option<Paragraph<'a>>> {
    match &app.file_contents {
        Some(file) => match file {
            Some(contents) => Some(Some(
                Paragraph::new(contents.clone())
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(match app.ui_mode {
                                UIMode::Inspect => Style::default()
                                    .add_modifier(Modifier::BOLD)
                                    .fg(app.nomad_style.tui.border_color),
                                _ => Style::default(),
                            })
                            .border_type(BorderType::Rounded)
                            .title(match app.ui_mode {
                                UIMode::Inspect => " 🧐 ",
                                _ => "",
                            }),
                    )
                    .scroll((app.scroll, 0))
                    .style(match app.popup_mode {
                        PopupMode::Disabled => match app.ui_mode {
                            UIMode::Inspect => Style::default(),
                            _ => Style::default().add_modifier(Modifier::DIM),
                        },
                        _ => Style::default().add_modifier(Modifier::DIM),
                    })
                    .wrap(Wrap { trim: false }),
            )),
            None => Some(None),
        },
        None => None,
    }
}

/// Display a message that just says nothing was found. This is displayed if no
/// directory items were found after filtering.
pub fn nothing_found_view<'a>() -> Paragraph<'a> {
    Paragraph::new("nothing was found 😞")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .border_type(BorderType::Rounded)
                .style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC)
                        .fg(Color::Red),
                ),
        )
        .wrap(Wrap { trim: false })
}

/// Display the help TUI mode.
pub fn help_view<'a>(app: &App) -> Paragraph<'a> {
    Paragraph::new(HELP_TEXT)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.nomad_style.tui.border_color))
                .border_type(BorderType::Rounded)
                .title(Span::styled(
                    " ❓ help ",
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::White),
                )),
        )
        .scroll((app.scroll, 0))
        .wrap(Wrap { trim: false })
}

/// Display a message containing the error that was raised.
pub fn error_view<'a>(error_message: &'a str) -> Paragraph<'a> {
    Paragraph::new(format!("\n{error_message}"))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .border_type(BorderType::Rounded)
                .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red))
                .title(" ⚠️   ERROR  ⚠️  ")
                .title_alignment(Alignment::Center),
        )
        .wrap(Wrap { trim: false })
}
