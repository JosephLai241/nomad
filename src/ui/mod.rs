//! The user interface for interactive mode.

pub mod app;
pub mod interface;
pub mod layouts;
pub mod stateful_widgets;
pub mod utils;
pub mod widgets;

use self::{
    app::{App, PopupMode, UIMode},
    interface::render_ui,
    utils::reset_args,
    widgets::HELP_TEXT,
};
use crate::{cli::Args, errors::NomadError, style::models::NomadStyle};

use anyhow::Result;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

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
        terminal.draw(|frame| render_ui(&mut app, args, frame))?;

        // Handle keyboard events.
        match receiver.recv()? {
            Event::Input(event) => {
                match app.popup_mode {
                    // ============
                    // Normal mode.
                    // ============
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
                                    if let Err(error) =
                                        app.refresh(args, nomad_style, target_directory)
                                    {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    }
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle Git markers.
                            KeyCode::Char('g') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.no_git = !args.no_git;
                                    if let Err(error) =
                                        app.refresh(args, nomad_style, target_directory)
                                    {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    }
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle showing hidden directories.
                            KeyCode::Char('h') => match app.ui_mode {
                                UIMode::Breadcrumbs => {
                                    if app.breadcrumbs.state.selected().is_none() {
                                        app.breadcrumbs
                                            .state
                                            .select(Some(app.breadcrumbs.items.len() - 1));
                                    }
                                    app.breadcrumbs.previous();
                                }
                                UIMode::Normal => {
                                    args.hidden = !args.hidden;
                                    if let Err(error) =
                                        app.refresh(args, nomad_style, target_directory)
                                    {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    }
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle icons.
                            KeyCode::Char('i') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.no_icons = !args.no_icons;
                                    if let Err(error) =
                                        app.refresh(args, nomad_style, target_directory)
                                    {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    }
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle directory labels.
                            KeyCode::Char('l') => match app.ui_mode {
                                UIMode::Breadcrumbs => {
                                    if app.breadcrumbs.state.selected().is_none() {
                                        app.breadcrumbs
                                            .state
                                            .select(Some(app.breadcrumbs.items.len() - 1));
                                    }
                                    app.breadcrumbs.next();
                                }
                                UIMode::Normal => {
                                    args.label_directories = !args.label_directories;
                                    if let Err(error) =
                                        app.refresh(args, nomad_style, target_directory)
                                    {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    }
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle showing metadata for all items.
                            KeyCode::Char('m') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.metadata = !args.metadata;
                                    if let Err(error) =
                                        app.refresh(args, nomad_style, target_directory)
                                    {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    }
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle numbered items.
                            KeyCode::Char('n') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.numbers = !args.numbers;
                                    if let Err(error) =
                                        app.refresh(args, nomad_style, target_directory)
                                    {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    }
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle plain mode.
                            KeyCode::Char('p') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.plain = !args.plain;
                                    if let Err(error) =
                                        app.refresh(args, nomad_style, target_directory)
                                    {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    }
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
                            // Reload the tree.
                            KeyCode::Char('r') => match app.ui_mode {
                                UIMode::Normal => {
                                    if let Err(error) =
                                        app.refresh(args, nomad_style, target_directory)
                                    {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    }
                                }
                                _ => {}
                            },
                            // In Normal mode, display all settings.
                            KeyCode::Char('s') => match app.ui_mode {
                                UIMode::Normal => app.popup_mode = PopupMode::Settings,
                                _ => {}
                            },
                            // In Normal mode, disrespect all `.ignore` rules.
                            KeyCode::Char('D') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.disrespect = !args.disrespect;
                                    if let Err(error) =
                                        app.refresh(args, nomad_style, target_directory)
                                    {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    }
                                }
                                _ => {}
                            },
                            // In Normal mode, toggle applying all labels.
                            KeyCode::Char('L') => match app.ui_mode {
                                UIMode::Normal => {
                                    args.all_labels = !args.all_labels;
                                    if let Err(error) =
                                        app.refresh(args, nomad_style, target_directory)
                                    {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    }
                                }
                                _ => {}
                            },
                            // Reset all arguments.
                            KeyCode::Char('R') => match app.ui_mode {
                                UIMode::Normal => {
                                    reset_args(args);
                                    if let Err(error) =
                                        app.refresh(args, nomad_style, target_directory)
                                    {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    }
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
                                UIMode::Breadcrumbs => {
                                    app.breadcrumbs
                                        .state
                                        .select(Some(app.breadcrumbs.items.len() - 1));
                                    app.ui_mode = UIMode::Normal;
                                }
                                UIMode::Help | UIMode::Inspect => {
                                    app.ui_mode = UIMode::Normal;
                                    app.scroll = 0;
                                }
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
                                    if let Err(error) = app.refresh(
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
                                    ) {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    } else {
                                        app.ui_mode = UIMode::Normal;
                                    }
                                }
                                UIMode::Normal => match app.selected_is_dir()? {
                                    Some(is_dir) => {
                                        if is_dir {
                                            if let Err(error) =
                                                app.refresh(args, nomad_style, target_directory)
                                            {
                                                app.popup_mode =
                                                    PopupMode::Error(error.to_string());
                                            }
                                        } else {
                                            app.ui_mode = UIMode::Inspect;
                                        }
                                    }
                                    None => {}
                                },
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
                                _ => {}
                            },
                            // Scroll up the directory tree, file, settings, or help menu.
                            KeyCode::Up | KeyCode::Char('k') => match app.ui_mode {
                                UIMode::Help | UIMode::Inspect => {
                                    if app.scroll != 0 {
                                        app.scroll -= 1;
                                    }
                                }
                                UIMode::Normal => {
                                    app.directory_tree.previous();
                                    if let Err(error) = app.cat_file() {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    }
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
                                UIMode::Inspect => {
                                    // TODO: ADD ANOTHER FIELD IN THE APP THAT STORES THE NUMBER OF
                                    // LINES IN A FILE?
                                    app.scroll += 1;
                                }
                                UIMode::Normal => {
                                    app.directory_tree.next();
                                    if let Err(error) = app.cat_file() {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    }
                                }
                                _ => {}
                            },

                            // Scroll to the top or beginning of a widget.
                            KeyCode::Char('0') => match app.ui_mode {
                                UIMode::Help | UIMode::Inspect => app.scroll = 0,
                                UIMode::Normal => {
                                    if let Some(ref mut directory_items) = app.directory_items {
                                        directory_items.state.select(Some(0));
                                    }
                                    app.directory_tree.state.select(Some(0));

                                    if let Err(error) = app.cat_file() {
                                        app.popup_mode = PopupMode::Error(error.to_string());
                                    }
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }

                    // ===========
                    // Error mode.
                    // ===========
                    PopupMode::Error(_) => match event.code {
                        _ => {
                            disable_raw_mode()?;
                            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                            terminal.show_cursor()?;
                            break;
                        }
                    },

                    // ===========
                    // Input mode.
                    // ===========
                    PopupMode::PatternInput => match event.code {
                        KeyCode::Backspace => {
                            app.user_input.pop();
                        }
                        KeyCode::Char(ch) => {
                            app.user_input.push(ch);
                        }
                        KeyCode::Enter => {
                            app.collected_input.push(app.user_input.drain(..).collect());

                            if let Err(error) =
                                app.pattern_search(args, nomad_style, target_directory)
                            {
                                app.popup_mode = PopupMode::Error(error.to_string());
                            }
                        }
                        KeyCode::Esc => {
                            app.user_input.clear();
                            app.popup_mode = PopupMode::Disabled;
                        }
                        _ => {}
                    },

                    // ====================
                    // Nothing found popup.
                    // ====================
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

                    // ===============
                    // Settings popup.
                    // ===============
                    PopupMode::Settings => match event.code {
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                            terminal.show_cursor()?;
                            break;
                        }
                        KeyCode::Char('s') => app.popup_mode = PopupMode::Disabled,
                        KeyCode::Up | KeyCode::Char('k') => app.app_settings.previous(),
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
