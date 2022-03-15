//! Create an application state for the TUI.

use std::{fs::File, io::Read, path::Path};

use tui::{
    style::Color,
    widgets::{ListState, Row, TableState},
};

use super::{
    stateful_widgets::{StatefulWidget, WidgetMode},
    utils::{get_breadcrumbs, get_settings, get_tree},
};
use crate::{cli::Args, errors::NomadError, style::models::NomadStyle, traverse::models::DirItem};

/// Contains the different modes that may be evoked based on user interaction.
///
/// These variants correspond to the different widgets in the UI.
pub enum UIMode {
    /// Move focus to the breadcrumbs at the top of the user interface.
    Breadcrumbs,
    /// Enter the help menu.
    Help,
    /// Move focus to the `cat`ed file and enable vertical and horizontal scrolling.
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
    pub file_contents: Option<Option<Vec<String>>>,
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
    /// Create a new interactive instance with the target directory.
    pub fn new(
        args: &Args,
        nomad_style: &'a NomadStyle,
        target_directory: &str,
    ) -> Result<App<'a>, NomadError> {
        let (tree, items) = get_tree(args, nomad_style, target_directory)?;
        let mut directory_tree = StatefulWidget::new(tree, ListState::default(), WidgetMode::Files);
        let mut directory_items = if args.dirs {
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
                                            .split("\n")
                                            .map(|line| line.to_string())
                                            .collect::<Vec<String>>(),
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
        args: &Args,
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
            get_breadcrumbs(&new_directory)?,
            ListState::default(),
            WidgetMode::Standard,
        );

        let (tree, items) = get_tree(args, nomad_style, &new_directory)?;

        self.directory_tree = StatefulWidget::new(tree, ListState::default(), WidgetMode::Files);
        self.directory_items = if args.dirs {
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

        Ok(())
    }

    /// Refresh the `App` after the user searched for a pattern.
    pub fn pattern_search(
        &mut self,
        args: &mut Args,
        nomad_style: &'a NomadStyle,
        target_directory: &str,
    ) -> Result<(), NomadError> {
        args.pattern = self.collected_input.pop();

        if let Err(error) = self.refresh(args, nomad_style, target_directory) {
            match error {
                NomadError::NothingFound => self.popup_mode = PopupMode::NothingFound,
                _ => {}
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
                        _ if marker == conflicted => self.nomad_style.tui.conflicted_color,
                        _ if marker == deleted => self.nomad_style.tui.deleted_color,
                        _ if marker == modified => self.nomad_style.tui.modified_color,
                        _ if marker == renamed => self.nomad_style.tui.renamed_color,
                        _ if marker == staged_added => self.nomad_style.tui.staged_added_color,
                        _ if marker == staged_deleted => self.nomad_style.tui.staged_deleted_color,
                        _ if marker == staged_modified => {
                            self.nomad_style.tui.staged_modified_color
                        }
                        _ if marker == staged_renamed => self.nomad_style.tui.staged_renamed_color,
                        _ if marker == untracked => self.nomad_style.tui.untracked_color,
                        _ => Color::Reset,
                    },
                    None => Color::Reset,
                },
                None => Color::Reset,
            },
            None => Color::Reset,
        }
    }
}
