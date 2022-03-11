//! Building layouts for the UI.

use tui::layout::{Constraint, Direction, Layout, Rect};

/// Create a centered `Rect` for input popups.
pub fn get_single_line_popup_area(frame: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Percentage(8),
            Constraint::Percentage(45),
        ])
        .split(frame);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(popup_layout[1])[1]
}

/// Create a centered `Rect` for error popups.
pub fn get_error_popup_area(frame: Rect) -> Rect {
    let error_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
        ])
        .split(frame);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(error_layout[1])[1]
}

/// Create a centered popup area to display the current settings.
pub fn get_settings_area(frame: Rect) -> Rect {
    let settings_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(frame);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(settings_layout[1])[1]
}
