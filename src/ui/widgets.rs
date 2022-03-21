//! Widgets for the TUI.

use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs, Wrap},
};

use super::app::{App, PopupMode, UIMode};
use crate::ui::HELP_TEXT;

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
                .fg(app.nomad_style.tui.standard_item_highlight_color),
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
                                UIMode::Inspect => {
                                    if app.match_lines.items.is_empty() {
                                        Spans::from(Span::from(" 🧐 "))
                                    } else {
                                        Spans::from(vec![
                                            Span::styled(
                                                format!(" {} ", app.match_lines.items.len()),
                                                Style::default()
                                                    .add_modifier(Modifier::BOLD)
                                                    .fg(Color::White),
                                            ),
                                            Span::styled(
                                                format!(
                                                    "MATCH{} ",
                                                    if app.match_lines.items.len() > 1 {
                                                        "ES"
                                                    } else {
                                                        ""
                                                    }
                                                ),
                                                Style::default()
                                                    .add_modifier(Modifier::BOLD)
                                                    .fg(app.nomad_style.tui.regex.match_color),
                                            ),
                                            Span::styled(
                                                "[",
                                                Style::default()
                                                    .add_modifier(Modifier::BOLD)
                                                    .fg(Color::White),
                                            ),
                                            Span::styled(
                                                format!(
                                                    "{} / {}",
                                                    app.match_lines.state.selected().unwrap_or(1)
                                                        + 1,
                                                    app.match_lines.items.len()
                                                ),
                                                Style::default()
                                                    .add_modifier(Modifier::BOLD)
                                                    .fg(Color::White),
                                            ),
                                            Span::styled(
                                                "] ",
                                                Style::default()
                                                    .add_modifier(Modifier::BOLD)
                                                    .fg(Color::White),
                                            ),
                                        ])
                                    }
                                }
                                _ => Spans::from(Span::from("")),
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
pub fn error_view(error_message: &str) -> Paragraph<'_> {
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
