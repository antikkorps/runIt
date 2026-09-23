# runIt — backlog

What it is and how the notes must look: see the [README](README.md) — not
repeated here. This file holds the **state** and **what comes next**.

North star: what navi does not do is keep **the detail of the note behind the
command** (preview), all of it inside a typing interface. Guardrail: do not
reimplement navi feature by feature.

## Current state (MVP)

- [x] parse markdown → shell snippets (one line = one snippet, `#` = description)
- [x] fzf to pick, with a preview of the note's section
- [x] filling of the `<param>` holes (never the `$VAR`), typed at the keyboard
- [x] assembled command **printed** on stdout
- [x] zero dependency (std + external fzf), 4 unit tests
- [x] one single scanner, `holes()`, returning each hole with its byte span

Accepted limit of the MVP: it **prints**, it does not insert into the prompt yet.

## P1 — the widget, keystone

Decision: runIt **lives through the widget** (no "run on its own" experience to
polish). A single widget mechanism unlocks three capabilities at once.

- [ ] **zsh widget — insert at the cursor**: the shortcut inserts the assembled
      command at the cursor position (`LBUFFER`), it does not *replace* the line.
      From that single rule follow:
  - **typing on the fly** — the command lands in the prompt, ready;
  - **path completion done by the shell (Philosophy 2)** — a path-shaped `<param>`
    is left empty with the cursor on it, and **your zsh** completes it (Tab, `~`,
    globs, your own functions); we do not reimplement completion;
  - **pipe composition** — since we insert at the cursor and the widget can be
    re-invoked: `foo |` → widget → `grep …` is inserted → `|` → widget → … Nothing
    more to write, it falls out of "insert at the cursor" + "re-invocable".
- [ ] **Cursor/hole protocol**: how the widget learns where to put the cursor.
      Options: runIt emits `offset\tcommand`, or a sentinel character the widget
      strips. Cursor on the 1st hole if there is one, otherwise at end of line.
- [ ] **Guardrail**: runIt inserts text, **the shell owns the pipe**. Do not try to
      "understand" the pipeline (columns from stage n to stage n+1…) — that is the
      navi trap wearing another hat.
- [ ] **Root configuration**: `~/.config/runit/config` or a default, so that
      `RUNIT_ROOT` need not be passed on every call.
- [ ] **Clear errors**: fzf missing, `/dev/tty` unavailable, empty root.

## P2 — filling the variables (the differentiating core)

**Three kinds of holes, not one.** The kind comes from the **name** of the
parameter, already written in the note: no new syntax to invent.

| kind | examples | treatment |
|---|---|---|
| **value** | `<seuil>`, `<col>`, `<n>`, `<ip>` | raw substitution, that is enough |
| **syntax** | `<sep>`, `<delim>` | closed list → menu (`,` `;` `\t` `:` …) |
| **program / pattern** | `<motif>`, `<regex>`, `<remplacement>` | no filling: empty hole, cursor placed on it |

The rule that follows:

> **runIt does not fill what it could not quote — it puts the cursor there.**

This is Philosophy 2 (P1) extended from paths to regexes. It holds because we
**insert** instead of executing: a badly quoted command is visible before you
press Enter. navi does the same raw substitution we do — the ground is free.

Observed on the sed/awk notes. Extracting the `<param>` holes does hold already
(`awk 'NR>=10 && NR<=20'` and `awk '$3 > 100'` yield no false positive: the
`alnum|_|-` validation of the name is what filters). What breaks is the value
injected into a context that has a syntax of its own:

```
sed 's/<motif>/…/g'  + /usr/local     → sed: unknown option to `s'
awk '/<motif>/ …'    + erreur d'accès → sh: Unterminated quoted string
sed 's/<motif>/[&]/' + a              → & = the whole match, SILENT error
```

- [ ] **Type the hole from its name**: name → kind table (`motif`, `regex`,
      `remplacement` → program; `sep`, `delim` → syntax; anything else → value).
      No new syntax in the notes; in exchange it forces **naming things right** on
      the memento side — which we want anyway (see P4).
- [ ] **Suggestions for enumerable variables** (*syntax* kind): `<sep>`,
      `<niveau>`… → menu of a fixed list. Path-shaped variables do NOT belong here:
      they go through shell completion (Ph. 2, see P1), not through an `fd` menu.
- [ ] **Program holes left empty** (*program* kind): ask nothing, emit the hole
      empty and let the widget put the cursor on it — same cursor/hole protocol
      as in P1.
- [ ] **Minimal, deterministic escaping**: know whether the hole sits inside a
      `'…'` (scan the quotes up to the offset) and, if so, turn `'` into `'\''` in
      the typed value. ~15 lines, fixes the awk case cleanly. The `sed s///`
      delimiter is **not** code: it is a note convention (`s|<motif>|<rempl>|`,
      already documented in `fiches/shell/sed.md`).
- [x] **One single notion of "placeholder"** (DRY, and a real bug): `params()`
      scanned and validated the holes while `substitute()` did a naive `replace()`
      over a `HashMap` — two definitions free to drift, and a result that depended
      on the iteration order. With `<a>` = `<b>` and `<b>` = `x`, the same input
      yielded `sed 's/<b>/x/'` or `sed 's/x/x/'` from one run to the next, because a
      substituted value got substituted again. Now `holes()` is the only scanner: it
      returns each hole with its byte span, `params()` is one of its clients (names,
      in order, deduplicated) and `substitute()` rebuilds the string in a single
      pass, never re-reading what it just wrote. The hole typing above lands on top
      of it.
- [ ] **Memory of what was typed**: offer back the last value used for a given
      `<param>`.
- [ ] **Clean cancellation** in the middle of filling (Esc → we output nothing).

## P3 — parsing robustness

- [ ] **Multi-line commands**: heredocs, SQL spread over several lines, `\`
      continuations. Today one line = one snippet.
- [ ] **The `  #` heuristic**: a command that itself contains `  #` breaks the
      desc/command split. Handle quotes, or an explicit description syntax.
- [ ] **Languages**: decide what to do with `sql`, `toml`… blocks (show them in
      the preview without offering them for execution?).

## P4 — note-side convention (to settle together with the memento)

- [ ] **Placeholders vs concrete examples**: the "example values" convention is a
      *safety* rule (no real pasteable value), not a preference for the concrete.
      So `<IP>` is fine, and even safer. To decide: a placeholder for what **varies
      per use**, keep concrete what **illustrates** (`chmod 600`).
- [ ] Possible "spot the reserved values" mode (10.10.10.5, johndoe) as candidate
      params, without touching the notes.

## P5 — quality & project

- [ ] **Rename the binary** (collision with the `runit` init system).
- [ ] **Finish the English pass on identifiers**: the MVP still carries French
      names (`nom`, `valide`, `est_shell`, `contenu`, `ligne`, `debut`, `fin`,
      `marqueur`, `titre`). Comments, docs and the newer code are already English.
      Do it with the LSP rename (`grn`), in its own commit — a rename diff mixed
      with a behaviour change is a diff nobody reviews.
- [ ] **README**: what, why, install, the widget.
- [ ] **Integration tests** on a fixture corpus (a folder of sample notes).
- [x] **CI**: `cargo fmt --check`, `clippy`, `test` (Forgejo Actions) —
      `.forgejo/workflows/ci.yml`, image pinned to `rust:1.95-slim`.

## Ideas / further out

- [ ] **Native TUI** (`ratatui`) with styled markdown in the preview, once fzf
      shows its limits.
- [ ] **`--run` mode**: execute directly (with a confirmation) instead of printing.
- [ ] Work on **any markdown base**, not just the memento.
