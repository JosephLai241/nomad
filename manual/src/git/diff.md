# `git diff`

> **NOTE**: Requires a preceeding run in a [labeled mode](../labels.md).

You can use the `git diff` command to quickly display the diff for files. For example, if you wanted to `diff` the 2nd and 5th file as well as all files containing a Git status in the directory labeled "b", you would run the following command:

```
nd git diff 2 5 b
```

`nomad`'s `git diff` offers visual improvements over the original `git diff` command by making it much easier to read, in my opinion. See for yourself:

<!-- DO A SIDE BY SIDE COMPARISON OF THE ORIGINAL AND NOMAD'S GIT DIFF HERE -->

If you pass a directory label, only items containing a Git status will be `diff`ed, just like the original `git diff` command.
