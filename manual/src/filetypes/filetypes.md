# `ft` - Filtering Items by Filetype or Glob

The `ft` subcommand allows you to filter results by filetypes or globs. This subcommand contains additional subcommands that allow you to control whether to match (include) or negate (ignore) filetypes or globs.

## Usage

```
Filter directory items by filetype

USAGE:
    nd ft <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    match      Only display files matching the specified filetypes and/or globs
    negate     Do not display files that match the specified filetypes and/or globs
    options    List the current set of filetype definitions. Optionally search for a filetype. ie. `nd filetype options rust` 
```
