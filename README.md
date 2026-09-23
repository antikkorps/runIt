# runIt

Fuzzy-pick a command out of a folder of **markdown** notes, fill in its
`<parameters>`, get it back ready to run.

The note stays the **single source**: no `.cheat` file to maintain alongside it,
runIt reads the markdown directly. And unlike a classic cheatsheet manager, it
keeps **the detail of the note behind the command**: while you pick, the preview
pane shows the section of the note around the line — the context and the typing
in the same place.

## How it works

1. runIt walks your notes and indexes every **line** of a ` ```sh ` block as a
   command. The trailing comment (`command   # description`) becomes the label.
2. `fzf` lets you search by description; the right-hand pane shows the note.
3. The `<param>` holes of the chosen line are asked for one by one (`$VAR`, on
   the other hand, are left to the shell).
4. The assembled command goes to `stdout`.

## Install

```sh
cargo build --release
# the binary: target/release/runit
```

Requires [`fzf`](https://github.com/junegunn/fzf) in the `PATH`.

## Use

```sh
# point it at the notes, then run it
RUNIT_ROOT=~/documents/memento/fiches runit
```

`RUNIT_ROOT` can also be passed as the first argument; failing that, runIt looks
for `./fiches`.

## Expected note format

- ` ```sh ` / ` ```bash ` blocks holding **one command per line**;
- an optional description after `#`, separated by **two or more spaces**;
- a `<parameter>` wherever a value changes on every use (`ssh root@<ip>`) —
  never a real pasteable value, never a `$VAR` from the environment.

```sh
awk -F'<sep>' '{print substr($<col>,1,<n>)}' <fichier>   # extraire une colonne
```

The notes themselves are in French, and so are the messages runIt prints; the
code and its documentation are in English.

## Status

See [BACKLOG.md](BACKLOG.md) — it holds the current state and what comes next,
so it is not repeated here.

## Licence

Personal project, to be defined.
