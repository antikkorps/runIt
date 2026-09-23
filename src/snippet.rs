use std::path::PathBuf;

/// The unit is **the command line** — the "every line must be pasteable"
/// convention of the notes. The description comes from the trailing comment
/// (`# ...`), the context from the nearest `## ` heading above.
pub struct Snippet {
    pub command: String, // "awk -F'<sep>' '{print substr($<col>,1,<n>)}' <fichier>"
    pub description: String, // the text after `#`, displayed in fzf
    pub file: PathBuf,   // fiches/shell/awk.md
    pub line: usize,     // 1-based — used by the preview and to locate the line
    #[allow(dead_code)]
    pub section: String, // the enclosing `## ...` (reserved for a future display)
}
