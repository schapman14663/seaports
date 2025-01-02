# seaports
Seaports is a CLI written in Rust that is intended to allow users to quickly initialise project folders.

## Okay so why Rust?
Okay, so the truth of the matter is that if you're seeing this part of the README.md, I'm still in the very early stages of development. So it's very possible that it may be changed to something else instead of Rust. 

For now, the "Why Rust?" is relatively simple: I like the look of Rust, and for some reason, I feel drawn to it. Maybe it's the Crab.

There is also partially the aspect that Rust was the original inspiration for this app, as it's very much intended to shadow the ``cargo new`` functionality that Rust employs, and create something that can similarly be used to create a set of files as a starting point for the users projects.

## Current Intended Structure
Again, if you're seeing this it's probably still early days for this CLI. But the intention is that running a command like ``seaports new`` would create a new directory, with a reasonable set of initial files such as ``.gitignore``, ``src/``, initialise git and set up an initial docker file.

### Current Framework Mapping
```
seaports
|-new
  |-*project name* \[required\]
  |-chase \[flag\]
  |-edit \[flag\]
  |-verbose \[flag\]
```
