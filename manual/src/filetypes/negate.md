# Negating Filetypes or Globs

You can exclude filetypes or globs by using the `negate` subcommand. Use the `-f` flag to exclude filetypes and the `-g` flag to exclude globs.

For example, if you only wanted to negate C++ files and files that match the glob `*.asdf`, you would run:

```
nd ft negate -f cpp -g "*.asdf"
```

Both the `-f` and `-g` flags accept a list of filetypes or globs. An example:

```
nd ft negate -f c cpp java -g "*.asdf" "*.qwerty"
```

