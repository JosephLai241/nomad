//! The user interface for interactive mode.

pub mod app;
pub mod layouts;
pub mod stateful_widgets;
pub mod widgets;

use self::{
    app::{App, PopupMode, UIMode},
    layouts::{get_settings_area, get_single_line_popup_area},
    widgets::{
        cat_view, error_view, get_breadcrumbs, help_view, normal_view, nothing_found_view,
        HELP_TEXT,
    },
};
use crate::{cli::Args, errors::NomadError, style::models::NomadStyle};

use anyhow::Result;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph, Row, Table},
    Terminal,
};

use std::{
    io::stdout,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

/// Variants for UI events.
enum Event<I> {
    /// Detected input from the user.
    Input(I),
    /// The UI is idle.
    Tick,
}

/// Enter `nomad`'s interactive mode.
pub fn enter_interactive_mode(
    args: &mut Args,
    nomad_style: &NomadStyle,
    target_directory: &str,
) -> Result<(), NomadError> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Set up input handling.
    let (sender, receiver) = mpsc::channel();

    let tick_rate = Duration::from_millis(100);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // Poll for a tick rate duration. Send a tick event if there are no events.
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            // Check if there is an event. If there is, send the event key on the channel.
            if event::poll(timeout).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    sender.send(Event::Input(key)).unwrap();
                }
            }

            // Send an `Event::Tick` if no events are available.
            if last_tick.elapsed() >= tick_rate {
                sender.send(Event::Tick).unwrap_or(());
                last_tick = Instant::now();
            }
        }
    });

    let max_help_scroll = HELP_TEXT.as_bytes().iter().filter(|&&c| c == b'\n').count();
    let mut app = App::new(args, nomad_style, target_directory)?;

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(frame.size());

            match &app.ui_mode {
                UIMode::Breadcrumbs | UIMode::Normal => {
                    frame.render_widget(get_breadcrumbs(&app), chunks[0]);

                    let normal_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                        .split(chunks[1]);

                    frame.render_stateful_widget(
                        normal_view(&app),
                        normal_chunks[0],
                        &mut app.directory_tree.state,
                    );

                    let centered_right_chunk = Layout::default()
                        .direction(Direction::Vertical).constraints([
                            Constraint::Percentage(40),
                            Constraint::Percentage(5),
                            Constraint::Percentage(10),
                            Constraint::Percentage(5),
                            Constraint::Percentage(40),
                        ])
                        .split(normal_chunks[1])[2];

                    match cat_view(&app) {
                        Some(paragraph) => match paragraph {
                            Some(cat_view) => {
                                frame.render_widget(cat_view, normal_chunks[1]);
                            }
                            None => {
                                frame.render_widget(
                                    Paragraph::new("<EMPTY>")
                                        .alignment(Alignment::Center)
                                        .style(Style::default()
                                        .add_modifier(Modifier::BOLD)
                                        .fg(Color::Red)),
                                    centered_right_chunk
                                );
                            }
                        }
                        None => {
                            frame.render_widget(
                                Paragraph::new("Press <ENTER> to refresh the TUI with the contents of this directory.")
                                    .alignment(Alignment::Center),
                                centered_right_chunk
                            );
                        }
                    }


                    match &app.popup_mode {
                        PopupMode::Disabled => {}
                        PopupMode::Error(error) => {
                            let popup_area = get_single_line_popup_area(chunks[1]);

                            frame.render_widget(Clear, popup_area);
                            frame.render_widget(error_view(&error), popup_area);
                        }
                        PopupMode::Export => {}
                        PopupMode::NothingFound => {
                            let popup_area = get_single_line_popup_area(chunks[1]);

                            frame.render_widget(Clear, popup_area);
                            frame.render_widget(nothing_found_view(), popup_area);

                            args.pattern = None;
                        }
                        PopupMode::PatternInput => {
                            let popup_area = get_single_line_popup_area(chunks[1]);

                            frame.render_widget(Clear, popup_area);
                            frame.render_widget(
                                Paragraph::new(app.user_input.as_ref())
                                    .style(Style::default().fg(Color::Blue))
                                    .block(
                                        Block::default()
                                            .borders(Borders::ALL)
                                            .title_alignment(Alignment::Center)
                                            .title(" search for a pattern "),
                                    ),
                                popup_area,
                            );

                            frame.set_cursor(
                                popup_area.x + app.user_input.len() as u16 + 1,
                                popup_area.y + 1,
                            );
                        }
                        PopupMode::Reloading => {
                            let reloading_popup_area = get_single_line_popup_area(chunks[1]);
                            frame.render_widget(
                                Paragraph::new("RELOADING...").alignment(Alignment::Center),
                                reloading_popup_area,
                            );
                        }
                        PopupMode::Settings => {
                            let settings_area = get_settings_area(chunks[1]);
                            let settings_table = Table::new(app.app_settings.items.clone())
                                .block(
                                    Block::default()
                                        .borders(Borders::ALL)
                                        .title(" ⚙️  current settings ")
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

                    frame.render_widget(help_view(&app), center_chunks[1]);
                }
                _ => {}
            }
        })?;

        // Handle keyboard events.
        match receiver.recv()? {
            Event::Input(event) => {
                match app.popup_mode {
                    PopupMode::Disabled => {
                        match event.code {
                            // =============
                            // TUI commands.
                            // =============

                            // Enter search/pattern match mode.
                            KeyCode::Char('/') => match app.ui_mode {
                                UIMode::Normal => app.popup_mode = PopupMode::PatternInput,
                                _ => {}
                            },
                            // In Normal mode, toggle only showing directories.
                            KeyCode::Char('d') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.dirs = !args.dirs;
                                    app.refresh(args, nomad_style, target_directory)?;
                                }
                                _ => {}
                            },
                            // In Normal mode, open the highlighted item(s) in an editor.
                            KeyCode::Char('e') => match app.ui_mode {
                                UIMode::Normal => {
                                    // TODO: EXIT THE ALTERNATE SCREEN AND SPAWN AN EDITOR.
                                    //       RETURN TO THE ALTERNATE SCREEN ONCE THE EDITOR
                                    //       IS CLOSED.
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle Git markers.
                            KeyCode::Char('g') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.no_git = !args.no_git;
                                    app.refresh(args, nomad_style, target_directory)?;
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle showing hidden directories.
                            KeyCode::Char('h') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.hidden = !args.hidden;
                                    app.refresh(args, nomad_style, target_directory)?;
                                }
                                UIMode::Breadcrumbs => {
                                    if app.breadcrumbs.state.selected().is_none() {
                                        app.breadcrumbs
                                            .state
                                            .select(Some(app.breadcrumbs.items.len() - 1));
                                    }
                                    app.breadcrumbs.previous();
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle icons.
                            KeyCode::Char('i') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.no_icons = !args.no_icons;
                                    app.refresh(args, nomad_style, target_directory)?;
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle directory labels.
                            KeyCode::Char('l') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.label_directories = !args.label_directories;
                                    app.refresh(args, nomad_style, target_directory)?;
                                }
                                UIMode::Breadcrumbs => {
                                    if app.breadcrumbs.state.selected().is_none() {
                                        app.breadcrumbs
                                            .state
                                            .select(Some(app.breadcrumbs.items.len() - 1));
                                    }
                                    app.breadcrumbs.next();
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle showing metadata for all items.
                            KeyCode::Char('m') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.metadata = !args.metadata;
                                    app.refresh(args, nomad_style, target_directory)?;
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle numbered items.
                            KeyCode::Char('n') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.numbers = !args.numbers;
                                    app.refresh(args, nomad_style, target_directory)?;
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle plain mode.
                            KeyCode::Char('p') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.plain = !args.plain;
                                    app.refresh(args, nomad_style, target_directory)?;
                                }
                                _ => {}
                            },
                            // Quit interactive mode.
                            KeyCode::Char('q') => {
                                disable_raw_mode()?;
                                execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                                terminal.show_cursor()?;
                                break;
                            }
                            // In Normal mode, display all settings.
                            KeyCode::Char('s') => match app.ui_mode {
                                UIMode::Normal => app.popup_mode = PopupMode::Settings,
                                _ => {}
                            },
                            KeyCode::Char('x') => match app.ui_mode {
                                UIMode::Normal => {
                                    app.popup_mode = PopupMode::Export;
                                    //
                                    //
                                    //
                                    // TODO: UPDATE THIS
                                    //
                                    //
                                    //
                                    //args.export = !args.plain;
                                    app.refresh(args, nomad_style, target_directory)?;
                                }
                                _ => {}
                            },
                            // In Normal mode, disrespect all `.ignore` rules.
                            KeyCode::Char('D') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.disrespect = !args.disrespect;
                                    app.refresh(args, nomad_style, target_directory)?;
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle applying all labels.
                            KeyCode::Char('L') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.all_labels = !args.all_labels;
                                    app.refresh(args, nomad_style, target_directory)?;
                                }
                                _ => {}
                            },
                            // Reload the tree.
                            KeyCode::Char('R') => match app.ui_mode {
                                UIMode::Normal => {
                                    app.refresh(args, nomad_style, target_directory)?;
                                }
                                _ => {}
                            },
                            // Enter help mode/display the help message.
                            KeyCode::Char('?') => match app.ui_mode {
                                UIMode::Help => {}
                                _ => app.ui_mode = UIMode::Help,
                            },
                            // Different operations depending on the UI mode:
                            // * Breadcrumbs or Normal mode - cycles between the two modes.
                            // * Help - exits the help screen.
                            KeyCode::Esc => match app.ui_mode {
                                UIMode::Breadcrumbs => app.ui_mode = UIMode::Normal,
                                UIMode::Help => {
                                    app.ui_mode = UIMode::Normal;
                                    app.scroll = 0;
                                }
                                UIMode::Inspect => app.ui_mode = UIMode::Normal,
                                UIMode::Normal => app.ui_mode = UIMode::Breadcrumbs,
                                _ => {}
                            },
                            // Different operations dependent on the UI mode:
                            // * Breadcrumbs
                            //     + Refresh the UI with a new tree and updated breadcrumbs.
                            // * Normal (tree)
                            //     + If a directory is selected, refresh the UI with a new
                            //       tree and updated breadcrumbs.
                            //     + If a file is selected, enter the file and enable scrolling.
                            KeyCode::Enter => match app.ui_mode {
                                UIMode::Breadcrumbs => {
                                    app.refresh(
                                        args,
                                        nomad_style,
                                        &format!(
                                            "/{}",
                                            app.breadcrumbs.items[0..app
                                                .breadcrumbs
                                                .state
                                                .selected()
                                                .map_or(
                                                    app.breadcrumbs.items.len() - 1,
                                                    |index| index + 1
                                                )]
                                                .join("/")
                                                .to_string()
                                        ),
                                    )?;
                                    app.ui_mode = UIMode::Normal;
                                }
                                UIMode::Normal => {
                                    // TODO: CHECK IF SELECTED ITEM IS A DIR OR FILE
                                    //       IF FILE:
                                    //          EDIT THE FILE BY OPENING IT WITH THE EDITOR
                                    //       IF DIR:
                                    //          REFRESH THE APP LIKE IF IT WERE IN BREADCRUMBS MODE
                                    //
                                }
                                _ => {}
                            },

                            // ===========
                            // Navigation.
                            // ===========

                            // Cycle through the breadcrumbs or lateral scrolling when
                            // inspecting a file.
                            KeyCode::Left => match app.ui_mode {
                                UIMode::Breadcrumbs => {
                                    if app.breadcrumbs.state.selected().is_none() {
                                        app.breadcrumbs
                                            .state
                                            .select(Some(app.breadcrumbs.items.len() - 1));
                                    }
                                    app.breadcrumbs.previous();
                                }
                                UIMode::Inspect => {
                                    // TODO: ENABLE LATERAL SCROLLING AND SCROLL LEFT.
                                }
                                _ => {}
                            },
                            // Cycle through the breadcrumbs or lateral scrolling when
                            // inspecting a file.
                            KeyCode::Right => match app.ui_mode {
                                UIMode::Breadcrumbs => {
                                    if app.breadcrumbs.state.selected().is_none() {
                                        app.breadcrumbs
                                            .state
                                            .select(Some(app.breadcrumbs.items.len() - 1));
                                    }
                                    app.breadcrumbs.next();
                                }
                                UIMode::Inspect => {
                                    // TODO: ENABLE LATERAL SCROLLING AND SCROLL RIGHT.
                                }
                                UIMode::Normal => {
                                    // TODO: MAKE IT GO TO THE NEXT COLUMN
                                }
                                _ => {}
                            },
                            // Scroll up the directory tree, file, settings, or help menu.
                            KeyCode::Up | KeyCode::Char('k') => match app.ui_mode {
                                UIMode::Help => {
                                    if app.scroll != 0 {
                                        app.scroll -= 1;
                                    }
                                }
                                UIMode::Normal => {
                                    app.directory_tree.previous();
                                    app.cat_file()?;
                                }
                                _ => {}
                            },
                            // Scroll down the directory tree, file, settings, or help menu.
                            KeyCode::Down | KeyCode::Char('j') => match app.ui_mode {
                                UIMode::Help => {
                                    if (app.scroll as usize) + 20 < max_help_scroll {
                                        app.scroll += 1
                                    }
                                }
                                UIMode::Normal => {
                                    app.directory_tree.next();
                                    app.cat_file()?;
                                }
                                _ => {}
                            },

                            // Scroll to the top or beginning of a widget.
                            KeyCode::Char('0') => match app.ui_mode {
                                UIMode::Normal => {
                                    app.directory_items.state.select(Some(0));
                                    app.directory_tree.state.select(Some(0));
                                }
                                UIMode::Help => app.scroll = 0,
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                    PopupMode::Export | PopupMode::PatternInput => match event.code {
                        KeyCode::Backspace => {
                            app.user_input.pop();
                        }
                        KeyCode::Char(ch) => {
                            app.user_input.push(ch);
                        }
                        KeyCode::Enter => {
                            app.collected_input.push(app.user_input.drain(..).collect());
                            app.pattern_search(args, nomad_style, target_directory)?;
                        }
                        KeyCode::Esc => {
                            app.user_input.clear();
                            app.popup_mode = PopupMode::Disabled;
                        }
                        _ => {}
                    },
                    PopupMode::NothingFound => match event.code {
                        KeyCode::Char('/') => app.popup_mode = PopupMode::PatternInput,
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                            terminal.show_cursor()?;
                            break;
                        }
                        _ => app.popup_mode = PopupMode::Disabled,
                    },
                    PopupMode::Settings => match event.code {
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                            terminal.show_cursor()?;
                            break;
                        }
                        KeyCode::Char('s') => app.popup_mode = PopupMode::Disabled,
                        KeyCode::Up | KeyCode::Char('k') => {
                            //app.app_settings.state.select(Some(
                            //match app.app_settings.state.selected() {
                            //Some(i) => i,
                            //None => 0,
                            //},
                            //));
                            app.app_settings.previous()
                        }
                        KeyCode::Down | KeyCode::Char('j') => app.app_settings.next(),
                        KeyCode::Char('0') => app.scroll = 0,

                        _ => {}
                    },
                    _ => {}
                }
            }
            Event::Tick => {}
        }
    }

    Ok(())
}
