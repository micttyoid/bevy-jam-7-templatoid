# Bevy T2PW Templatoid

Forked from [Bevy_new_2d](https://github.com/TheBevyFlock/bevy_new_2d)

# Start

Clone the repository
```sh
git clone https://github.com/micttyoid/bevy-t3pw-templatoid.git
cd bevy-t3pw-templatoid
```

You may compile the latest version of the CLI from scratch using `cargo install`
```sh
cargo install --git https://github.com/TheBevyFlock/bevy_cli --tag cli-v0.1.0-alpha.2 --locked bevy_cli
```

Run it on your browser
```sh
bevy run web --open
```

If you want to run it on your desktop (not on your browser)
```sh
cargo run
```

If you don't have:
- Rust compiler: here's [the instruction](https://rust-lang.org/tools/install/)

# Note on the external tileset(tsx) used in a map file(tmx)

The current work around:
- Use external tileset
- Embed exactly the same tileset (but don't use it)

Why is this necessary even if embedding a tileset effectively the same as using
the embedded tileset
- Versatility/just-work for your non-technical teammate

# TODO

- Document better
- 
- Featurize aseprite support
- Bring good stuff back from the mainstream: comments
