# `git status` - `git status` in Tree Form

Use `git status` to see the Git status in tree form. You can run this in lieu of running `nomad` in normal mode to only see tracked files that contain a Git status.

This command pairs well with the other Git commands described below this section.

### Usage

```
USAGE:
    nd git status [FLAGS] [OPTIONS]

FLAGS:
    -L, --all-labels           Label both files and directories. Alias for `-n -l`
    -h, --help                 Prints help information
    -l, --label-directories    Label directories with characters
        --loc                  Display code statistics (lines of code, blanks, and comments) for each item
    -m, --metadata             Show item metadata such as file permissions, owner, group, file size, and last modified time 
        --no-colors            Do not display any colors
        --no-git               Do not display Git status markers
        --no-icons             Do not display icons
    -n, --numbered             Label directory items with numbers
        --plain                Mute icons, Git markers, and colors to display a plain tree
    -s, --stats                Display traversal statistics after the tree is displayed
        --summary              Display `tokei` (lines of code counter) statistics. This only applies if `--loc` is provided 
    -V, --version              Prints version information

OPTIONS:
        --export <export>      Export the tree to a file. Optionally include a target filename
    -p, --pattern <pattern>    Only display items matching this pattern. Supports regex expressions
```

