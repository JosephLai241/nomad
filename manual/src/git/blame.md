# `git blame`

> **NOTE**: Requires a preceeding run in a [labeled mode](../labels.md).

You can use the `git blame` subcommand to quickly blame a file. **This subcommand only accepts one file, so directory labels will not work!**

`nomad`'s `git blame` offers some visual improvements over the original `git blame` command:

* Commit hashes, authors, and timestamps are colorized differently to provide contrast among the columns
* Lines are colorized based on a unique color assigned to each author
    + These colors are randomly chosen and will be different each time you run `git blame`. Commits made by you are plain white whereas commits by other authors are assigned a color.

### Usage

```
USAGE:
    nd git blame [FLAGS] [OPTIONS] <file-number>

FLAGS:
        --emails     Display emails for each blame line instead of timestamps
    -h, -help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --lines <lines>...    Restrict a range of lines to display in the blame 

ARGS:
    <file-number>    Display a blame for this file
```
