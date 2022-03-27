# `git branch`

You can use the `git branch` subcommand to display all your branches in a tree form (this may be disabled, see the [Usage](#usage) section).

`nomad`'s `git branch` displays additional data by default, such as:

* Whether a branch is `HEAD`
* Whether an upstream branch is set

> Unfortunately I have not implemented `git checkout` for v1.0.0, but I plan on implementing it in the next version for quick branch switching.

### Usage

```
USAGE:
    nd git branch [FLAGS] [OPTIONS]

FLAGS:
    -f, --flat          Display branches in a normal list
    -h, --help          Prints help information
        --no-icons      Do not display icons
    -n, --numbered      Label branches with numbers
    -s, --statistics    Display the total number of branches
    -V, --version       Prints version information

OPTIONS:
        --export <export>      Export the tree to a file. Optionally include a target filename
    -p, --pattern <pattern>    Only display branches matching this pattern. Supports regex expressions
```
