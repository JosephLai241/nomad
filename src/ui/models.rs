//! Structs used in the UI.

/// Contains keybindings for all modes excluding help mode.
#[derive(Debug)]
pub struct Keybindings<'a> {
    /// The keybindings for breadcrumbs mode.
    pub breadcrumbs: Vec<(&'a str, &'a str)>,
    /// The keybindings for inspect mode.
    pub inspect: Vec<(&'a str, &'a str)>,
    /// The keybindings for normal mode.
    pub normal: Vec<(&'a str, &'a str)>,
}

impl Default for Keybindings<'_> {
    /// Create the `Keybindings` struct with the keybindings for each mode
    /// (breadcrumbs, inspect, and normal modes).
    fn default() -> Self {
        let breadcrumbs = vec![
            (" h, left", "move left in the breadcrumbs"),
            (" l, right", "move right in the breadcrumbs"),
            (" <ESC>", "switch back to normal tree mode"),
            (" <ENTER>", "enter the selected directory"),
        ];
        let inspect = vec![
            (" /", "search for a pattern within the file"),
            (" 0", "scroll to the top of the file"),
            (
                " 1 - 9",
                "scroll 'n' lines down. <SHIFT> + 'n' scrolls 'n' lines up",
            ),
            (" j, down", "scroll down the file"),
            (" k, up", "scroll up the file"),
            (" n", "snap to the next pattern match in the file"),
            (" N", "snap to the previous pattern match in the file"),
            (" R", "refresh the file contents"),
            (" <ESC>", "return to normal tree mode"),
        ];
        let normal = vec![
            (" /", "search for a pattern in file paths"),
            (" 0", "scroll to the top of the tree"),
            (" d", "toggle only displaying directories"),
            (
                " e",
                "edit the selected item in a text editor (if it is a file)",
            ),
            (" g", "toggle Git markers"),
            (" h", "toggle displaying hidden items"),
            (" i", "toggle icons"),
            (" j, down", "scroll down the directory tree"),
            (" k, up", "scroll up the directory tree"),
            (" l", "toggle directory labels"),
            (" m", "toggle displaying item metadata"),
            (" n", "toggle item numbers"),
            (" p", "toggle plain mode"),
            (" r", "refresh the tree with your current settings"),
            (" s", "toggle the settings pane for the current tree"),
            (
                " D",
                "toggle disrespecting all rules specified in ignore-type files",
            ),
            (" L", "toggle all labels (directories and items)"),
            (" R", "reset all current settings and refresh the tree"),
            (" <ESC>", "move to breadcrumbs mode"),
            (
                " <ENTER>",
                "enter the selected directory, or inspect the selected file",
            ),
        ];

        Self {
            breadcrumbs,
            inspect,
            normal,
        }
    }
}
