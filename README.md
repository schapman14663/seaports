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
  |-*project name* [required]
  |-chase [flag]
  |-edit [flag]
  |-verbose [flag]

```

### Aliases
I am trying to include aliases for some flags that make sense but feel free to let me know if you feel any should be added.

Known Aliases:

\[Chase Flag\]
- "cd"
- "zox" (yes for zoxide, I know that zoxide should actually probably be a 'z' but I haven't decided on if I'll include 'z' for Zig or not in the Language Argument)

\[Edit Flag\]
- "nano"
- "vim"
- "nvim"
- "code"
- "vscode"

When I actually get to implementing the Edit Flag properly the idea is probably to look at using an environment variable (possibly something like $SEAPORTS_EDITOR) to automatically launch the users preferred editor.

\[No Git\]
- "ng"

I am planning to look into support for other Version Controls (again, possibly through use of some kind of environment variable i.e. $SEAPORTS_VC)

\[No Docker\]
- "nd"

Same with Containers, support for other Containerization methods to come later once I've got initial set up in place (And again, likely through an environment variable i.e. $SEAPORTS_CONTAINERS)


