     _____ _____ _____ _____ ____
    |   | |     |     |  _  |    \
    | | | |  |  | | | |     |  |  |
    |_|___|_____|_|_|_|__|__|____/

> Explore your filesystem.

# Table of Contents

* [What Is `nomad` and Why?](#what-is-nomad)
* [Features](#features)
* [Usage](#usage)
* [Walkthrough](#walkthrough)
	+ [Default Behavior (Tree View)](#default-behavior-tree-view)
		* [Quick Open/Edit a File](#quick-openedit-a-file)
	+ [Interactive Mode](#interactive-mode)
	+ [Integrated `bat`](#integrated-bat)

# What Is `nomad` and Why?

The original goal of `nomad` was to create an alternative `tree` command, but its scope has widened during development.

`nomad` is primarily an alternative `tree` command, but is ultimately an all-in-one package for exploring your filesystem.

This project is largely inspired by [`lsd`][lsd], an improved `ls` alternative written in Rust.

# Features

`nomad` displays a directory tree by default, but it offers additional functionality such as:

* Quick open/edit of a file wtihin the directory (after running `nomad` in numbered mode (`-n`)).
* Interactive TUI where you can navigate through directories and see a preview of a selected file.
* Built-in [`bat`][bat].

# Usage

```
```

# Walkthrough

## Default Behavior (Tree View)

`nomad`'s default function is to replace the `tree` command. You can display a tree for your current directory just by running `nmd` after `cd`ing into that directory:

```
cd some_directory/
nmd
```

Alternatively, pass in the name of the directory you wish to display a tree:

```
nmd some_directory/
```

### Quick Open/Edit a File

`nomad` provides a quick way to open and edit a file after displaying a directory's tree. First, run `nomad` in numbered mode:

```
mnd -n
```

This will display your directory's contents with numbers next to each file:

```
```

Then pass the file's number with the `-o` flag to open that file with your default text editor:

```
nmd -o 0
```

## Interactive Mode

## Integrated `bat`

<!-- LINKS -->
[bat]: https://github.com/sharkdp/bat
[lsd]: https://github.com/Peltoche/lsd
