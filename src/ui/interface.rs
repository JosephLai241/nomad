//! The user interface of the TUI.

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Row, Table},
    Frame,
};

use crate::cli::global::GlobalArgs;

use super::{
    app::{App, PopupMode, UIMode},
    layouts::{
        get_error_popup_area, get_keybindings_area, get_settings_area, get_single_line_popup_area,
    },
    widgets::{cat_view, error_view, get_breadcrumbs, help_view, normal_view, nothing_found_view},
};

/// The message that is displayed in the `cat` view area if the user is in normal
/// mode and the current highlighted item is a directory.
const EMPTY_CAT_MESSAGE: &str = r#"
       ________  ________  ________  ________   _______
      ╱    ╱   ╲╱        ╲╱        ╲╱        ╲_╱       ╲
     ╱         ╱         ╱         ╱         ╱         ╱
    ╱         ╱         ╱         ╱         ╱         ╱
    ╲__╱_____╱╲________╱╲__╱__╱__╱╲___╱____╱╲________╱


    press <ENTER> or 'r' to enter this directory

    press 's' to display your current settings

    press <ESC> to cycle through widgets

    press 'K' to display the available keybindings for the current widget

    press 'q' to exit Rootless mode

    press '?' to display the help menu

"#;

/// Render the user interface for the TUI.
pub fn render_ui<B>(app: &mut App, args: &mut GlobalArgs, frame: &mut Frame<B>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(frame.size());

    match &app.ui_mode {
        UIMode::Breadcrumbs | UIMode::Inspect | UIMode::Normal => {
            frame.render_widget(get_breadcrumbs(app), chunks[0]);

            let normal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(chunks[1]);

            frame.render_stateful_widget(
                normal_view(app),
                normal_chunks[0],
                &mut app.directory_tree.state,
            );

            match cat_view(app) {
                Some(paragraph) => match paragraph {
                    Some(cat_view) => {
                        frame.render_widget(cat_view, normal_chunks[1]);
                    }
                    None => {
                        let centered_right_chunk = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Percentage(40),
                                Constraint::Percentage(5),
                                Constraint::Percentage(10),
                                Constraint::Percentage(5),
                                Constraint::Percentage(40),
                            ])
                            .split(normal_chunks[1])[2];

                        frame.render_widget(
                            Paragraph::new("<EMPTY>")
                                .alignment(Alignment::Center)
                                .style(
                                    Style::default().add_modifier(Modifier::BOLD).fg(Color::Red),
                                ),
                            centered_right_chunk,
                        );
                    }
                },
                None => {
                    let centered_info_chunk = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Percentage(20),
                            Constraint::Percentage(60),
                            Constraint::Percentage(20),
                        ])
                        .split(normal_chunks[1])[1];

                    frame.render_widget(
                        Paragraph::new(EMPTY_CAT_MESSAGE).style(
                            Style::default()
                                .add_modifier(Modifier::BOLD)
                                .add_modifier(Modifier::DIM),
                        ),
                        centered_info_chunk,
                    );
                }
            }

            match &app.popup_mode {
                PopupMode::Disabled => {}
                PopupMode::Error(error) => {
                    let error_area = get_error_popup_area(chunks[1]);

                    frame.render_widget(Clear, error_area);
                    frame.render_widget(error_view(error), error_area);
                }
                PopupMode::PatternInput => {
                    let popup_area = get_single_line_popup_area(chunks[1]);

                    frame.render_widget(Clear, popup_area);
                    frame.render_widget(
                        Paragraph::new(app.user_input.as_ref()).block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_style(
                                    Style::default()
                                        .add_modifier(Modifier::BOLD)
                                        .fg(app.nomad_style.tui.border_color),
                                )
                                .border_type(BorderType::Rounded)
                                .title_alignment(Alignment::Center)
                                .title(Spans::from(vec![
                                    Span::styled(
                                        " search for a pattern ",
                                        Style::default()
                                            .add_modifier(Modifier::BOLD)
                                            .fg(Color::White),
                                    ),
                                    Span::styled(
                                        "[",
                                        Style::default()
                                            .add_modifier(Modifier::BOLD)
                                            .fg(Color::White),
                                    ),
                                    Span::styled(
                                        match app.ui_mode {
                                            UIMode::Inspect => "FILE",
                                            UIMode::Normal => "TREE",
                                            _ => "NONE",
                                        },
                                        Style::default()
                                            .add_modifier(Modifier::BOLD)
                                            .fg(Color::Indexed(172)),
                                    ),
                                    Span::styled(
                                        "] ",
                                        Style::default()
                                            .add_modifier(Modifier::BOLD)
                                            .fg(Color::White),
                                    ),
                                ])),
                        ),
                        popup_area,
                    );

                    frame.set_cursor(
                        popup_area.x + app.user_input.len() as u16 + 1,
                        popup_area.y + 1,
                    );
                }
                PopupMode::NothingFound => {
                    let popup_area = get_single_line_popup_area(chunks[1]);

                    frame.render_widget(Clear, popup_area);
                    frame.render_widget(nothing_found_view(), popup_area);

                    args.regex.pattern = None;
                }
                PopupMode::Settings => {
                    let settings_area = get_settings_area(chunks[1]);
                    let settings_table = Table::new(app.app_settings.items.clone())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_style(
                                    Style::default()
                                        .add_modifier(Modifier::BOLD)
                                        .fg(app.nomad_style.tui.border_color),
                                )
                                .border_type(BorderType::Rounded)
                                .title(Span::styled(
                                    " ⚙️  current settings ",
                                    Style::default().fg(Color::White),
                                ))
                                .title_alignment(Alignment::Center),
                        )
                        .column_spacing(1)
                        .header(
                            Row::new(vec!["\n setting", "\nenabled/value"])
                                .height(3)
                                .style(Style::default().add_modifier(Modifier::BOLD)),
                        )
                        .highlight_style(
                            Style::default()
                                .add_modifier(Modifier::BOLD)
                                .bg(Color::Black),
                        )
                        .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);

                    frame.render_widget(Clear, settings_area);
                    frame.render_stateful_widget(
                        settings_table,
                        settings_area,
                        &mut app.app_settings.state,
                    );
                }
                PopupMode::ShowKeybindings => {
                    let keybindings_area = get_keybindings_area(chunks[1]);
                    let keybindings_table = Table::new(app.keybindings_for_mode.items.clone())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_style(
                                    Style::default()
                                        .add_modifier(Modifier::BOLD)
                                        .fg(app.nomad_style.tui.border_color),
                                )
                                .border_type(BorderType::Rounded)
                                .title(Spans::from(vec![
                                    Span::styled(
                                        " ⌨️  available keybindings ",
                                        Style::default().fg(Color::White),
                                    ),
                                    Span::styled(
                                        "[",
                                        Style::default()
                                            .add_modifier(Modifier::BOLD)
                                            .fg(Color::White),
                                    ),
                                    Span::styled(
                                        (match app.ui_mode {
                                            UIMode::Breadcrumbs => "BREADCRUMBS",
                                            UIMode::Inspect => "INSPECT",
                                            UIMode::Normal => "NORMAL",
                                            _ => "NONE",
                                        })
                                        .to_string(),
                                        Style::default()
                                            .add_modifier(Modifier::BOLD)
                                            .fg(Color::Indexed(172)),
                                    ),
                                    Span::styled(
                                        "] ",
                                        Style::default()
                                            .add_modifier(Modifier::BOLD)
                                            .fg(Color::White),
                                    ),
                                ]))
                                .title_alignment(Alignment::Center),
                        )
                        .column_spacing(1)
                        .header(
                            Row::new(vec!["\n key(s)", "\ndescription"])
                                .height(3)
                                .style(Style::default().add_modifier(Modifier::BOLD)),
                        )
                        .highlight_style(
                            Style::default()
                                .add_modifier(Modifier::BOLD)
                                .bg(Color::Black)
                                .fg(Color::White),
                        )
                        .widths(&[Constraint::Percentage(30), Constraint::Percentage(70)]);

                    frame.render_widget(Clear, keybindings_area);
                    frame.render_stateful_widget(
                        keybindings_table,
                        keybindings_area,
                        &mut app.keybindings_for_mode.state,
                    );
                }
            }
        }
        UIMode::Help => {
            let help_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ])
                .split(frame.size());
            let center_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ])
                .split(help_chunks[1]);

            frame.render_widget(help_view(app), center_chunks[1]);
        }
    }
}
