//! Text constants for TUI help messages.

/// The help text displayed in the help menu after pressing '?'.
pub const HELP_TEXT: &str = r#"
 Press <ESC> to exit this screen.

 Use the directional or Vim directional keys [j, k] to scroll.

 Optionally type a number or its <SHIFT> counterpart to scroll down/up `n` lines,
 ie. '4' to scroll down 4 lines and '$' to scroll up 4 lines.

 -------------------------------------------------------------------------------

 Table of Contents
 =================

 * Widgets
     + Cycling Through Widgets
     + Normal Widget (Tree View)
     + Breadcrumbs Widget
     + Inspect Widget
 * Keybindings
     + Navigation
     + Commands

 -------------------------------------------------------------------------------

 Widgets
 =======

 Rootless mode has 4 widgets:

     * Normal (tree view)
     * Breadcrumbs
     * Inspect (file view)
     * Help (this widget)

 The border of the active widget is colorized and other widgets are dimmed to
 make it obvious which one you are in.

 **TIP**: If you are not sure what keybindings are available for the current
          widget, press 'K' to display a list of available keybindings.

 Cycling Through Widgets
 -----------------------

 <ESC> allows you to exit/cycle through the widgets. Here is a small diagram
 that depicts how <ESC> works.

                   <ESC>            <ESC>
     Breadcrumbs <=======> Normal <======= Inspect

 You can enter the help widget at any time by pressing '?'. <ESC>ing from the
 help widget will always take you back to the normal/tree view.

              <ESC>
     Normal <======= Help

 Normal Widget (Tree View)
 -------------------------

 This is the widget you are in when you enter Rootless mode and is the leftmost
 widget. You can control the appearance of the tree, enter into the directories
 or files, open a file in a text editor, filter results by pattern, etc. See the
 Keybindings section for details.

 Breadcrumbs Widget
 ------------------

 This is the widget at the very top of the TUI. This enables you to enter parent
 directories. You can traverse left or right and press <ENTER> to refresh the
 tree with the contents of the selected directory.

 Inspect Widget
 --------------

 This widget is conditionally rendered based on the current highlighted item in
 the tree. This widget appears on the right side of the TUI if the highlighted
 item is a file and will contain the contents of that file.

 You can scroll up/down and search for patterns in the file while in this widget.
 See the Keybindings section for details.

 -------------------------------------------------------------------------------

 Keybindings
 ===========

 Navigation
 ----------

 Use the directional keys or Vim directional keys [h, j, k, l] to navigate the TUI.

 Commands
 --------

 Listed below are the commands you can use, which widgets support it, and a short
 description of what it does.


 0           Widgets: Normal, Inspect, Help
                 Scroll to the top of the widget

 1 - 9       Widgets: Inspect, Help
                 Scroll `n` lines down. <SHIFT> + `n` scrolls `n` lines up.

 d           Widget: Normal
                 Toggle only displaying directories

 e           Widgets: Normal, Inspect
                 Open the selected file in a text editor (Neovim, Vim, Vi, or Nano)
                 This only applies if the selected item is a file

 g           Widget: Normal
                 Toggle Git status markers

 h           Widget: Normal
                 Toggle displaying hidden items

 i           Widget: Normal
                 Toggle icons

 l           Widget: Normal
                 Toggle directory labels

 m           Widget: Normal
                 Toggle metadata

 n           Widget: Normal
                 Toggle item labels
             Widget: Inspect
                 If a pattern was matched, snap to the next match

 p           Widget: Normal
                 Toggle plain mode

 q           Widgets: all
                 Quit Rootless mode

 r           Widget: Normal
                 Refresh the tree

 s           Widget: Normal
                 Display the settings for the current tree

 D           Widget: Normal
                 Toggle disrespecting all rules specified in ignore-type files

 K           Widget: Normal
                 Display the keybindings available for the widget you are in

 L           Widget: Normal
                 Toggle all labels (directories and items)

 N           Widget: Inspect
                 If a pattern was matched, snap to the previous match

 R           Widget: Normal
                 Reset all options to its default value and refresh the current
                 tree
             Widget: Inspect
                 Refresh the current file

 /           Widgets: Normal, Inspect
                 Search for a pattern. Supports regex expressions and available in
                 the Normal and Inspect widgets

 ?           Widgets: all
                 Display this help message

 <ENTER>     Widget: Normal
                 If a file is selected
                     Enter the Inspect (file view) widget
                 If a directory is selected
                     Enter the selected directory and refresh the tree with the
                     contents of the new directory
             Widget: Breadcrumbs
                 Enter the selected directory and refresh the tree with the
                 contents of the new directory.

 <ESC>       Widget: all
                 Cycle through modes/widgets

 -------------------------------------------------------------------------------

               ________  ________  ________  ________   _______
              ╱    ╱   ╲╱        ╲╱        ╲╱        ╲_╱       ╲
             ╱         ╱         ╱         ╱         ╱         ╱
            ╱         ╱         ╱         ╱         ╱         ╱
            ╲__╱_____╱╲________╱╲__╱__╱__╱╲___╱____╱╲________╱

                               [ ROOTLESS ]
"#;
