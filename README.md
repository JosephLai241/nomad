     _____ _____ _____ _____ ____
    |   | |     |     |  _  |    \
    | | | |  |  | | | |     |  |  |
    |_|___|_____|_|_|_|__|__|____/

> The next gen `tree` command.

# Table of Contents

<!--TODO: FIX LINKS TO "AVAILABLE FLAGS"-->
<!--TODO: FIX LINKS TO "AVAILABLE FLAGS"-->

* [What Is `nomad` and Why?](#what-is-nomad-and-why)
* [Features](#features)
* [Prerequisite Setup](#prerequisite-setup)
* [Usage](#usage)
	+ [Main Usage](#main-usage)
	+ [Git Usage](#git-usage)
* [Walkthrough](#walkthrough)
	+ [Default Behavior (Stylized Tree View)](#default-behavior-stylized-tree-view)
	+ [Numbered Mode](#numbered-mode)
		* [Available Flags](#available-flags)
	+ [Git Integration](#git-integration)
		* [Color Code/Status Marker Key](#color-codestatus-marker-key)
		* [Available Subcommands](#available-subcommands)
		* [`git add`](#git-add)
		* [`git commit`](#git-commit)
		* [`git diff`](#git-diff)
		* [`git status`](#git-status)
	* [Quick Open/Edit a File](#quick-openedit-a-file)
	+ [Interactive Mode](#interactive-mode)
	+ [`bat` Integration](#bat-integration)
	+ [Display Metadata and Traversal Statistics](#display-metadata-and-traversal-statistics)
		* [Available Flags](#available-flags)

<!--TODO: FIX LINKS TO "AVAILABLE FLAGS"-->
<!--TODO: FIX LINKS TO "AVAILABLE FLAGS"-->

# What Is `nomad` and Why?

I often use the `tree` command whenever I am programming. I think it is a very clever tool but unfortunately a bit outdated by today's standards. I wanted to write my own new-gen `tree` command, so I did just that.

This project is inspired by [`lsd`][lsd]'s colorization and icons. `lsd` is an improved `ls` alternative written in Rust.

# Features

At its core, `nomad` is, again, an alternative/upgraded `tree` command with the following differences:

* Displays a stylized tree which integrates icons and colors.
* Git integration
	+ Respect `.gitignore` and rules defined in similar `.ignore`-type files. This is enabled by default.
	+ Display Git status markers next to items within a Git repository. This is enabled by default.
	+ Integrated Git commands
		+ Quickly point Git commands such as `git add/diff` to a file within the tree. This allows you to add files or view diffs without needing to type out the entire path to the file.
		+ `git status` in tree form.
* Display file metadata such as file permissions, owner, group, file size, and last modified time.
* Quick open/edit of a file wtihin the directory (after running `nomad` in numbered mode (`-n`)).
* Built-in [`bat`][bat], a `cat` alternative written in Rust, to quickly view a file within the tree.
<!--* Interactive TUI where you can navigate through directories and see a preview of a selected file.-->

# Prerequisite Setup

`nomad` requires a [NerdFont][NerdFont] to correctly display the icons. Install a NerdFont before installing `nomad`, otherwise you will be very sad when you see what the tree looks like without a NerdFont.

Follow the [NerdFont installation guide][NerdFont Installation] for instructions on how to do so for your system.

# Usage

## Main Usage

```
USAGE:
    nd [FLAGS] [OPTIONS] [directory] [SUBCOMMAND]

FLAGS:
        --disrespect     Disrespect all ignore rules
    -h, --help           Prints help information
        --hidden         Display hidden files
    -i, --interactive    Initialize an interactive file/directory explorer
    -m, --metadata       Show item metadata such as file permissions, owner, group, file size, and last modified time  
    -n, --numbered       Show directory contents with numbers
    -s, --stats          Display directory traversal statistics after the tree is displayed
    -V, --version        Prints version information

OPTIONS:
    -b, --bat <bat>          `bat` (the Rust alternative to the `cat` command) a file
        --export <export>    Export the tree to a file instead of displaying
    -o, --open <open>        Open a file based on its index within the tree
                             This may be used after running `nomad` in numbered mode (`-n`)

ARGS:
    <directory>    Explore this directory

SUBCOMMANDS:
    git     Run commonly used Git commands
    help    Prints this message or the help of the given subcommand(s)
```

## Git Usage

```
USAGE:
    nomad git <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add       Equivalent to the `git add` command
    commit    Equivalent to the `git commit` command. Optionally include a message after the command, ie. `git commit "YOUR MESSAGE HERE"`  
              The default commit message is "Updating" if no message is included
    diff      Equivalent to the `git diff` command
    help      Prints this message or the help of the given subcommand(s)
    status    Equivalent to the `git status` command. Only display changed/unstaged files in the tree
```

# Walkthrough

## Default Behavior (Stylized Tree View)

You can display a tree for your current directory just by running `nd` after `cd`ing into that directory:

```
cd some_directory/
nd
```

Alternatively, pass in the name of the directory you wish to target:

```
nd some_directory/
```

## Numbered Mode

**Any options that require a target file argument requires `nomad` to be run in numbered mode prior to using that command. You can do this by including the `-n` flag.**

The options that require numbered mode before running are:

```
-b, --bat <file_number>
-o, --open <file_number>

git add <file_number(s)>
git diff <file_number>
```

## Git Integration

`nomad` respects rules specified in `.ignore`-type files by default, which includes `.gitignore`s.

Git status markers appear next to any changed item within a Git repository that is present in the tree. In other words, any nested Git repositories will receive this treatment.

### Color Code/Status Marker Key

Here are some keys detailing the colors/Git markers that may appear in the tree.

Changes in the **working directory** are marked/colorized as such:

| Marker | Description | Color  |
|--------|-------------|--------|
| `D`    | Deleted     | Red    |
| `M`    | Modified    | Yellow |
| `U`    | Untracked   | Green  |
| `R`    | Renamed     | Orange |

Changes that are **staged** (after running `nd git add`) are marked/colorized as such:

| Marker | Description | Color  |
|--------|-------------|--------|
| `SD`   | Deleted     | Red    |
| `SM`   | Modified    | Yellow |
| `SU`   | Untracked   | Green  |
| `SR`   | Renamed     | Orange |

**Conflicts** are marked/colorized as such:

| Marker     | Description      | Color  |
|------------|------------------|--------|
| `CONFLICT` | Conflicting file | Red    |

### Available Subcommands

```
git
    add
    commit
    diff
    status
```

### `git add`

> ***NOTE:*** Requires running in numbered mode beforehand.

Quickly run a `git add` for a file without the burden of typing out the entire file path. Pass in the number that corresponds with the file you want to stage.

```
nd git add 12    // Stages the 12th file in the tree.
```

You can stage multiple items at once by delimiting numbers with a space like so:

```
nd git add 3 5 12 16    // Stages the 3rd, 5th, 12th, and 16th file in the tree.
```

If you run `nomad` again, the files that are staged will be colorized depending on its Git status.

### `git commit`

You can run `git commit` via `nomad` by prepending `nd`:

```
nd git commit
```

You can optionally include a commit message following the command like so:

```
nd git commit "Your commit message here"
```

The default commit message is `"Updating"` if no commit message is provided.

### `git diff`

> ***NOTE:*** Requires running in numbered mode beforehand.

Quickly run a `git diff` for a file without the burden of typing out the entire file path.

This command will use the built-in `bat` to display the target file's diff.

```
nd git diff 2    // View the diff for the 2nd file in the tree.
```

### `git status`

This subcommand restricts the tree to only display files that have been modified. Think `git status` in tree form.

## Quick Open/Edit a File

> ***NOTE:*** Requires running in numbered mode beforehand.

`nomad` provides a quick way to open and edit a file after displaying a directory's tree.

It will try to open the file with your default editor by extracting the value from the `$EDITOR` environment variable. If you do not have a default editor set, the following editors are supported and will be tried in this order:

1. [Neovim][Neovim]
2. [Vim][Vim]
3. [Vi][Vi]
4. [Nano][Nano]

Pass the file's number with the `-o` flag to open that file with your default text editor:

```
nd -o <file_number>
```

## Interactive Mode

## `bat` Integration

> ***NOTE:*** Requires running in numbered mode beforehand.

[`bat`][bat] is a *much* improved `cat` alternative and is integrated into `nomad`.

Quickly `bat` a file by passing the file's number with the `-b` flag:

```
nd -b <file_number>
```

The following features are integrated into `bat`:

| Feature                           | Description
|-----------------------------------|-------------------------------
| `grid`                            | Paint a grid that separates line numbers, Git changes, and the code
| `header`                          | Show a header with the file name
| `line_numbers`                    | Show line numbers
| `paging_mode` - `QuitIfOneScreen` | Use a pager if the output exceeds the current terminal's length
| `true_color`                      | Output 24-bit colors
| `vcs_modification_markers`        | Show markers for VCS changes.
| `wrapping_mode` - `Character`     | Text wrapping is enabled

## Display Metadata and Traversal Statistics

### Available Flags

```
-m
-s
```

To display file metadata such as file permissions, owner, group, file size, and last modified time by including the `-m` flag.

To display traversal statistics such as the number of directories and files in the tree, as well as the time it took to traverse the directory, include the `-s` flag.

<!-- LINKS -->
[bat]: https://github.com/sharkdp/bat
[lsd]: https://github.com/Peltoche/lsd
[Nano]: https://www.nano-editor.org/
[NerdFont]: https://www.nerdfonts.com/
[NerdFont Installation]: https://github.com/ryanoasis/nerd-fonts#font-installation
[Neovim]: https://github.com/neovim/neovim
[Vi]: http://ex-vi.sourceforge.net/
[Vim]: https://www.vim.org/
