# `config edit` - Edit the Configuration File

To simplify customization, **you do *not* have to make your own configuration file** -- `nomad` will write one to your disk if it does not already exist.

To access the configuration file, simply use the `config edit` subcommand:

```
nd config edit
```

I have written a walkthrough of sorts into the TOML file detailing how you can choose built-in colors or pick a custom color, so I will not elaborate how to do so here. You can also take a look at [`nomad.toml`][nomad.toml] in this repository; this is the same configuration file that is written to your disk.

[nomad.toml]: https://github.com/JosephLai241/nomad/blob/main/nomad.toml
