use crate::snippet::Snippet;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// Langages dont les blocs contiennent des commandes réellement collables.
fn est_shell(lang: &str) -> bool {
    matches!(lang, "sh" | "bash" | "console" | "shell" | "zsh")
}

/// Parcourt `root` récursivement et collecte tous les snippets shell.
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
        } else if path.extension().map_or(false, |e| e == "md") {
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

        // Ouverture / fermeture de bloc de code (fence ```)
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
            // Hors bloc : on ne suit que le fil des titres de section.
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
            continue; // ligne vide ou commentaire pur : rien à lancer
        }

        let (command, description) = split_desc(ligne);
        out.push(Snippet {
            command,
            // Pas de `# ...` ? on retombe sur le titre de section pour l'affichage.
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

/// Sépare `cmd   # description` sur **deux espaces ou plus** avant le `#`
/// (ta convention d'alignement des commentaires). Heuristique volontairement
/// simple : un `#` collé dans la commande n'est pas pris pour une description.
fn split_desc(ligne: &str) -> (String, String) {
    if let Some(pos) = ligne.find("  #") {
        let (cmd, desc) = ligne.split_at(pos);
        let desc = desc.trim_start().trim_start_matches('#').trim();
        (cmd.trim_end().to_string(), desc.to_string())
    } else {
        (ligne.trim().to_string(), String::new())
    }
}

/// Sous-commande `runit preview <chemin:ligne>` : appelée par fzf pour afficher
/// **la section de fiche autour** de la ligne choisie — le détail all-in-one.
pub fn preview(loc: &str) -> Result<(), Box<dyn Error>> {
    let (file, line) = loc.rsplit_once(':').ok_or("format attendu : chemin:ligne")?;
    let line: usize = line.trim().parse()?;
    let contenu = fs::read_to_string(file)?;
    let lignes: Vec<&str> = contenu.lines().collect();

    // Borne haute : le `## ` le plus proche au-dessus de la ligne cible.
    let mut debut = 0;
    for j in (0..line.saturating_sub(1)).rev() {
        if lignes.get(j).map_or(false, |l| l.starts_with("## ")) {
            debut = j;
            break;
        }
    }
    // Borne basse : le prochain `## `.
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
