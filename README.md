     _____ _____ _____ _____ ____
    |   | |     |     |  _  |    \
    | | | |  |  | | | |     |  |  |
    |_|___|_____|_|_|_|__|__|____/

> Explore your filesystem.

# Table of Contents

<!--TODO: FIX LINKS TO "AVAILABLE FLAGS"-->
<!--TODO: FIX LINKS TO "AVAILABLE FLAGS"-->

* [What Is `nomad` and Why?](#what-is-nomad-and-why)
* [Features](#features)
* [Prerequisite Setup](#prerequisite-setup)
* [Usage](#usage)
* [Walkthrough](#walkthrough)
	+ [Default Behavior (Stylized Tree View)](#default-behavior-stylized-tree-view)
	+ [Numbered Mode](#numbered-mode)
		* [Available Flags](#available-flags)
	* [Quick Open/Edit a File](#quick-openedit-a-file)
	+ [Interactive Mode](#interactive-mode)
	+ [Integrated `bat`](#integrated-bat)
	+ [Display Metadata and Traversal Statistics](#display-metadata-and-traversal-statistics)
		* [Available Flags](#available-flags)

<!--TODO: FIX LINKS TO "AVAILABLE FLAGS"-->
<!--TODO: FIX LINKS TO "AVAILABLE FLAGS"-->

# What Is `nomad` and Why?

I often use the `tree` command whenever I am programming. I think it is a very clever tool but unfortunately a bit outdated by today's standards. I wanted to write my own new-gen `tree` command, so I did just that.

This project is largely inspired by [`lsd`][lsd]'s colorization and icons. `lsd` is an improved `ls` alternative written in Rust.

# Features

`nomad` displays a stylized directory tree which integrates icons and colors by default, but it offers additional functionality such as:

* Git integration
	+ Display Git status markers next to items within a Git repository. This is also enabled by default.
	+ Quickly point Git commands such as `git add/diff` to a file within the tree.
	+ Restrict the tree to only display files that have been modified. Think `git status` in tree form.
* Display file metadata such as file permissions, owner, group, file size, and last modified time.
* Quick open/edit of a file wtihin the directory (after running `nomad` in numbered mode (`-n`)).
* Built-in [`bat`][bat], a `cat` alternative written in Rust, to quickly view a file within the tree.
<!--* Interactive TUI where you can navigate through directories and see a preview of a selected file.-->

# Prerequisite Setup

`nomad` requires a [NerdFont][NerdFont] to correctly display the icons. Install a NerdFont before installing `nomad`, otherwise you will be very sad when you see what the tree looks like without a NerdFont.

Follow the [NerdFont installation guide][NerdFont Installation] for instructions on how to do so.

# Usage

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

# Walkthrough

**TIP:** Any options that require a target file requires `nomad` to be run in numbered mode prior to using that command. You can do this by including the `-n` flag.

The options that require numbered mode before running are:

```
-b, --bat <file_number>
-o, --open <file_number>

git add <file_number>
git diff <file_number>
```

## Default Behavior (Stylized Tree View)

You can display a tree for your current directory just by running `nd` after `cd`ing into that directory:

```
cd some_directory/
nd
```

Alternatively, pass in the name of the directory you wish to display a tree:

```
nd some_directory/
```

## Numbered Mode

### Available Flags

```
-n
```

## Quick Open/Edit a File

`nomad` provides a quick way to open and edit a file after displaying a directory's tree.

It will try to open the file with your default editor by extracting the value from the `$EDITOR` environment variable. If you do not have a default editor set, the following editors are supported and will be tried in this order:

1. [Neovim][Neovim]
2. [Vim][Vim]
3. [Vi][Vi]
4. [Nano][Nano]

Run `nomad` in numbered mode, then pass the file's number with the `-o` flag to open that file with your default text editor:

```
nd -o <file_number>
```

## Interactive Mode

## Integrated `bat`

[`bat`][bat] is a *much* improved `cat` alternative and is integrated into `nomad`.

Run `nomad` in numbered mode, then pass the file's number with the `-b` flag to `bat` a file:

```
nd -b <file_number>
```

The following features are integrated into `bat`:

| Feature                           | Description
|-----------------------------------|-------------------------------
| `grid`					        | Paint a grid that separates line numbers, Git changes, and the code
| `header`					        | Show a header with the file name
| `line_numbers`			        | Show line numbers
| `paging_mode` - `QuitIfOneScreen` | Use a pager if the output exceeds the current terminal's length
| `true_color`  			        | Output 24-bit colors
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
