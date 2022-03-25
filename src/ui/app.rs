//! Create an application state for the TUI.

use std::{fs::File, io::Read, path::Path};

use anyhow::Result;
use regex::{Match, Regex};
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Cell, ListState, Row, TableState},
};

use super::{
    models::Keybindings,
    stateful_widgets::{StatefulWidget, WidgetMode},
    utils::{get_breadcrumbs, get_settings, get_tree},
};
use crate::{
    cli::global::GlobalArgs, errors::NomadError, style::models::NomadStyle,
    traverse::models::DirItem,
};

/// Contains the different modes that may be evoked based on user interaction.
///
/// These variants correspond to the different widgets in the UI.
pub enum UIMode {
    /// Move focus to the breadcrumbs at the top of the user interface.
    Breadcrumbs,
    /// Enter the help menu.
    Help,
    /// Move focus to the `cat`ed file and enable scrolling and pattern searching.
    Inspect,
    /// Normal mode.
    Normal,
}

/// Contains the different popup modes that may be evoked based on user interaction.
pub enum PopupMode {
    /// No popup is rendered.
    Disabled,
    /// Render a popup with the error message if applicable.
    Error(String),
    /// Nothing was found after a pattern was provided.
    NothingFound,
    /// Render a popup that accepts a pattern.
    PatternInput,
    /// Render the settings menu as a popup.
    Settings,
    /// Show the keybindings for a particular mode.
    ShowKeybindings,
}

/// Contains the UI's current state.
pub struct App<'a> {
    /// Hold each `Row` of settings displayed in the settings popup.
    pub app_settings: StatefulWidget<Row<'a>, TableState>,
    /// Hold a `BreadcrumbState` for UI navigation.
    pub breadcrumbs: StatefulWidget<String, ListState>,
    /// Collected user input.
    pub collected_input: Vec<String>,
    /// The current directory of the tree that is displayed.
    pub current_directory: String,
    /// All items in the target directory.
    pub directory_items: Option<StatefulWidget<DirItem, ListState>>,
    /// The directory tree.
    pub directory_tree: StatefulWidget<String, ListState>,
    /// Stores `None` or `Some(file contents)`.
    pub file_contents: Option<Option<Vec<Spans<'a>>>>,
    /// Stores the keybindings for each mode (breadcrumbs, inspect, and normal mode).
    pub keybindings: Keybindings<'a>,
    /// Stores the current set of available keybindings.
    pub keybindings_for_mode: StatefulWidget<Row<'a>, TableState>,
    /// Stores the line numbers where regex matches occur.
    pub match_lines: StatefulWidget<u16, ListState>,
    /// Store the `NomadStyle` struct.
    pub nomad_style: &'a NomadStyle,
    /// Hold the current popup mode.
    pub popup_mode: PopupMode,
    /// Hold the scroll position for `Scroll` mode.
    pub scroll: u16,
    /// Hold the current UI mode.
    pub ui_mode: UIMode,
    /// Hold the user input for popup prompts.
    pub user_input: String,
}

impl<'a> App<'a> {
    /// Create a new Rootless instance with the target directory.
    pub fn new(
        args: &GlobalArgs,
        nomad_style: &'a NomadStyle,
        target_directory: &str,
    ) -> Result<App<'a>, NomadError> {
        let (tree, items) = get_tree(args, nomad_style, target_directory)?;
        let mut directory_tree = StatefulWidget::new(tree, ListState::default(), WidgetMode::Files);
        let mut directory_items = if args.modifiers.dirs {
            None
        } else {
            Some(StatefulWidget::new(
                match items {
                    Some(paths) => paths,
                    None => Vec::new(),
                },
                ListState::default(),
                WidgetMode::Files,
            ))
        };

        directory_tree.state.select(Some(0));

        if let Some(ref mut directory_items) = directory_items {
            directory_items.state.select(Some(0));
        }

        let keybindings = Keybindings::default();
        let normal_keybindings = &keybindings
            .normal
            .iter()
            .map(|(keybinding, description)| {
                Row::new(vec![
                    Cell::from(keybinding.to_string()),
                    Cell::from(description.to_string()),
                ])
            })
            .collect::<Vec<Row>>();

        Ok(App {
            app_settings: StatefulWidget::new(
                get_settings(args),
                TableState::default(),
                WidgetMode::Standard,
            ),
            breadcrumbs: StatefulWidget::new(
                get_breadcrumbs(target_directory)?,
                ListState::default(),
                WidgetMode::Standard,
            ),
            collected_input: Vec::new(),
            current_directory: Path::new(target_directory)
                .to_str()
                .unwrap_or("?")
                .to_string(),
            directory_items,
            directory_tree,
            file_contents: None,
            keybindings,
            keybindings_for_mode: StatefulWidget::new(
                normal_keybindings.to_vec(),
                TableState::default(),
                WidgetMode::Standard,
            ),
            match_lines: StatefulWidget::new(vec![], ListState::default(), WidgetMode::Standard),
            nomad_style,
            popup_mode: PopupMode::Disabled,
            scroll: 0,
            ui_mode: UIMode::Normal,
            user_input: String::new(),
        })
    }

    /// Check if the selected item is a directory.
    /// Returns `true` if it is, returns `false` if it is a file.
    pub fn selected_is_dir(&self) -> Result<Option<bool>, NomadError> {
        match self.directory_tree.state.selected() {
            Some(index) => match &self.directory_items {
                Some(directory_items) => {
                    let selected_item = &directory_items.items[index];

                    if Path::new(&selected_item.path).canonicalize()?.is_dir() {
                        Ok(Some(true))
                    } else {
                        Ok(Some(false))
                    }
                }
                None => Ok(Some(true)), // `args.dirs` is `true` if `self.directory_items` is `None`.
            },
            None => Ok(None),
        }
    }

    /// `cat` the selected item if it is a file.
    pub fn cat_file(&mut self) -> Result<(), NomadError> {
        self.match_lines = StatefulWidget::new(vec![], ListState::default(), WidgetMode::Standard);
        self.match_lines.state.select(Some(0));

        match self.selected_is_dir()? {
            Some(is_dir) => {
                if is_dir {
                    self.file_contents = None;
                } else {
                    match self.directory_tree.state.selected() {
                        Some(index) => match &self.directory_items {
                            Some(directory_items) => {
                                let mut buffer = Vec::new();
                                let mut file = File::open(&directory_items.items[index].path)?;

                                file.read_to_end(&mut buffer)?;

                                self.file_contents = if buffer.is_empty() {
                                    Some(None)
                                } else {
                                    Some(Some(
                                        String::from_utf8_lossy(&buffer)
                                            .split('\n')
                                            .map(|line| Spans::from(Span::from(line.to_string())))
                                            .collect::<Vec<Spans>>(),
                                    ))
                                }
                            }
                            None => self.file_contents = None, // `args.dirs` is `true` if `self.directory_items` is `None`.
                        },
                        None => self.file_contents = None,
                    }
                }
            }
            None => self.file_contents = None,
        }

        Ok(())
    }

    /// Search the current file for a pattern and colorize matches accordingly.
    /// Returns a newly formatted `cat`ed file.
    ///
    /// Dim lines do not contain any matches whereas bright lines contain a match
    /// somewhere. This was implemented to make it easier to tell where the matches are.
    pub fn search_in_file(&mut self) -> Result<(), NomadError> {
        if let Some(Some(file_spans)) = &self.file_contents {
            if let Some(input) = self.collected_input.pop() {
                match Regex::new(&input) {
                    Ok(regex) => {
                        let mut collected_spans: Vec<Spans> = Vec::new();
                        let mut matched_lines: Vec<u16> = Vec::new();

                        for (index, spans) in file_spans.iter().enumerate() {
                            for span in spans.0.iter() {
                                let spanned_line = &mut span
                                    .content
                                    .to_string()
                                    .chars()
                                    .map(|character| Span::from(character.to_string()))
                                    .collect::<Vec<Span>>();
                                let matches =
                                    regex.find_iter(&span.content).collect::<Vec<Match>>();

                                if !matches.is_empty() {
                                    for matched in matches {
                                        for i in matched.start()..matched.end() {
                                            spanned_line[i] = Span::styled(
                                                spanned_line[i].content.to_string(),
                                                Style::default()
                                                    .add_modifier(Modifier::BOLD)
                                                    .fg(self.nomad_style.tui.regex.match_color),
                                            );
                                        }
                                        matched_lines.push(index as u16);
                                    }

                                    collected_spans.push(Spans::from(spanned_line.clone()));
                                } else {
                                    let dimmed_line = spanned_line
                                        .iter()
                                        .map(|span| {
                                            Span::styled(
                                                span.content.to_string(),
                                                Style::default().add_modifier(Modifier::DIM),
                                            )
                                        })
                                        .collect::<Vec<Span>>();

                                    collected_spans.push(Spans::from(dimmed_line.clone()));
                                }
                            }
                        }

                        if !matched_lines.is_empty() {
                            self.file_contents = Some(Some(collected_spans));
                            self.match_lines = StatefulWidget::new(
                                matched_lines,
                                ListState::default(),
                                WidgetMode::Standard,
                            );
                            self.match_lines.state.select(Some(0));
                            self.scroll = self.match_lines.items
                                [self.match_lines.state.selected().unwrap_or(0)];

                            self.popup_mode = PopupMode::Disabled;
                        } else {
                            self.popup_mode = PopupMode::NothingFound;
                        }
                    }
                    Err(error) => self.popup_mode = PopupMode::Error(error.to_string()),
                }
            }
        }

        Ok(())
    }

    /// Get the path to the current file.
    pub fn get_current_file(&self) -> Result<String, NomadError> {
        match self.selected_is_dir()? {
            Some(is_dir) => {
                if is_dir {
                    Err(NomadError::NotAFile)
                } else {
                    match self.directory_tree.state.selected() {
                        Some(index) => match &self.directory_items {
                            Some(directory_items) => Ok(directory_items.items[index].path.clone()),
                            None => Err(NomadError::NoItems),
                        },
                        None => Err(NomadError::NothingSelected),
                    }
                }
            }
            None => Err(NomadError::NothingSelected),
        }
    }

    /// Get the current directory from the breadcrumbs.
    fn get_target_by_breadcrumbs(&self) -> String {
        let end = match self.breadcrumbs.state.selected() {
            Some(index) => index + 1,
            None => self.breadcrumbs.items.len(),
        };

        format!("/{}", self.breadcrumbs.items[..end].join("/"))
    }

    /// Refresh the tree and update the app's `breadcrumbs`, `directory_items`,
    /// and `directory_tree`.
    pub fn refresh(
        &mut self,
        args: &mut GlobalArgs,
        nomad_style: &'a NomadStyle,
        target_directory: &str,
    ) -> Result<(), NomadError> {
        self.nomad_style = nomad_style;
        self.app_settings = StatefulWidget::new(
            get_settings(args),
            TableState::default(),
            WidgetMode::Standard,
        );
        self.file_contents = None;
        self.scroll = 0;

        let crumb_path = self.get_target_by_breadcrumbs();
        let current_directory_clone = self.current_directory.clone();

        let new_directory = match self.ui_mode {
            UIMode::Breadcrumbs => target_directory,
            UIMode::Normal => match &self.directory_items {
                Some(directory_items) => match self.selected_is_dir()? {
                    Some(is_dir) => {
                        if is_dir {
                            match self.popup_mode {
                                PopupMode::PatternInput => &current_directory_clone,
                                _ => match self.directory_tree.state.selected() {
                                    Some(index) => &directory_items.items[index].path,
                                    None => target_directory,
                                },
                            }
                        } else {
                            &crumb_path
                        }
                    }
                    None => &crumb_path,
                },
                None => &crumb_path,
            },
            _ => &crumb_path,
        };

        self.current_directory = Path::new(new_directory).to_str().unwrap_or("?").to_string();

        self.breadcrumbs = StatefulWidget::new(
            get_breadcrumbs(new_directory)?,
            ListState::default(),
            WidgetMode::Standard,
        );

        let (tree, items) = get_tree(args, nomad_style, new_directory)?;

        self.directory_tree = StatefulWidget::new(tree, ListState::default(), WidgetMode::Files);
        self.directory_items = if args.modifiers.dirs {
            None
        } else {
            Some(StatefulWidget::new(
                match items {
                    Some(paths) => paths,
                    None => Vec::new(),
                },
                ListState::default(),
                WidgetMode::Files,
            ))
        };

        self.directory_tree.state.select(Some(0));
        if let Some(ref mut directory_items) = self.directory_items {
            directory_items.state.select(Some(0));
        }

        self.popup_mode = PopupMode::Disabled;
        self.ui_mode = UIMode::Normal;

        args.regex.pattern = None;

        Ok(())
    }

    /// Refresh the `App` after the user searched for a pattern.
    pub fn pattern_search(
        &mut self,
        args: &mut GlobalArgs,
        nomad_style: &'a NomadStyle,
        target_directory: &str,
    ) -> Result<(), NomadError> {
        args.regex.pattern = self.collected_input.pop();

        if let Err(error) = self.refresh(args, nomad_style, target_directory) {
            if let NomadError::NothingFound = error {
                self.popup_mode = PopupMode::NothingFound;
            }
        } else {
            self.popup_mode = PopupMode::Disabled;
        }

        Ok(())
    }

    /// Return the color of the highlighted item if it contains Git changes.
    pub fn get_git_color(&self) -> Color {
        let conflicted = &self.nomad_style.git.conflicted_marker;
        let deleted = &self.nomad_style.git.deleted_marker;
        let modified = &self.nomad_style.git.modified_marker;
        let renamed = &self.nomad_style.git.renamed_marker;
        let staged_added = &self.nomad_style.git.staged_added_marker;
        let staged_deleted = &self.nomad_style.git.staged_deleted_marker;
        let staged_modified = &self.nomad_style.git.staged_modified_marker;
        let staged_renamed = &self.nomad_style.git.staged_renamed_marker;
        let untracked = &self.nomad_style.git.untracked_marker;

        match self.directory_tree.state.selected() {
            Some(index) => match &self.directory_items {
                Some(directory_items) => match &directory_items.items[index].marker {
                    Some(marker) => match marker.to_string() {
                        _ if marker == conflicted => self.nomad_style.tui.git.conflicted_color,
                        _ if marker == deleted => self.nomad_style.tui.git.deleted_color,
                        _ if marker == modified => self.nomad_style.tui.git.modified_color,
                        _ if marker == renamed => self.nomad_style.tui.git.renamed_color,
                        _ if marker == staged_added => self.nomad_style.tui.git.staged_added_color,
                        _ if marker == staged_deleted => {
                            self.nomad_style.tui.git.staged_deleted_color
                        }
                        _ if marker == staged_modified => {
                            self.nomad_style.tui.git.staged_modified_color
                        }
                        _ if marker == staged_renamed => {
                            self.nomad_style.tui.git.staged_renamed_color
                        }
                        _ if marker == untracked => self.nomad_style.tui.git.untracked_color,
                        _ => Color::Reset,
                    },
                    None => Color::Reset,
                },
                None => Color::Reset,
            },
            None => Color::Reset,
        }
    }

    /// Update the available keybindings for the current mode.
    pub fn update_keybindings(&mut self) {
        let empty_vec = vec![];

        let table_items = match &self.ui_mode {
            UIMode::Breadcrumbs => &self.keybindings.breadcrumbs,
            UIMode::Inspect => &self.keybindings.inspect,
            UIMode::Normal => &self.keybindings.normal,
            _ => &empty_vec,
        };

        let keybindings_rows = table_items
            .iter()
            .map(|(keybinding, description)| {
                Row::new(vec![
                    Cell::from(keybinding.to_string()).style(Style::default().fg(Color::Gray)),
                    Cell::from(description.to_string()).style(Style::default().fg(Color::Gray)),
                ])
            })
            .collect::<Vec<Row>>();

        self.keybindings_for_mode = StatefulWidget::new(
            keybindings_rows,
            TableState::default(),
            WidgetMode::Standard,
        );
    }
}
