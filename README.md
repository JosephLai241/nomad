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
    + ["This `README` is just a bunch of GIFs. Where the hell is the manual?"](#this-readme-is-just-a-bunch-of-gifs-where-the-hell-is-the-manual)
* [Prerequisites](#prerequisites)
* [Standard Usage](#standard-usage)
    + [Unlocked Functionality via Item Labels](#unlocked-functionality-via-item-labels)
* [`bat` - `bat` Files in the Tree](#bat---bat-files-in-the-tree)
* [`edit` - Edit Files in the Tree](#edit---edit-files-in-the-tree)
* [`tokei`](#tokei)
    + [`tokei` - Subcommand (Overview)](#tokei---subcommand-overview)
    + [`nd --tokei` - Flag (Tree View)](#nd---tokei---flag-tree-view)
* [Filtering (Including or Excluding) Items by Filetype](#filtering-including-or-excluding-items-by-filetype)
    + [`ft match` - Including Filetypes and/or Globs](#ft-match---including-filetypes-andor-globs)
	+ [`ft negate` - Excluding Filetypes and/or Globs](#ft-negate---excluding-filetypes-andor-globs)
    + [`ft options` - Viewing Filetype Globs](#ft-options---viewing-filetype-globs)
* [Git Integration](#git-integration)
	+ [Git Status Markers](#git-status-markers)
    + [`git add`](#git-add)
    + [`git blame`](#git-blame)
    + [`git branch`](#git-branch)
    + [`git commit`](#git-commit)
    + [`git diff`](#git-diff)
    + [`git restore`](#git-restore)
    + [`git status`](#git-status)
* [Rootless Mode](#rootless-mode)
* [Configuration/Customization](#configurationcustomization)
    + [`config edit` - Editing the Configuration File](#config-edit---editing-the-configuration-file)
	+ [`config preview` - Previewing Your Configurations](#config-preview---previewing-your-configurations)
* [Inspirations](#inspirations)

# Introduction

`nomad` is a rewrite of the [`tree`][tree] command with a ***ton*** of additional features such as Git integration, built-in [`bat`][bat] and [`tokei`][tokei], the ability to customize its appearance, and even a TUI (terminal UI) mode.

I think the `tree` command is a useful CLI tool, but is unfortunately lacking some features that I think would make it even better. I decided to build my own next gen `tree` command that implements the features I would have wanted in the original `tree` command.

## "This `README` is just a bunch of GIFs. Where the hell is the manual?"

The manual for `nomad` was originally this `README`, but it was too long to comfortably navigate. This `README` merely serves as a preview of sorts with some information for each feature.

**[The manual is available here][nomad manual]** and is also linked in the About section of this repository.

# Prerequisites

`nomad`'s icons require a [NerdFont][NerdFont] to render correctly. Refer to the [NerdFont installation instructions][NerdFont Installation] to install a NerdFont for your system. It is quite simple to do and will not take too long.

# Standard Usage

By default, `nomad` will display a tree visual representing the directory structure of the target directory and respect rules specified in ignore-type files such as `.gitignore`s. This behavior may be disabled.

Each item will also be labeled with a NerdFont-supported icon corresponding to its filetype as well as Git status markers indicating the Git status of the file. See the [Git Status Markers](#git-status-markers) section to learn more about what each default marker/color represents.

![standard-demo][standard-demo]

## Unlocked Functionality via Item Labels

**`nomad`'s capabilities are drastically enhanced through the use of item labels.**

These are the flags that will apply labels to items within the tree:

| Flag | Function                                                      |
|------|---------------------------------------------------------------|
| `-l` | Applies labels to directories only                            |
| `-n` | Applies labels to items only                                  |
| `-L` | Applies labels to directories and items. An alias for `-l -n` |

# `bat` - `bat` Files in the Tree

> **NOTE:** Requires a preceeding run in a [labeled mode](#unlocked-functionality-via-item-labels).

Quickly `bat` files by passing item labels into the `bat` subcommand.

> **NOTE:** This command works with item and/or directory labels. If directory labels are provided, all items within that directory will be `bat`ed in the order they appear.

![bat demo][bat demo]

# `edit` - Edit Files in the Tree

> **NOTE:** Requires a preceeding run in a [labeled mode](#unlocked-functionality-via-item-labels).

Quickly edit files by passing item labels into the edit subcommand.

> **NOTE:** This command works with item and/or directory labels. If directory labels are provided, all items within that directory will be opened in a text editor.

![edit demo][edit demo]

> Opened with [`Neovim`][Neovim], the best text editor.

# `tokei`

`Tokei` may be accessed through the subcommand or in tree view to display LoC data for individual files.

## `tokei` - Subcommand (Overview)

![tokei subcommand demo][tokei subcommand demo]

You can quickly see a `tokei` summary/overview for a project by using the `tokei` subcommand.

## `nd --tokei` - Flag (Tree View)

![tokei tree demo][tokei tree demo]

You can view `tokei` statistics for individual files by using the `--tokei` flag. This will display the lines of blanks, code, comments, and total number of lines for each file.

# Filtering (Including or Excluding) Items by Filetype

You can filter out items in a directory by `match`ing or `negate`ing filetypes or globs.

## `ft match` - Including Filetypes and/or Globs

![filetype match demo][filetype match demo]

## `ft negate` - Excluding Filetypes and/or Globs

![filetype negate demo][filetype negate demo]

## `ft options` - Viewing Filetype Globs

You can view all the preset globs for each filetype by using the `ft options` subcommand. Optionally specify a filetype after the subcommand to search/view the globs for that specific filetype.

![filetype options demo][filetype options demo]

# Git Integration

`nomad` has Git integration to allow for easy access to commonly used Git subcommands!

## Git Status Markers

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

> **NOTE:** Staged deleted filenames will also be painted with a strikethrough.

If you do not like the default marker or color configuration, you can [customize it to your liking](#configurationcustomization).

## `git add`

> **NOTE:** Requires a preceeding run in a [labeled mode](#unlocked-functionality-via-item-labels).

Quickly `git add` files by passing items labels into the subcommand.

> **NOTE:** This command works with item and/or directory labels. If directory labels are provided, all items within that directory that are tracked by Git and contain a Git status will be added.

![git add demo][git add demo]

## `git blame`

> **NOTE:** Requires a preceeding run in a [labeled mode](#unlocked-functionality-via-item-labels).

Quickly run `git blame` on a file by passing an item label into the subcommand.

> **NOTE:** This command only accepts one item label.

Commits made by you remain plain while commits made by other authors are painted with a color. Each author is assigned a random color, so these colors will be different each time you run `git blame`.

![git blame demo][git blame demo]

## `git branch`

You can view `git branch` in tree form. This works especially well if your branch names look like filepaths. Some examples are:

* `feature/git/something-new`
* `bugfix/some-bug`

![git branch demo][git branch demo]

## `git commit`

`git commit` has been integrated for convenience and offers some visual improvements over the original command.

![git commit demo][git commit demo]

## `git diff`

> **NOTE:** Requires a preceeding run in a [labeled mode](#unlocked-functionality-via-item-labels).

Quickly `git diff` files by passing item labels into the subcommand. This command offers visual improvements and additional data over the original command.

> **NOTE:** This command works with item and/or directory labels. If directory labels are provided, all items within that directory that are tracked by Git and contain a Git status will be `diff`ed in the order they appear.

![git diff demo][git diff demo]

## `git restore`

> **NOTE:** Requires a preceeding run in a [labeled mode](#unlocked-functionality-via-item-labels).

Quickly `git restore` files by passing item labels into the subcommand.

> **NOTE:** This command works with item and/or directory labels. If directory labels are provided, all items within that directory that are tracked by Git and contain a Git status will be restored in the order they appear.

![git restore demo][git restore demo]

## `git status`

You can view `git status` in tree form. This command will only display items that are tracked by Git and contain changes or are untracked.

![git status demo][git status demo]

# Rootless Mode

Rootless mode is the TUI (terminal UI) mode that allows you to dynamically interact with directory trees.

![rootless mode demo][rootless mode demo]

# Configuration/Customization

You can configure/customize `nomad` *without* the hassles of creating your own configuration file in the correct directory with the correct syntax.

## `config edit` - Editing the Configuration File

You can easily access the configuration file by using the `config edit` subcommand.

![config edit demo][config edit demo]

## `config preview` - Previewing Your Configurations

You can preview all your configuration options in a dummy tree by using the `config preview` subcommand.

![config preview demo][config preview demo]

# Inspirations

Be sure to check out the tools that inspired me to create this project!

* [`tree`][tree] - the OG command
* [`lsd`][lsd] - a modern `ls` alternative/rewrite

<!-- DEMO GIFS -->
[bat demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/bat.gif
[edit demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/edit.gif
[tokei subcommand demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/tokei-subcommand.gif
[tokei tree demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/tokei-tree.gif
[filetype match demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/ft-match.gif
[filetype negate demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/ft-negate.gif
[filetype options demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/ft-options.gif
[git add demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/git-add.gif
[git blame demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/git-blame.gif
[git branch demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/git-branch.gif
[git commit demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/git-commit.gif
[git diff demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/git-diff.gif
[git restore demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/git-restore.gif
[git status demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/git-status.gif
[rootless mode demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/rootless-mode.gif
[standard-demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/standard.gif
[config edit demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/config-edit.gif
[config preview demo]: https://github.com/JosephLai241/nomad/blob/demo-gifs/gifs/config-preview.gif


<!-- LINKS -->
[bat]: https://github.com/sharkdp/bat
[lsd]: https://github.com/Peltoche/lsd
[tokei]: https://github.com/XAMPPRocky/tokei
[tree]: https://linux.die.net/man/1/tree

[nomad manual]: https://josephlai241.github.io/nomad/

[Neovim]: https://github.com/neovim/neovim
[NerdFont]: https://www.nerdfonts.com/
[NerdFont Installation]: https://github.com/ryanoasis/nerd-fonts#font-installation
