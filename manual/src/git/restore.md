# `git restore`

> **NOTE**: Requires a preceeding run in a [labeled mode](../labels.md).

You can use the `git restore` subcommand to restore files back to its clean state. For example, if you wanted to restore the 2nd and 5th file as well as all files containing a Git status in the directory labeled "b", you would run the following command:

```
nd git restore 2 5 b
```

