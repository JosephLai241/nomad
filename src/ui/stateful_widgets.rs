//! Stateful widgets for the user interface.

use tui::widgets::{ListState, TableState};

/// A widget that contains its own state.
pub struct StatefulWidget<T, U> {
    /// Items that are contained in this widget.
    pub items: Vec<T>,
    /// The widget's state.
    pub state: U,
    /// The widget's mode.
    pub widget_mode: WidgetMode,
}

/// The modes in which the widget may operate.
pub enum WidgetMode {
    /// The widget is in the files mode (files may be selected from the `App`
    /// based on which index is `selected()`).
    Files,
    /// The widget is in standard mode.
    Standard,
}

impl<T, U> StatefulWidget<T, U> {
    /// Create a new `StatefulList` containing generic `items`.
    pub fn new(items: Vec<T>, state: U, widget_mode: WidgetMode) -> StatefulWidget<T, U> {
        StatefulWidget {
            items,
            state,
            widget_mode,
        }
    }
}

/// Enable round robin behavior for lists containing a `ListState`.
impl<T> StatefulWidget<T, ListState> {
    /// Get the next item in `self.items`.
    pub fn next(&mut self) {
        let limit = match self.widget_mode {
            WidgetMode::Files => 2,
            WidgetMode::Standard => 1,
        };

        self.state.select(Some(match self.state.selected() {
            Some(index) => {
                if index >= self.items.len() - limit {
                    0
                } else {
                    index + 1
                }
            }
            None => 0,
        }));
    }

    /// Get the previous item in `self.items`.
    pub fn previous(&mut self) {
        let limit = match self.widget_mode {
            WidgetMode::Files => 2,
            WidgetMode::Standard => 1,
        };

        self.state.select(Some(match self.state.selected() {
            Some(index) => {
                if index == 0 {
                    self.items.len() - limit
                } else {
                    index - 1
                }
            }
            None => 0,
        }));
    }
}

/// Enable round robin behavior for tables containing a `TableState`.
impl<T> StatefulWidget<T, TableState> {
    /// Get the next item in `self.items`.
    pub fn next(&mut self) {
        self.state.select(Some(match self.state.selected() {
            Some(index) => {
                if index >= self.items.len() - 1 {
                    0
                } else {
                    index + 1
                }
            }
            None => 0,
        }));
    }

    /// Get the previous item in `self.items`.
    pub fn previous(&mut self) {
        self.state.select(Some(match self.state.selected() {
            Some(index) => {
                if index == 0 {
                    self.items.len() - 1
                } else {
                    index - 1
                }
            }
            None => 0,
        }));
    }
}
