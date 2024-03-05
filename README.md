# crepe-bordeaux
The cross-platform clipboard cli tool

![crepe bordeaux](crepe-bordeaux.png)


### Copy:
```console
$ echo "foo" | cb
```


### Paste:
```console
$ cb
foo
```

### Clear:
```console
$ cb clear
```

### Save to a register:
```console
$ echo "thing I want to save for a while" | cb memorable-name
```

### Paste from a register:
```console
$ cb memorable-name
thing I want to save for a while
```

### Clear a register:
```console
$ cb memorable-name clear
```

### Clear system clipboard and all registers:
```console
$ cb clear-all
```

### Select a register interactively with [fzf](https://github.com/junegunn/fzf) (or [skim](https://github.com/lotabout/skim)) in bash:
```
$ cb $(cb list | fzf)
```


### No clipboard available?
`cb` will write to a .txt file in the folder determined by Rust's [std::env::temp_dir](https://doc.rust-lang.org/std/env/fn.temp_dir.html)

### Want to use a different folder?
Set the environment variable `CB_DIR`

### Why, though?
I was enjoying [Amila's](https://github.com/amilajack/clipboard) `cb` interface on my main computer and had already forked it to add Wayland support with [arboard](https://github.com/1Password/arboard). However, on a minimal install of NixOS with no display/window manager there was no clipboard for it to use, so I made this.

### Why "crepe-bordeaux", though?
The names [clipboard-cli](https://crates.io/crates/clipboard-cli) and [cli-clipboard](https://crates.io/crates/cli-clipboard) were already taken, and [tealdeer](https://crates.io/crates/tealdeer) inspired creativity.
