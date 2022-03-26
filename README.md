       ________  ________  ________  ________   _______ 
      ╱    ╱   ╲╱        ╲╱        ╲╱        ╲_╱       ╲
     ╱         ╱         ╱         ╱         ╱         ╱
    ╱         ╱         ╱         ╱         ╱         ╱ 
    ╲__╱_____╱╲________╱╲__╱__╱__╱╲___╱____╱╲________╱  

> The customizable next gen `tree` command with Git integration and TUI.

![Rust](https://img.shields.io/badge/Rust-black?style=flat-square&logo=rust)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/JosephLai241/nomad/Rust?style=flat-square&logo=github)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/JosephLai241/nomad?style=flat-square)
![Lines of code](https://img.shields.io/tokei/lines/github/JosephLai241/nomad?style=flat-square)
![License](https://img.shields.io/github/license/JosephLai241/nomad?style=flat-square)

# Table of Contents

* [Introduction](#introduction)
* [What `nomad` Has to Offer](#what-nomad-has-to-offer)
* [Prerequisite Setup](#prerequisite-setup)
    + [Installing a NerdFont on MacOS](#installing-a-nerdfont-on-macos)
	+ [Installing a NerdFont on Linux](#installing-a-nerdfont-on-linux)
	+ [Installing a NerdFont on Windows](#installing-a-nerdfont-on-windows)
	+ ["I don't want to install a NerdFont"](#i-dont-want-to-install-a-nerdfont)
* [Standard Usage](#standard-usage)
    + [Flags](#flags)
* [Labeled Modes](#labeled-modes)
    + [Unlocked Functionality](#unlocked-functionality)
* [`bat` - `bat` Files](#bat---bat-files)
* [`edit` - Edit Files](#edit---edit-files)
* [`tokei` - Display Code Statistics](#tokei---display-code-statistics)
* [`ft` - Filtering Items by Filetype or Glob](#ft---filtering-items-by-filetype-or-glob)
    + [Usage](#usage)
	+ [Matching Filetypes or Globs](#matching-filetypes-or-globs)
	+ [Negating Fiietypes or Globs](#negating-filetypes-or-globs)
	+ [Listing Filetype Definitions](#listing-filetype-definitions)
* [`git` - Run `Git` Commands](#git---run-git-commands)
    + [`git status` - `git status` in Tree Form](#git-status---git-status-in-tree-form)
	    * [Usage](#usage-1)
		* [Git Status Markers](#git-status-markers)
	+ [`git add`](#git-add)
	+ [`git blame`](#git-blame)
	    * [Usage](#usage-2)
	+ [`git branch`](#git-branch)
	    * [Usage](#usage-3)
	+ [`git commit`](#git-commit)
	+ [`git diff`](#git-diff)
* [Customizing `nomad`](#customizing-nomad)
    + ["What Can I Customize?"](#what-can-i-customize)
	+ [`config edit` - Edit the Configuration File](#config-edit---edit-the-configuration-file)
	+ [`config preview` - Preview Your Settings](#config-preview---preview-your-settings)
* [`releases` - View `nomad` Releases](#releases---view-nomad-releases)
    + [`releases all` - View All Releases](#releases-all---view-all-releases)
    + [`releases VERSION_NUMBER` - View Release Data for a Specific Version](#releases-version_number-view-release-data-for-a-specific-version)
* [Upgrading `nomad`](#upgrading-nomad)

# Introduction

I think the `tree` command is a useful terminal utility but is unfortunately a bit short on some features.

`nomad` is a modern, customizable `tree` alternative that aims to expand upon the concept by simplifying the way you interact with files and directories within your terminal.

# What `nomad` Has to Offer

When used without extra options, `nomad` will display a stylized tree for a directory while respecting any rules that are specified in any ignore-type files such as `.gitignore`s (this can be disabled).

There are a ton of features baked into `nomad`. Here is a list of all it has to offer (so far):

* Display tree items with NerdFont-supported icons and Git status markers regardless of whether you run `nomad` in a directory containing a Git repository.
* Filter (including or excluding) directory items by filetype or glob patterns.
* Interact with directory items without typing paths.
    + [`bat`][bat], an improved `cat` alternative written in Rust, or open file(s) in the tree.
	+ Quickly `git diff/add/restore/blame` files.
* Integrated Git
    + In addition to `git diff/add/restore/blame`, `git status` and `git branch` are implemented in tree form.
	* Visually improved `git diff` and `git blame`.
* [Rootless mode](#rootless-mode) -- `nomad` in a TUI (terminal UI) with file preview and pattern searching capability.
* Integrated [`Tokei`][Tokei], a program that displays statistics about your code such as the number of files, total lines, comments, and blank lines.
* Convenient configuration editing and preview.
* It's self-upgrading and you can see the available releases all from your terminal!

Since there are so many features, I have included a link that points back to the table of contents at the end of each section for your convenience.

# Prerequisite Setup

**`nomad` requires a [NerdFont][NerdFont] to correctly display the icons.** Install a NerdFont before installing `nomad`, otherwise you will be very sad when you see what the tree looks like without a NerdFont.

I will provide instructions to install the Hack NerdFont on your system. If you want to use another font, follow the [NerdFont installation guide][NerdFont Installation] for instructions on how to do so for your system.

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

## Installing a NerdFont on Linux

## Installing a NerdFont on Windows

## "I don't want to install a NerdFont"

If you do not want a crispy looking tree, you can include the `--mute-icons` flag to disable icons.

Including this flag every time you run `nomad` can become cumbersome, so it may be helpful to create an alias for it in your `.bashrc` or equivalent:

```bash
# In `.bashrc`

alias nd="nd --mute-icons"
```

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

See the [Git Status Markers](#git-status-markers) section for more details.

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
        --summary              Display `tokei` (lines of code counter) statistics. This only applies if `--loc` is provided 
    -V, --version              Prints version information

OPTIONS:
        --export <export>                Export the tree to a file. Optionally include a target filename
        --max-depth <max-depth>          Set the maximum depth to recurse
        --max-filesize <max-filesize>    Set the maximum filesize (in bytes) to include in the tree
    -p, --pattern <pattern>              Only display items matching this pattern. Supports regex expressions
```

[Back to Table of Contents](#table-of-contents)

# Labeled Modes

Running `nomad` in a labeled mode unlocks its full potential and allows you to perform actions on items in the tree very quickly.

The flags that enable labels are:

* `-l` - labels directories
* `-n` - labels items
* `-L` - labels both directories and items. This is an alias for `-l -n`

Directories are labeled with a letter and items are labeled by numbers. Numbers may be appended to directory labels if there are more than 26 directories in the tree.

## Unlocked Functionality

By using a labeled mode, you gain access to the following subcommands:

* `bat` - `bat` file(s)
* `edit` - open file(s) in a text editor
* `git GIT_SUBCOMMAND` - execute `git` `add`, `blame`, `diff`, and `restore` for file(s)

**All of these subcommands will accept item and/or directory labels.** See their respective sections for more details.

[Back to Table of Contents](#table-of-contents)

# `bat` - `bat` Files

> **NOTE**: Requires a preceeding run in a [labeled mode](#labeled-modes).

You can quickly `bat` files by using the `bat` subcommand. As mentioned before, `bat` is a modern `cat` alternative that supports syntax highlighting and displays Git change markers like you would see in your text editor, to name two of its features.

For example, if you wanted to `bat` the 2nd and 5th file as well as all files in the directory labeled "b", you would run the following command:

```
nd bat 2 5 b
```

Files will be `bat`ed in the order of the labels you provide.

[Back to Table of Contents](#table-of-contents)

# `edit` - Edit Files

> **NOTE**: Requires a preceeding run in a [labeled mode](#labeled-modes).

You can quickly open files in a text editor by using the `edit` subcommand.

`nomad` will try to find your default `$EDITOR` setting and open files with that editor if you have specified one. If your `$EDITOR` is not set, it will try to open files with the following text editor in this order:

1. Neovim
2. Vim
3. Vi
4. Nano

The first editor that is found on your machine will be used to edit your files.

For example, if you wanted to open the 2nd and 5th file as well as all files in the directory labeled "b", you would run the following command:

```
nd edit 2 5 b
```

[Back to Table of Contents](#table-of-contents)

# `tokei` - Display Code Statistics

`Tokei` is a code statistics tool written in Rust. This tool displays data such as the number of files, total lines, comments, and blank lines present in your current directory.

`Tokei` is available as its own stand-alone CLI tool, but you can also access its default behavior by using the `tokei` subcommand.

```
nd tokei
```

[Back to Table of Contents](#table-of-contents)

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

## Matching Filetypes or Globs

You can include filetypes or globs by using the `match` subcommand. Use the `-f` flag to include filetypes and the `-g` flag to match globs.

For example, if you only wanted to include Rust files and files that match the glob `*.asdf`, you would run:

```
nd ft match -f rust -g "*.asdf"
```

Both the `-f` and `-g` flags accept a list of filetypes or globs. An example:

```
nd ft match -f rust toml py -g "*.asdf" "*.qwerty"
```

## Negating Fiietypes or Globs

You can exclude filetypes or globs by using the `negate` subcommand. Use the `-f` flag to include filetypes and the `-g` flag to match globs.

For example, if you only wanted to negate C++ files and files that match the glob `*.asdf`, you would run:

```
nd ft match -f cpp -g "*.asdf"
```

Both the `-f` and `-g` flags accept a list of filetypes or globs. An example:

```
nd ft match -f c cpp java -g "*.asdf" "*.qwerty"
```

## Listing Filetype Definitions

If you want to see the built-in filetype globs, use the `options` subcommand to see a table containing the filetype and the globs for that particular filetype. You can also search for a filetype to only display those matching globs.

For example, if you wanted to see the globs for Rust files, you would run:

```
nd ft options rust
```

[Back to Table of Contents](#table-of-contents)

# `git` - Run `Git` Commands

> **NOTE**: Most Git commands require a preceeding run in a [labeled mode](#labeled-modes).

In addition to respecting ignore-type files such as `.gitignore`s and display Git status markers, `nomad` integrates the following Git commands:

* `git add`
* `git blame`
* `git branch`
* `git commit`
* `git diff`
* `git restore`
* `git status`

**These commands are not meant to be a full replacement for the original Git commands yet!** Git is a *massive* ecosystem, so I have not yet implemented these commands to support everything the original `git` counterpart is capable of. `nomad`'s `git` commands work well if you need to do something basic with those Git commands, however.

## `git status` - `git status` in Tree Form

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

### Git Status Markers

Here is a table that contains the default Git status markers, the marker's color, and what it represents:

| Marker | Color    | Status              |
|--------|----------|---------------------|
| `!`    | Red      | Conflicting         |
| `D`    | Red      | Deleted             |
| `M`    | Orange   | Modified            |
| `R`    | Orange   | Renamed             |
| `TC`   | Purple   | Type change         |
| `SA`   | \*Green  | Staged, Added       |
| `SD`   | \*Red    | Staged, Deleted     |
| `SM`   | \*Orange | Staged, Modified    |
| `SR`   | \*Orange | Staged, Renamed     |
| `STC`  | \*Purple | Staged, type change |
| `U`    | Gray     | Untracked           |

> \* The filename will also be painted the same color.

[Back to Table of Contents](#table-of-contents)

If you do not like the default marker or color configuration, you can [customize it to your liking](#customizing-nomad)

## `git add`

> **NOTE**: Requires a preceeding run in a [labeled mode](#labeled-modes).

You can use the `git add` subcommand to quickly stage files. For example, if you wanted to stage the 2nd and 5th file as well as all files containing a Git status in the directory labeled "b", you would run the following command:

```
nd git add 2 5 b
```

If you pass a directory label, only items containing a Git status will be staged, just like the original `git add` command.

## `git blame`

> **NOTE**: Requires a preceeding run in a [labeled mode](#labeled-modes).

You can use the `git blame` subcommand to quickly blame a file. **This subcommand only accepts one file, so directory labels will not work!**

`nomad`'s `git blame` offers some visual improvements over the original `git blame` command. Commit hashes, authors, and timestamps are colorized differently to provide contrast among the columns. Additionally, lines are colorized based on a unique color assigned to each author. These colors are randomly chosen and will be different each time you run `git blame`. Commits by you are plain white whereas commits by other authors are assigned a color.

### Usage

```
USAGE:
    nd git blame [FLAGS] [OPTIONS] <file-number>

FLAGS:
        --emails     Display emails for each blame line instead of timestamps
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --lines <lines>...    Restrict a range of lines to display in the blame 

ARGS:
    <file-number>    Display a blame for this file
```

[Back to Table of Contents](#table-of-contents)

## `git branch`

You can use the `git branch` subcommand to display all your branches in a tree form (this may be disabled, see the [Usage](#usage-3) section).

`nomad`'s `git branch` displays additional data by default, such as:

* Whether a branch is `HEAD`
* Whether an upstream branch is set

> Unfortunately I have not implemented `git checkout` for v1.0.0, but I plan on implementing it in the next version for quick branch switching.

### Usage

```
USAGE:
    nd git branch [FLAGS] [OPTIONS]

FLAGS:
    -f, --flat          Display branches in a normal list
    -h, --help          Prints help information
        --no-icons      Do not display icons
    -n, --numbered      Label branches with numbers
    -s, --statistics    Display the total number of branches
    -V, --version       Prints version information

OPTIONS:
        --export <export>      Export the tree to a file. Optionally include a target filename
    -p, --pattern <pattern>    Only display branches matching this pattern. Supports regex expressions
```

[Back to Table of Contents](#table-of-contents)

## `git commit`

`git commit` has also been implemented and offers some visual improvements over the original `git commit` command.

[Back to Table of Contents](#table-of-contents)

## `git diff`

> **NOTE**: Requires a preceeding run in a [labeled mode](#labeled-modes).

You can use the `git diff` command to quickly display the diff for files. For example, if you wanted to diff the 2nd and 5th file as well as all files containing a Git status in the directory labeled "b", you would run the following command:

```
nd git diff 2 5 b
```

`nomad`'s `git diff` offers visual improvements over the original `git diff` command by making it much easier to read, in my opinion. See for yourself:

<!-- DO A SIDE BY SIDE COMPARISON OF THE ORIGINAL AND NOMAD'S GIT DIFF HERE -->

If you pass a directory label, only items containing a Git status will be diffed, just like the original `git diff` command.

[Back to Table of Contents](#table-of-contents)

## `git restore`

You can use the `git restore` subcommand to restore files back to its clean state. For example, if you wanted to restore the 2nd and 5th file as well as all files containing a Git status in the directory labeled "b", you would run the following command:

```
nd git restore 2 5 b
```

[Back to Table of Contents](#table-of-contents)

# Customizing `nomad`

You can customize `nomad` to your liking if you do not like the default configuration.

## "What Can I Customize?"

The following settings are customizable for the normal tree:

* Indentation
* Indentation characters (the tree's branches)
* Padding
* Directory color
* Label colors (for labeled modes)
* Git markers and its colors
* Regex match color

The following settings are customizable for [Rootless mode](#rootless-mode):

* Widget border color
* Standard item highlight color (the color for items that do not contain a Git status)
* Git status colors (for items)
* Regex match color

## `config edit` - Edit the Configuration File

To simplify customization, **you do *not* have to make your own configuration file** -- `nomad` will write one to your disk if it does not already exist.

To access the configuration file, simply use the `config edit` subcommand:

```
nd config edit
```

I have written a walkthrough of sorts into the TOML file detailing how you can choose built-in colors or pick a custom color, so I will not elaborate how to do so here. You can also take a look at `nomad.toml` in this repository; this is the same configuration file that is written to your disk.

## `config preview` - Preview Your Settings

You can quickly see a preview of your settings by using the `config preview` subcommand:

```
nd config preview
```

This will display a dummy tree with all your settings applied.

[Back to Table of Contents](#table-of-contents)

# `releases` - View `nomad` Releases

You can view `nomad` releases directly from `nomad` itself. Releases are retrieved from GitHub.

## `releases all` - View All Releases

Use the `releases all` subcommand to view all `nomad` releases in a table containing:

* The name of the release
* Version number
* Release Date
* Description
* Attached assets

```
nd releases all
```

## `releases VERSION_NUMBER` - View Release Data for a Specific Version

You can view release data for a specific version by passing in the version number. For example, if you wanted to view release data for v1.0.0, you would run the following command:

```
nd releases v1.0.0
```

[Back to Table of Contents](#table-of-contents)

# Upgrading `nomad`

`nomad` can upgrade itself! Simply run `nd upgrade` to upgrade to the latest version. You can also run `nd upgrade --check` to just check if there is an update available.

[Back to Table of Contents](#table-of-contents)


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
[Tokei]: https://github.com/XAMPPRocky/tokei
[Vi]: http://ex-vi.sourceforge.net/
[Vim]: https://www.vim.org/
