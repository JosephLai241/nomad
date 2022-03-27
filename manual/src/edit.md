# `edit` - Edit Files

> **NOTE**: Requires a preceeding run in a [labeled mode](./labels.md).

You can quickly open files in a text editor by using the `edit` subcommand.

`nomad` will try to find your default `$EDITOR` setting and open files with that editor if you have specified one. If your `$EDITOR` is not set, it will try to open files with the following text editor in this order:

1. [Neovim][Neovim]
2. [Vim][Vim]
3. [Vi][Vi]
4. [Nano][Nano]

The first editor that is found on your machine will be used to edit your files.

For example, if you wanted to open the 2nd and 5th file as well as all files in the directory labeled "b", you would run the following command:

```
nd edit 2 5 b
```

[Nano]: https://www.nano-editor.org/
[Neovim]: https://github.com/neovim/neovim
[Vi]: http://ex-vi.sourceforge.net/
[Vim]: https://www.vim.org/
