# `git` - Run Git Commands

> **NOTE**: Most Git commands require a preceeding run in a [labeled mode](../labels.md).

In addition to respecting ignore-type files such as `.gitignore`s and displaying [Git status markers](./status_markers.md), `nomad` implements the following Git commands:

* [`git add`](./add.md)
* [`git blame`](./blame.md)
* [`git branch`](./branch.md)
* [`git commit`](./commit.md)
* [`git diff`](./diff.md)
* [`git restore`](./restore.md)
* [`git status`](./status.md)

> **TIP:** I recommend taking a look at [`git status`](./status.md) before looking at the other sections.

**These commands are not meant to be a full replacement for the original Git commands yet!** Git is a *massive* ecosystem, so I have not yet implemented these commands to support everything the original `git` counterpart is capable of. `nomad`'s `git` commands work well if you need to do something basic with those Git commands, however.

All Git-related functionality is implemented via the [`git2`][git2] crate, which provides an API to Git.

[git2]: https://docs.rs/git2/latest/git2/
