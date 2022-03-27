# `bat` - `bat` Files

> **NOTE**: Requires a preceeding run in a [labeled mode](./labels.md).

You can quickly `bat` files by using the `bat` subcommand. As mentioned before, `bat` is a modern `cat` alternative that supports syntax highlighting and displays Git change markers like you would see in your text editor, to name two of its features.

For example, if you wanted to `bat` the 2nd and 5th file as well as all files in the directory labeled "b", you would run the following command:

```
nd bat 2 5 b
```

Files will be `bat`ed in the order of the labels you provide.
