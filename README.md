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
* [`bat` Files in the Tree](#bat-files-in-the-tree)
* [Edit Files in the Tree](#edit-files-in-the-tree)
* [`tokei`](#tokei)
* [Filtering (Including or Excluding) Items by Filetype](#filtering-including-or-excluding-items-by-filetype)
    + [Viewing Filetype Globs](#viewing-filetype-globs)
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

# `bat` Files in the Tree

# Edit Files in the Tree

> Opened with [`Neovim`][Neovim].

# `tokei`

<!-- PUT BOTH TREE VIEW AND STANDALONE SUBCOMMAND GIFS HERE -->

# Filtering (Including or Excluding) Items by Filetype

## Viewing Filetype Globs

# Git Integration

## Git Status Markers

## `git add`

## `git blame`

## `git branch`

## `git commit`

## `git diff`

## `git restore`

## `git status`

# Rootless Mode

Rootless mode is the TUI (terminal UI) mode that allows you to dynamically interact with directory trees.

# Configuration/Customization

You can configure/customize `nomad` *without* the hassles of creating your own configuration file in the correct directory with the correct syntax.

# Inspirations

Be sure to check out the tools that inspired me to create this project!

* [`tree`][tree] - the OG command
* [`lsd`][lsd] - a modern `ls` alternative/rewrite


<!-- LINKS -->
[bat]: https://github.com/sharkdp/bat
[lsd]: https://github.com/Peltoche/lsd
[tokei]: https://github.com/XAMPPRocky/tokei
[tree]: https://linux.die.net/man/1/tree

<!-- UPDATE THIS ONCE THE MANUAL'S DEPLOYED -->
[nomad manual]: https://some-link.com

[Neovim]: https://github.com/neovim/neovim
[NerdFont]: https://www.nerdfonts.com/
[NerdFont Installation]: https://github.com/ryanoasis/nerd-fonts#font-installation
