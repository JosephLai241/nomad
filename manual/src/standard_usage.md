# Standard Usage

You can display a tree for your current directory just by running `nd` after `cd`ing into that directory.

```
cd some_directory/
nd
```

Alternatively, pass in the name of the directory you wish to target:

```
nd some_directory/
```

`nomad` will assign an icon to each file within the tree based on its respective filetype and search for Git repositories in the target directory. If Git repositories are detected, Git markers will show next to the item indicating its Git status.

Git markers are shown regardless of whether you are running `nomad` in a directory containing a Git repository. For example, let's say you have a directory named `rust/` containing multiple Rust projects that are tracked by Git and are stored in sub-directories. Many of these projects contain changes that you have not committed. Running `nomad` on the `rust/` directory will display the Git changes for all of these projects. This provides a convenient overview for which projects have changes that you need to commit/revisit.

See the [Git Status Markers](./git/status_markers.md) section for more details.

## Flags

Here are the flags you can use in standard mode:

```
FLAGS:
    -L, --all-labels           Label both files and directories. Alias for `-n -l`
        --banner               Display the banner
        --dirs                 Only display directories
        --disrespect           Disrespect all ignore rules
    -h, --help                 Prints help information
        --hidden               Display hidden files
    -l, --label-directories    Label directories with characters
        --loc                  Display code statistics (lines of code, blanks, and comments) for each item
    -m, --metadata             Show item metadata such as file permissions, owner, group, file size, and last modified time 
        --no-colors            Do not display any colors
        --no-git               Do not display Git status markers
        --no-icons             Do not display icons
    -n, --numbered             Label directory items with numbers
        --plain                Mute icons, Git markers, and colors to display a plain tree
    -s, --stats                Display traversal statistics after the tree is displayed
    -V, --version              Prints version information

OPTIONS:
        --export <export>                Export the tree to a file. Optionally include a target filename
        --max-depth <max-depth>          Set the maximum depth to recurse
        --max-filesize <max-filesize>    Set the maximum filesize (in bytes) to include in the tree
    -p, --pattern <pattern>              Only display items matching this pattern. Supports regex expressions
```
