# Current Limitations

As of v1.0.0, `nomad` unfortunately experiences a performance hit when running it in very large directories. This is due to the way the directory tree is built.

This project uses the [`ptree`][ptree] crate to build the tree via its `TreeBuilder`. The `TreeBuilder` stores all tree items before building and displaying the tree, so the tree will only be displayed after all items (depending on your traversal settings) have been visited.

I have plans to replace this crate with a tree implementation that will display items in real time when they are visited, but for now I would recommend against running it in very large directories.

[ptree]: https://docs.rs/ptree/latest/ptree/
