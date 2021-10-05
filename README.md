# Colorways

A Sokoban-ish puzzle game where things are exactly as they seem

My entry for Ludum Dare 49 - Unstable

![the title screen](raw/title_scrsh.png)

## Running from Source
- Make sure [Rust 1.55+](https://www.rust-lang.org/tools/install) is installed
- Download the source and run `cargo run --release`

### OSX
- OSX users may encounter an awful C++ compiler error including the following:

```c++
cargo:warning=clipper/wrapper.cpp:21:34: error: a space is required between consecutive right angle brackets (use '> >')
```

If this happens, try using the [ld49-submission-osx](https://github.com/orez-/ld49/tree/ld49-submission-osx) branch.

- If you get an error message about being unable to link `-lSDL2`, try installing SDL:

```bash
brew install SDL2
```
