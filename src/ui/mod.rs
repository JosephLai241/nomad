//! The user interface for rootless (interactive) mode.

pub mod app;
pub mod interface;
pub mod layouts;
pub mod models;
pub mod stateful_widgets;
pub mod text;
pub mod utils;
pub mod widgets;

use self::{
    app::{App, PopupMode, UIMode},
    interface::render_ui,
    text::HELP_TEXT,
    utils::reset_args,
};
use crate::{cli::global::GlobalArgs, errors::NomadError, style::models::NomadStyle};

use anyhow::Result;
use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
    },
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use std::io::stdout;

/// Variants for post-exit tasks.
pub enum ExitMode {
    /// Exit the UI without any post-exit tasks.
    Clean,
    /// Exit the UI and edit a specified file.
    Edit(Vec<String>),
}

/// Enter `nomad`'s rootless (interactive) mode.
pub fn enter_rootless_mode(
    args: &mut GlobalArgs,
    nomad_style: &NomadStyle,
    target_directory: &str,
) -> Result<ExitMode, NomadError> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, SetTitle("nomad  |  rootless"))?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(args, nomad_style, target_directory)?;

    let exit_mode = enter_event_loop(app, args, nomad_style, target_directory, &mut terminal)?;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(exit_mode)
}

/// Enter the rootless mode event loop.
fn enter_event_loop<'a, B>(
    mut app: App<'a>,
    args: &mut GlobalArgs,
    nomad_style: &'a NomadStyle,
    target_directory: &str,
    terminal: &mut Terminal<B>,
) -> Result<ExitMode, NomadError>
where
    B: Backend,
{
    let max_help_scroll = HELP_TEXT.as_bytes().iter().filter(|&&c| c == b'\n').count();
    let mut exit_mode = ExitMode::Clean;

    loop {
        terminal.draw(|frame| render_ui(&mut app, args, frame))?;

        // Handle keyboard events.
        if let Event::Key(key) = read()? {
            match app.popup_mode {
                // ============
                // Normal mode.
                // ============
                PopupMode::Disabled => {
                    match key.code {
                        // =============
                        // TUI commands.
                        // =============

                        // Enter search/pattern match mode.
                        KeyCode::Char('/') => match app.ui_mode {
                            UIMode::Inspect => {
                                if let Err(error) = app.cat_file() {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }

                                app.popup_mode = PopupMode::PatternInput;
                            }
                            UIMode::Normal => app.popup_mode = PopupMode::PatternInput,
                            _ => {}
                        },
                        // In Normal mode, toggle only showing directories.
                        KeyCode::Char('d') => {
                            if let UIMode::Normal = app.ui_mode {
                                args.modifiers.dirs = !args.modifiers.dirs;
                                if let Err(error) = app.refresh(args, nomad_style, target_directory)
                                {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }
                            }
                        }
                        KeyCode::Char('e') => {
                            if let UIMode::Normal = app.ui_mode {
                                match app.get_current_file() {
                                    Ok(file_path) => {
                                        exit_mode = ExitMode::Edit(vec![file_path]);
                                    }
                                    Err(error) => {
                                        app.popup_mode = PopupMode::Error(error.to_string())
                                    }
                                }
                            }
                        }
                        // In Normal mode, toggle Git markers.
                        KeyCode::Char('g') => {
                            if let UIMode::Normal = app.ui_mode {
                                args.style.no_git = !args.style.no_git;
                                if let Err(error) = app.refresh(args, nomad_style, target_directory)
                                {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }
                            }
                        }
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
                                args.modifiers.hidden = !args.modifiers.hidden;
                                if let Err(error) = app.refresh(args, nomad_style, target_directory)
                                {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }
                            }
                            _ => {}
                        },
                        // In Normal mode, toggle icons.
                        KeyCode::Char('i') => {
                            if let UIMode::Normal = app.ui_mode {
                                args.style.no_icons = !args.style.no_icons;
                                if let Err(error) = app.refresh(args, nomad_style, target_directory)
                                {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }
                            }
                        }
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
                                args.labels.label_directories = !args.labels.label_directories;
                                if let Err(error) = app.refresh(args, nomad_style, target_directory)
                                {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }
                            }
                            _ => {}
                        },
                        // In Normal mode, toggle showing metadata for all items.
                        KeyCode::Char('m') => {
                            if let UIMode::Normal = app.ui_mode {
                                args.meta.metadata = !args.meta.metadata;
                                if let Err(error) = app.refresh(args, nomad_style, target_directory)
                                {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }
                            }
                        }
                        // In Normal mode, toggle numbered items.
                        KeyCode::Char('n') => match app.ui_mode {
                            UIMode::Inspect => {
                                app.match_lines.next();

                                if let Some(index) = app.match_lines.state.selected() {
                                    app.scroll = app.match_lines.items[index] as u16;
                                }
                            }
                            UIMode::Normal => {
                                args.labels.numbers = !args.labels.numbers;
                                if let Err(error) = app.refresh(args, nomad_style, target_directory)
                                {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }
                            }
                            _ => {}
                        },
                        // In Normal mode, toggle plain mode.
                        KeyCode::Char('p') => {
                            if let UIMode::Normal = app.ui_mode {
                                args.style.plain = !args.style.plain;
                                if let Err(error) = app.refresh(args, nomad_style, target_directory)
                                {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }
                            }
                        }
                        // Quit Rootless mode.
                        KeyCode::Char('q') => {
                            break;
                        }
                        // Reload the tree.
                        KeyCode::Char('r') => {
                            if let UIMode::Normal = app.ui_mode {
                                if let Err(error) = app.refresh(args, nomad_style, target_directory)
                                {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }
                            }
                        }
                        // In Normal mode, display all settings.
                        KeyCode::Char('s') => {
                            if let UIMode::Normal = app.ui_mode {
                                app.popup_mode = PopupMode::Settings
                            }
                        }
                        // In Normal mode, disrespect all `.ignore` rules.
                        KeyCode::Char('D') => {
                            if let UIMode::Normal = app.ui_mode {
                                args.modifiers.disrespect = !args.modifiers.disrespect;
                                if let Err(error) = app.refresh(args, nomad_style, target_directory)
                                {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }
                            }
                        }
                        // Show keybindings for a mode.
                        KeyCode::Char('K') => match app.ui_mode {
                            UIMode::Help => {}
                            _ => {
                                app.update_keybindings();
                                app.popup_mode = PopupMode::ShowKeybindings;
                            }
                        },
                        // In Normal mode, toggle applying all labels.
                        KeyCode::Char('L') => {
                            if let UIMode::Normal = app.ui_mode {
                                args.labels.all_labels = !args.labels.all_labels;
                                if let Err(error) = app.refresh(args, nomad_style, target_directory)
                                {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }
                            }
                        }
                        KeyCode::Char('N') => {
                            if let UIMode::Inspect = app.ui_mode {
                                app.match_lines.previous();

                                if let Some(index) = app.match_lines.state.selected() {
                                    app.scroll = app.match_lines.items[index] as u16;
                                }
                            }
                        }
                        // Reset all arguments.
                        KeyCode::Char('R') => match app.ui_mode {
                            UIMode::Inspect => {
                                if let Err(error) = app.cat_file() {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }
                            }
                            UIMode::Normal => {
                                reset_args(args);
                                if let Err(error) = app.refresh(args, nomad_style, target_directory)
                                {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }
                            }
                            _ => {}
                        },
                        // Enter help mode/display the help message.
                        KeyCode::Char('?') => match app.ui_mode {
                            UIMode::Help => {}
                            _ => {
                                app.scroll = 0;
                                app.ui_mode = UIMode::Help;
                            }
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
                                app.scroll = 0;
                                app.ui_mode = UIMode::Normal;
                            }
                            UIMode::Normal => app.ui_mode = UIMode::Breadcrumbs,
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
                                match app.refresh(
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
                                    ),
                                ) {
                                    Ok(_) => app.ui_mode = UIMode::Normal,
                                    Err(error) => {
                                        app.popup_mode = PopupMode::Error(error.to_string())
                                    }
                                }
                            }
                            UIMode::Normal => match app.selected_is_dir() {
                                Ok(optional_bool) => {
                                    if let Some(is_dir) = optional_bool {
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
                                }
                                Err(error) => app.popup_mode = PopupMode::Error(error.to_string()),
                            },
                            _ => {}
                        },

                        // ===========
                        // Navigation.
                        // ===========

                        // Cycle through the breadcrumbs or lateral scrolling when
                        // inspecting a file.
                        KeyCode::Left => {
                            if let UIMode::Breadcrumbs = app.ui_mode {
                                if app.breadcrumbs.state.selected().is_none() {
                                    app.breadcrumbs
                                        .state
                                        .select(Some(app.breadcrumbs.items.len() - 1));
                                }
                            }
                        }
                        // Cycle through the breadcrumbs or lateral scrolling when
                        // inspecting a file.
                        KeyCode::Right => {
                            if let UIMode::Breadcrumbs = app.ui_mode {
                                if app.breadcrumbs.state.selected().is_none() {
                                    app.breadcrumbs
                                        .state
                                        .select(Some(app.breadcrumbs.items.len() - 1));
                                }
                            }
                        }
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
                        KeyCode::Char('1') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => app.scroll += 1,
                            _ => {}
                        },
                        KeyCode::Char('!') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => {
                                if app.scroll != 0 {
                                    app.scroll -= 1
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Char('2') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => app.scroll += 2,
                            _ => {}
                        },
                        KeyCode::Char('@') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => {
                                if app.scroll < 2 {
                                    app.scroll = 0
                                } else if app.scroll != 0 {
                                    app.scroll -= 2
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Char('3') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => app.scroll += 3,
                            _ => {}
                        },
                        KeyCode::Char('#') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => {
                                if app.scroll < 3 {
                                    app.scroll = 0
                                } else if app.scroll != 0 {
                                    app.scroll -= 3
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Char('4') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => app.scroll += 4,
                            _ => {}
                        },
                        KeyCode::Char('$') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => {
                                if app.scroll < 4 {
                                    app.scroll = 0
                                } else if app.scroll != 0 {
                                    app.scroll -= 4
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Char('5') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => app.scroll += 5,
                            _ => {}
                        },
                        KeyCode::Char('%') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => {
                                if app.scroll < 5 {
                                    app.scroll = 0
                                } else if app.scroll != 0 {
                                    app.scroll -= 5
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Char('6') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => app.scroll += 6,
                            _ => {}
                        },
                        KeyCode::Char('^') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => {
                                if app.scroll < 6 {
                                    app.scroll = 0
                                } else if app.scroll != 0 {
                                    app.scroll -= 6
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Char('7') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => app.scroll += 7,
                            _ => {}
                        },
                        KeyCode::Char('&') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => {
                                if app.scroll < 7 {
                                    app.scroll = 0
                                } else if app.scroll != 0 {
                                    app.scroll -= 7
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Char('8') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => app.scroll += 8,
                            _ => {}
                        },
                        KeyCode::Char('*') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => {
                                if app.scroll < 8 {
                                    app.scroll = 0
                                } else if app.scroll != 0 {
                                    app.scroll -= 8
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Char('9') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => app.scroll += 9,
                            _ => {}
                        },
                        KeyCode::Char('(') => match app.ui_mode {
                            UIMode::Help | UIMode::Inspect => {
                                if app.scroll < 9 {
                                    app.scroll = 0
                                } else if app.scroll != 0 {
                                    app.scroll -= 9
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
                PopupMode::Error(_) => {
                    reset_args(args);

                    if let Err(error) = app.refresh(args, nomad_style, target_directory) {
                        app.popup_mode = PopupMode::Error(error.to_string());
                    }
                }

                // ===========
                // Input mode.
                // ===========
                PopupMode::PatternInput => match key.code {
                    KeyCode::Backspace => {
                        app.user_input.pop();
                    }
                    KeyCode::Char(ch) => {
                        app.user_input.push(ch);
                    }
                    KeyCode::Enter => {
                        app.scroll = 0;
                        app.collected_input.push(app.user_input.drain(..).collect());

                        match app.ui_mode {
                            UIMode::Inspect => {
                                if let Err(error) = app.search_in_file() {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }
                            }
                            UIMode::Normal => {
                                if let Err(error) =
                                    app.pattern_search(args, nomad_style, target_directory)
                                {
                                    app.popup_mode = PopupMode::Error(error.to_string());
                                }
                            }
                            _ => {}
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
                PopupMode::NothingFound => match key.code {
                    KeyCode::Char('/') => app.popup_mode = PopupMode::PatternInput,
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => app.popup_mode = PopupMode::Disabled,
                },

                // ===============
                // Settings popup.
                // ===============
                PopupMode::Settings => match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('s') | KeyCode::Esc => app.popup_mode = PopupMode::Disabled,
                    KeyCode::Up | KeyCode::Char('k') => app.app_settings.previous(),
                    KeyCode::Down | KeyCode::Char('j') => app.app_settings.next(),
                    _ => {}
                },

                // ==================
                // Keybindings popup.
                // ==================
                PopupMode::ShowKeybindings => match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('K') | KeyCode::Esc => app.popup_mode = PopupMode::Disabled,
                    KeyCode::Up | KeyCode::Char('k') => app.keybindings_for_mode.previous(),
                    KeyCode::Down | KeyCode::Char('j') => app.keybindings_for_mode.next(),
                    _ => {}
                },
            }
        }
    }

    Ok(exit_mode)
}
