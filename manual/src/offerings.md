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
* [Rootless mode](./rootless.md) -- `nomad` in a TUI (terminal UI) with file preview and pattern searching capability.
* Integrated [`Tokei`][Tokei], a program that displays statistics about your code such as the number of files, total lines, comments, and blank lines.
* Convenient configuration editing and preview.
* It's self-upgrading and you can see the available releases all from your terminal!

[bat]: https://github.com/sharkdp/bat
[tokei]: https://github.com/XAMPPRocky/tokei
