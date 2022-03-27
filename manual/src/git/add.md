# `git add`

> **NOTE**: Requires a preceeding run in a [labeled mode](../labels.md).

You can use the `git add` subcommand to quickly stage files. For example, if you wanted to stage the 2nd and 5th file as well as all files containing a Git status in the directory labeled "b", you would run the following command:

```
nd git add 2 5 b
```

If you pass a directory label, only items containing a Git status will be staged, just like the original `git add` command.
