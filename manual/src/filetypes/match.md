# Matching Filetypes or Globs

You can include filetypes or globs by using the `match` subcommand. Use the `-f` flag to include filetypes and the `-g` flag to match globs.

For example, if you only wanted to include Rust files and files that match the glob `*.asdf`, you would run:

```
nd ft match -f rust -g "*.asdf"
```

Both the `-f` and `-g` flags accept a list of filetypes or globs. An example:

```
nd ft match -f rust toml py -g "*.asdf" "*.qwerty"
```

