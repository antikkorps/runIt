use crate::snippet::Snippet;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// Languages whose blocks hold commands that are actually pasteable.
fn est_shell(lang: &str) -> bool {
    matches!(lang, "sh" | "bash" | "console" | "shell" | "zsh")
}

/// Walks `root` recursively and collects every shell snippet.
pub fn walk(root: &Path) -> Result<Vec<Snippet>, Box<dyn Error>> {
    let mut out = Vec::new();
    walk_into(root, &mut out)?;
    Ok(out)
}

fn walk_into(dir: &Path, out: &mut Vec<Snippet>) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            walk_into(&path, out)?;
        } else if path.extension().is_some_and(|e| e == "md") {
            parse_file(&path, out)?;
        }
    }
    Ok(())
}

fn parse_file(path: &Path, out: &mut Vec<Snippet>) -> Result<(), Box<dyn Error>> {
    let contenu = fs::read_to_string(path)?;
    let mut section = String::new();
    let mut in_block = false;
    let mut block_is_shell = false;

    for (i, raw) in contenu.lines().enumerate() {
        let ligne = raw.trim_end();

        // Opening / closing of a code block (``` fence)
        if let Some(rest) = ligne.trim_start().strip_prefix("```") {
            if in_block {
                in_block = false;
            } else {
                in_block = true;
                block_is_shell = est_shell(rest.trim());
            }
            continue;
        }

        if !in_block {
            // Outside a block: we only track the thread of section headings.
            if let Some(titre) = ligne.strip_prefix("## ") {
                section = titre.to_string();
            }
            continue;
        }

        if !block_is_shell {
            continue;
        }

        let t = ligne.trim_start();
        if t.is_empty() || t.starts_with('#') {
            continue; // blank line or pure comment: nothing to run
        }

        let (command, description) = split_desc(ligne);
        out.push(Snippet {
            command,
            // No `# ...`? fall back to the section heading for display.
            description: if description.is_empty() {
                section.clone()
            } else {
                description
            },
            file: PathBuf::from(path),
            line: i + 1,
            section: section.clone(),
        });
    }
    Ok(())
}

/// Splits `cmd   # description` on **two or more spaces** before the `#` (the
/// comment-alignment convention of the notes). Deliberately simple heuristic:
/// a `#` glued to the command is not mistaken for a description.
fn split_desc(ligne: &str) -> (String, String) {
    if let Some(pos) = ligne.find("  #") {
        let (cmd, desc) = ligne.split_at(pos);
        let desc = desc.trim_start().trim_start_matches('#').trim();
        (cmd.trim_end().to_string(), desc.to_string())
    } else {
        (ligne.trim().to_string(), String::new())
    }
}

/// `runit preview <path:line>` subcommand: called by fzf to display **the
/// surrounding section of the note** for the highlighted line — the all-in-one
/// detail.
pub fn preview(loc: &str) -> Result<(), Box<dyn Error>> {
    let (file, line) = loc
        .rsplit_once(':')
        .ok_or("format attendu : chemin:ligne")?;
    let line: usize = line.trim().parse()?;
    let contenu = fs::read_to_string(file)?;
    let lignes: Vec<&str> = contenu.lines().collect();

    // Upper bound: the nearest `## ` above the target line.
    let mut debut = 0;
    for j in (0..line.saturating_sub(1)).rev() {
        if lignes.get(j).is_some_and(|l| l.starts_with("## ")) {
            debut = j;
            break;
        }
    }
    // Lower bound: the next `## `.
    let mut fin = lignes.len();
    for (j, l) in lignes.iter().enumerate().skip(line) {
        if l.starts_with("## ") {
            fin = j;
            break;
        }
    }

    for (j, l) in lignes.iter().enumerate().take(fin).skip(debut) {
        let marqueur = if j + 1 == line { '>' } else { ' ' };
        println!("{marqueur} {l}");
    }
    Ok(())
}
