//! Text constants for TUI help messages.

/// The help text displayed in the help menu after pressing '?'.
pub const HELP_TEXT: &str = r#"
 Use the directional or Vim directional keys [j, k] to scroll.
 Press <ESC> to exit this screen.

 Table of Contents
 =================

 - Keybindings
 - Modes


 Views
 =====

 Rootless mode has 4 views:

     * Normal/breadcrumbs
     *

 Keybindings
 ===========

 Navigation
 ----------

 Use the directional keys or Vim directional keys [h, j, k, l] to navigate the TUI.

 Commands
 --------

 0           In general, scroll to the top of the widget you are in

 1 - 9       Scroll `n` lines/items down. `shift` + `n` scrolls `n` lines/items up.

 d           Toggle only displaying directories

 e           Open the selected file in a text editor (Neovim, Vim, Vi, or Nano)
             This only applies if the selected item is a file

 g           Toggle Git markers

 h           Toggle displaying hidden items

 i           Toggle icons

 l           Toggle directory labels

 m           Toggle metadata

 n           Toggle item labels

 p           Toggle plain mode

 q           Quit rootless mode

 r           Refresh the tree

 s           Display the settings for the current tree

 x           Export the tree to a file. Optionally provide a filename

 D           Toggle disrespecting all rules specified in ignore-type files

 L           Toggle all labels (directories and items)

 R           Reset all options to its default value and refresh the current tree

 /           Search for a pattern. Supports regex expressions

 ?           Display this help message

 <ENTER>     If a file is selected       Enter scroll mode
             If a directory is selected  Enter the selected directory and redraw
                                         the tree
 <ESC>       Cycle through modes/widgets


 Modes
 =====

 This UI has four modes:

 * Breadcrumbs   Move focus to the breadcrumbs at the top of the TUI and
                 select a different directory to inspect
 * Normal        This is the default mode when Rootless mode is instantiated
 * Scroll        This mode may be entered if the currently highlighted item is a file

"#;
