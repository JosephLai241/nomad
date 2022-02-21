                               __
      ___  ___  __ _  ___ ____/ /
     / _ \/ _ \/  ' \/ _ `/ _  /
    /_//_/\___/_/_/_/\_,_/\_,_/

> The next gen `tree` command.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

# Table of Contents

* [Introduction](#introduction)
* [Prerequisite Setup](#prerequisite-setup)
	+ [Installing a NerdFont on MacOS](#installing-a-nerdfont-on-macos)
	+ [Installing a NerdFont on Other Systems](#installing-a-nerdfont-on-other-systems)
	+ ["I don't want to install a NerdFont"](#i-dont-want-to-install-a-nerdfont)
* [Usage](#usage)
	+ [Main Usage](#main-usage)
	+ [Filetype Usage](#filetype-usage)
	+ [Git Usage](#git-usage)
	+ [Releases Usage](#releases-usage)
* [Walkthrough](#walkthrough)
	+ [Default Behavior (Stylized Tree View)](#default-behavior-stylized-tree-view)
	+ [Labeled Modes](#labeled-modes)
		* [Unlocked Functionality](#unlocked-functionality)
	+ [Subcommands](#subcommands)
		* [`bat` Integration](#bat-integration)
			+ [Examples](#examples)
		* [Quick Edit/Open a File](#quick-editopen-a-file)
			+ [Examples](#examples-1)
		* [Filetype Filters](#filetype-filters)
			+ [`filetype match`](#filetype-match)
			+ [`filetype negate`](#filetype-negate)
			+ [`filetype options`](#filetype-options)
		* [Git Integration](#git-integration)
			+ [Color Code/Status Marker Key](#color-codestatus-marker-key)
			+ [Available Subcommands](#available-subcommands)
			+ [`git add`](#git-add)
				* [Examples](#examples-2)
			+ [`git commit`](#git-commit)
			+ [`git diff`](#git-diff)
				* [Examples](#examples-3)
			+ [`git status`](#git-status)
		* [View Releases](#view-releases)
			+ [`releases all`](#releases-all)
			+ [`releases info`](#releases-info)
		* [Update `nomad`](#update-nomad)

# Introduction

I often use the `tree` command whenever I am programming. I think it is a very clever tool but unfortunately a bit outdated by today's standards. I wanted to write my own next gen `tree` command, so I did just that.

This project is inspired by [`lsd`][lsd]'s colorization and icons. `lsd` is an improved `ls` alternative written in Rust.

`nomad` is a command-line tool that:

* Can display a stylized tree which integrates icons and colors.
* Git integration
	+ Respect `.gitignore` and rules defined in similar `.ignore`-type files. This is enabled by default.
	+ Display Git status markers next to items within a Git repository. This is enabled by default.
	+ Integrated Git commands
		+ Quickly point Git commands such as `git add/diff/unstage` to a file or a directory (to add/diff/unstage changed files within that directory) in the tree. This allows you to execute those commands without typing out the entire path(s).
		+ `git diff` improved for better readability.
		+ `git status` in tree form.
		+ `git commit <"OPTIONAL_MESSAGE">` integration.
* Filter directory items by filetype(s) or by a pattern.
	+ Filtering by filetype(s)
		* Match or exclude filetype(s) in the directory.
	+ Filtering by a pattern
		* Match or exclude a pattern. Supports regex expressions that are compatible with Rust's [regex][regex] crate.
* Quick edit/open of a file wtihin the directory (after running `nomad` in numbered mode (`-n`)).
* Built-in [`bat`][bat], a `cat` alternative written in Rust, to quickly view a file within the tree.
* Display releases without going on GitHub.
* Can update itself!
* Display file metadata such as file permissions, owner, group, file size, and last modified time.

# Prerequisite Setup

`nomad` requires a [NerdFont][NerdFont] to correctly display the icons. Install a NerdFont before installing `nomad`, otherwise you will be very sad when you see what the tree looks like without a NerdFont.

## Installing a NerdFont on MacOS

Installing a NerdFont on **MacOS** is particularly easy because NerdFonts are available via [Homebrew][Homebrew].

To install the Hack NerdFont, for example, run these commands:

```
brew tap homebrew/cask-fonts
brew install --cask font-hack-nerd-font
```

Then go to your terminal's preferences and set the font to the newly installed NerdFont. If you're using [iTerm2][iTerm2], this is located at:

```
iTerm2 -> Preferences -> Profiles tab -> Text tab -> Font dropdown -> Hack Nerd Font Mono
```

## Installing a NerdFont on Other Systems

Follow the [NerdFont installation guide][NerdFont Installation] for instructions on how to do so for your system.

## "I don't want to install a NerdFont"

If you do not want a crispy looking tree, you can include the `--mute-icons` flag to disable icons.

Including this flag every time you run `nomad` can become cumbersome, so it may be helpful to create an alias for it in your `.bashrc` or equivalent:

```bash
# In `.bashrc`

alias nd="nd --mute-icons"
```

# Usage

## Main Usage

```
USAGE:
    nomad [FLAGS] [OPTIONS] [directory] [SUBCOMMAND]

FLAGS:
        --dirs                 Only display directories
        --disrespect           Disrespect all ignore rules
    -h, --help                 Prints help information
        --hidden               Display hidden files
    -l, --label-directories    Label directories with characters
    -m, --metadata             Show item metadata such as file permissions, owner, group, file size, and last modified time
        --no-git               Do not display Git status markers
        --no-icons             Do not display icons
    -n, --numbered             Show directory contents with numbers
        --plain                Mute icons, Git markers, and colors to display a plain tree
    -s, --stats                Display directory traversal statistics after the tree is displayed
    -V, --version              Prints version information

OPTIONS:
        --export <export>                Export the tree to a file
        --max-depth <max-depth>          Set the maximum depth to recurse
        --max-filesize <max-filesize>    Set the maximum filesize (in bytes) to include in the tree
    -p, --pattern <pattern>              Only display files matching this pattern. Supports regex expressions

ARGS:
    <directory>    Display a tree for this directory

SUBCOMMANDS:
    bat         `bat` (the Rust alternative to the `cat` command) a file. This may be used after running nomad in numbered mode  
    edit        Edit a file with your default $EDITOR or with Neovim, Vim, Vi, or Nano. This may be used after running nomad in  
                numbered mode
    filetype    Filter directory items by filetype
    git         Run commonly used Git commands
    help        Prints this message or the help of the given subcommand(s)
    releases    Retrieve releases for this program (retrieved from GitHub)
    update      Update `nomad`
```

## Filetype Usage

```
USAGE:
    nomad filetype <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    match      Only display files matching this filetype. Enter a single filetype or a list of filetypes delimited by a space. ie.  
               `nd filetype match rust py go vim`
    negate     Do not display files that match this filetype. Enter a single filetype or a list of filetypes delimited by a space.  
               ie. `nd filetype negate c cpp java r`
    options    List the current set of filetype definitions. Optionally search for a filetype. ie. `nd filetype options rust`
```

## Git Usage

```
USAGE:
    nomad git <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add       The `git add` command. This may be used after running nomad in numbered mode or with labeled directories
    commit    The `git commit` command. Optionally include a message after the command, ie. `git commit "YOUR MESSAGE HERE"` The default  
              commit message is "Updating" if no message is included
    diff      The `git diff` command. This may be used after running nomad in numbered mode or with labeled directories
    help      Prints this message or the help of the given subcommand(s)
    status    The `git status` command. Only display changed/unstaged files in the tree
```

## Releases Usage

```
USAGE:
    nomad releases <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    all     List all releases
    help    Prints this message or the help of the given subcommand(s)
    info    Display information for a release version. Optionally search for a release version  
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

## Labeled Modes

Running `nomad` in a labeled mode unlocks its full potential and allows you to perform actions on items in your directory tree very quickly.

There are three labeled modes in which you can run `nomad`:

* `nd -n` - Labels the files.
* `nd -l` - Labels the directories.
* `nd -L` - Labels both directories and files. This is an alias for `nd -n -l`.

### Unlocked Functionality

By using a labeled mode, you gain access to the following flags/commands, which **require a preceding run in a labeled mode prior to use**:

* [`nd bat		<file_number(s)_or_directory_label(s)>`](#bat-integration)
* [`nd edit		<file_number(s)_or_directory_label(s)>`](#quick-editopen-a-file)
* [`nd git add	<file_number(s)_or_directory_label(s)>`](#git-add)

The `git diff` subcommand can take **optional** file number(s) and/or directory label(s). This would also mean running `nomad` in a labeled mode prior to using a command:

* [`nd git diff	<optional_file_number(s)_or_directory_label(s)>`](#git-diff)

# Subcommands

## `bat` Integration

> ***NOTE:*** Requires running in a [labeled mode](#labeled-modes) beforehand.

[`bat`][bat] is a *much* improved `cat` alternative and is integrated into `nomad`.

Quickly `bat` a file by passing the file's number with the `bat` subcommand:

```
nd bat <file_number>
```

#### Examples

```
nd bat 2 10 4	// `bat`s the 2nd, 10th, and 4th files in one go.

nd bat a f h	// `bat`s all files within the directories labeled "a", "f", and "h".

nd bat 2 f 6	// `bat`s the 2nd file, all files within the directory labeled "f", and the 6th file.
```

The following features are integrated into `bat`:

| Feature                           | Description                                                         |
|-----------------------------------|---------------------------------------------------------------------|
| `grid`                            | Paint a grid that separates line numbers, Git changes, and the code |
| `header`                          | Show a header with the file name                                    |
| `line_numbers`                    | Show line numbers                                                   |
| `paging_mode` - `QuitIfOneScreen` | Use a pager if the output exceeds the current terminal's length     |
| `true_color`                      | Output 24-bit colors                                                |
| `vcs_modification_markers`        | Show markers for VCS changes.                                       |
| `wrapping_mode` - `Character`     | Text wrapping is enabled                                            |

## Quick Edit/Open a File

> ***NOTE:*** Requires running in a [labeled mode](#labeled-modes) mode beforehand.

`nomad` provides a quick way to open and edit a file after displaying a directory's tree.

It will try to open the file with your default editor by extracting the value from the `$EDITOR` environment variable. If you do not have a default editor set, the following editors are supported and will be tried in this order:

1. [Neovim][Neovim]
2. [Vim][Vim]
3. [Vi][Vi]
4. [Nano][Nano]

Quickly edit/open a file by passing the file's number with the `edit` subcommand:

```
nd edit <file_number>
```

#### Examples

```
nd edit 2 10 4	// Open/edit the 2nd, 10th, and 4th files in one go.

nd edit a f h	// Open/edit all files within the directories labeled "a", "f", and "h".

nd edit 2 f 6	// Open/edit the 2nd file, all files within the directory labeled "f", and the 6th file.  
```

## Filetype Filters

You can filter directory items by filetype(s) by using the `filetype` subcommand followed by what you would like to do.

### `filetype match`

If the `filetype` subcommand is followed by `match`, the tree will only display directory items that match the filetype. For example, if you only want to display Rust filetypes, you would run:

```
nd filetype match rust
```

You can specify multiple filetypes delimited by a space. For example, to only display Rust, Python, Go, and Vim filetypes in the tree, you would run:

```
nd filetype match rust py go vim
```

### `filetype negate`

If the `filetype` subcommand is followed by `negate`, the tree will ignore directory items that match the filetype. For example, if you want to exclude C filetypes, you would run:

```
nd filetype negate c
```

Like `match`, you can specify multiple filetypes delimited by a space. For example, to exclude C, C++, Java, and R filetypes, you would run:

```
nd filetype negate c cpp java r
```

### `filetype options`

If you want to see all the filetype globs, use `options` to display a list of filetypes and their corresponding globs:

```
nd filetype options
```

You can optionally search for a specific filetype. Simply include the filetype after `options`. For example, if you want to search for globs corresponding to Rust files, you would run:

```
nd filetype options rust
```

## Git Integration

`nomad` respects rules specified in `.ignore`-type files by default, which includes `.gitignore`s.

Git status markers appear next to any changed item within a Git repository that is present in the tree. In other words, any nested Git repositories will receive this treatment.

### Color Code/Status Marker Key

Here are some keys detailing the colors/Git markers that may appear in the tree.

Changes in the **working directory** are stylized like so:

| Marker | Description | Color  |
|--------|-------------|--------|
| `D`    | Deleted     | Red    |
| `M`    | Modified    | Yellow |
| `U`    | Untracked   | Green  |
| `R`    | Renamed     | Orange |

Changes that are **staged** (after running `nd git add`) are stylized like so:

| Marker | Description | Color  |
|--------|-------------|--------|
| `SD`   | Deleted     | Red    |
| `SM`   | Modified    | Yellow |
| `SU`   | Untracked   | Green  |
| `SR`   | Renamed     | Orange |

Filenames will also be stylized if they are staged:

| Status        | Style              |
|---------------|--------------------|
| Deleted       | Red, strikethrough |
| Modified      | Yellow             |
| Untracked/New | Green              |
| Renamed       | Orange             |
| Conflicting   | Red                |

**Conflicts** are stylized like so:

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

> ***NOTE:*** Requires running in a [labeled mode](#labeled-modes) mode beforehand.

Quickly run a `git add` for a file without the burden of typing out the entire file path. Pass in the number that corresponds with the file you want to stage.

```
nd git add 12    // Stages the 12th file in the tree.
```

#### Examples

```
nd git add 2 10 4	// Stages the 2nd, 10th, and 4th files in one go.

nd git add a f h	// Stages all files within the directories labeled "a", "f", and "h".

nd git add 2 f 6	// Stages the 2nd file, all files within the directory labeled "f", and the 6th file.  
```

### `git commit`

You can run `git commit` via `nomad`:

```
nd git commit
```

You can optionally include a commit message following the command like so:

```
nd git commit "Your commit message here"
```

The default commit message is `"Updating"` if no commit message is provided.

### `git diff`

> ***NOTE:*** Requires running in a [labeled mode](#labeled-modes) mode beforehand.

Quickly run a `git diff` for a file without the burden of typing out the entire file path.

This command will use the built-in `bat` to display the target file's diff.

```
nd git diff 2    // View the diff for the 2nd file in the tree.
```

#### Examples

```
nd git diff 2 10 4	// View the diff for the 2nd, 10th, and 4th files in one go.

nd git diff a f h	// View the diff for all files within the directories labeled "a", "f", and "h".

nd git diff 2 f 6	// View the diff for the 2nd file, all files within the directory labeled "f", and the 6th file.  
```

### `git status`

This subcommand restricts the tree to only display files that have been modified. Think `git status` in tree form.

## View Releases

You can view releases for `nomad` directory from `nomad` itself. This can be achieved by using the `releases` subcommand.

### `releases all`

You can view all releases by using `all`.

```
nd releases all
```

This will display a table containing the following information for each release:

* Release name
* Version number
* Release date
* Description
* \*Assets

\* Assets is a nested table that includes the following information:

* Asset name
* Download URL

### `releases info`

`info` behaves similarly to `all`. It will display the information for your current version/release in a table with the same attributes as `all`.

```
nd releases info
```

You can optionally search for a specific version number by including it after `info`:

```
nd releases info 1.0.0
```

## Update `nomad`

**`nomad` can update itself**. Simply run `nd update` and you are all set! No hassle.

<!-- LINKS -->
[bat]: https://github.com/sharkdp/bat
[Homebrew]: https://brew.sh/
[iTerm2]: https://iterm2.com/
[lsd]: https://github.com/Peltoche/lsd
[Nano]: https://www.nano-editor.org/
[Neovim]: https://github.com/neovim/neovim
[NerdFont]: https://www.nerdfonts.com/
[NerdFont Installation]: https://github.com/ryanoasis/nerd-fonts#font-installation
[regex]: https://docs.rs/regex/latest/regex/
[Vi]: http://ex-vi.sourceforge.net/
[Vim]: https://www.vim.org/
