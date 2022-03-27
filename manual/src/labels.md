# Labeled Modes

Running `nomad` in a labeled mode unlocks its full potential and allows you to perform actions on items in the tree very quickly.

The flags that enable labels are:

* `-l` - labels directories
* `-n` - labels items
* `-L` - labels both directories and items. This is an alias for `-l -n`

Directories are labeled with a letter and items are labeled by numbers. Numbers may be appended to directory labels if there are more than 26 directories in the tree.
