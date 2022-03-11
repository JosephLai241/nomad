//! Stateful widgets for the user interface.

use tui::widgets::{ListState, TableState};

/// A widget that contains its own state.
pub struct StatefulWidget<T, U> {
    pub items: Vec<T>,
    pub state: U,
}

impl<T, U> StatefulWidget<T, U> {
    /// Create a new `StatefulList` containing generic `items`.
    pub fn new(items: Vec<T>, state: U) -> StatefulWidget<T, U> {
        StatefulWidget { items, state }
    }
}

/// Enable round robin behavior for lists containing a `ListState`.
impl<T> StatefulWidget<T, ListState> {
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
