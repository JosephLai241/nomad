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
* [`bat` - `bat` Files in the Tree](#bat-bat-files-in-the-tree)
* [`edit` - Edit Files in the Tree](#edit-files-in-the-tree)
* [`tokei`](#tokei)
    + [`tokei` - Subcommand](#tokei-subcommand)
    + [`nd --loc` - Tree View](#nd-loc-tree-view)
* [Filtering (Including or Excluding) Items by Filetype](#filtering-including-or-excluding-items-by-filetype)
    + [`ft match` - Including Filetypes and/or Globs](#ft-match-including-filetypes-andor-globs)
	+ [`ft negate` - Excluding Filetypes and/or Globs](#ft-negate-excluding-filetypes-andor-globs)
    + [`ft options` - Viewing Filetype Globs](#viewing-filetype-globs)
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
    + [`config edit` - Editing the Configuration File](#config-edit-editing-the-configuration-file)
	+ [`config preview` - Previewing Your Configurations](#config-preview-previewing-your-configurations)
* [Inspirations](#inspirations)

# Introduction

`nomad` is a rewrite of the [`tree`][tree] command with a ***ton*** of additional features such as Git integration, built-in [`bat`][bat] and [`tokei`][tokei], the ability to customize its appearance, and even a TUI (terminal UI) mode.

I think the `tree` command is a useful CLI tool, but is unfortunately lacking some features that I think would make it even better. I decided to build my own next gen `tree` command that implements the features I would have wanted in the original `tree` command.

## "This `README` is just a bunch of GIFs. Where the hell is the manual?"

The manual for `nomad` was originally this `README`, but it was too long to comfortably navigate. This `README` merely serves as a preview of sorts with some information.

**[The manual is available here][nomad manual]** and is also linked in the About section of this repository.

# Prerequisites

`nomad`'s icons require a [NerdFont][NerdFont] to render correctly. Refer to the [NerdFont installation instructions][NerdFont Installation] to install a NerdFont for your system. It is quite simple to do and will not take too long.

# Standard Usage

`nomad`'s capabilities are drastically enhanced through the use of item labels.

# `bat` - `bat` Files in the Tree

![bat demo][bat demo]

# `edit` - Edit Files in the Tree

![edit demo][edit demo]

> Opened with [`Neovim`][Neovim].

# `tokei`

`Tokei` may be accessed through the subcommand or in tree view to display LoC data for individual files.

## `tokei` - Subcommand

![tokei subcommand demo][tokei subcommand demo]

## `nd --loc` - Tree View

![tokei tree demo][tokei tree demo]

# Filtering (Including or Excluding) Items by Filetype

You can filter out items in a directory by `match`ing or `negating` filetypes or globs.

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

## `git add`

![git add demo][git add demo]

## `git blame`

![git blame demo][git blame demo]

## `git branch`

![git branch demo][git branch demo]

## `git commit`

![git commit demo][git commit demo]

## `git diff`

![git diff demo][git diff demo]

## `git restore`

![git restore demo][git restore demo]

## `git status`

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
