mod fill;
mod parse;
mod snippet;

use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, Write};
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    // Sous-commande cachée appelée par fzf pour le volet de preview.
    if args.len() >= 3 && args[1] == "preview" {
        return parse::preview(&args[2]);
    }

    // Racine des fiches : $RUNIT_ROOT, sinon 1er argument, sinon ./fiches
    let root = std::env::var("RUNIT_ROOT")
        .ok()
        .or_else(|| args.get(1).cloned())
        .unwrap_or_else(|| "fiches".to_string());

    let snippets = parse::walk(std::path::Path::new(&root))?;
    if snippets.is_empty() {
        eprintln!("runit : aucune commande trouvée sous « {root} »");
        std::process::exit(1);
    }

    // 1. Choix de la commande via fzf (avec preview de la fiche).
    let Some(cmd) = pick(&snippets)? else {
        return Ok(()); // Échap dans fzf : on ne sort rien.
    };

    // 2. Remplissage des <param>, lus au clavier via /dev/tty.
    let params = fill::params(&cmd);
    let mut valeurs: HashMap<String, String> = HashMap::new();
    if !params.is_empty() {
        let tty = std::fs::File::open("/dev/tty")?;
        let mut reader = std::io::BufReader::new(tty);
        for p in &params {
            eprint!("{p} = ");
            std::io::stderr().flush()?;
            let mut buf = String::new();
            reader.read_line(&mut buf)?;
            valeurs.insert(p.clone(), buf.trim_end_matches(['\n', '\r']).to_string());
        }
    }

    // 3. La commande assemblée sort sur stdout : le widget shell l'insère.
    println!("{}", fill::substitute(&cmd, &valeurs));
    Ok(())
}

/// Envoie les snippets à fzf (une ligne = `desc \t command \t file:line`),
/// n'affiche que desc+command, et branche le preview sur le champ caché.
fn pick(snippets: &[snippet::Snippet]) -> Result<Option<String>, Box<dyn Error>> {
    let exe = std::env::current_exe()?;
    let preview = format!("{} preview {{3}}", exe.display());

    let mut fzf = Command::new("fzf")
        .args([
            "--delimiter",
            "\t",
            "--with-nth",
            "1,2",
            "--preview",
            &preview,
            "--preview-window",
            "right:60%",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        let mut si = fzf.stdin.take().ok_or("stdin de fzf indisponible")?;
        for s in snippets {
            writeln!(
                si,
                "{}\t{}\t{}:{}",
                s.description,
                s.command,
                s.file.display(),
                s.line
            )?;
        }
    } // stdin fermé ici : fzf peut afficher.

    let out = fzf.wait_with_output()?;
    if !out.status.success() {
        return Ok(None); // Échap, ou fzf absent/annulé.
    }
    let ligne = String::from_utf8_lossy(&out.stdout);
    // champ 2 (index 1) = la commande brute, avec ses <param>.
    match ligne.split('\t').nth(1) {
        Some(cmd) if !cmd.trim().is_empty() => Ok(Some(cmd.trim().to_string())),
        _ => Ok(None),
    }
}
