use std::collections::HashMap;

struct Hole {
    name: String,
    start: usize,
    end: usize,
}

#[derive(PartialEq, Debug)]
pub enum Kind {
    Value,
    Program,
}

fn holes(cmd: &str) -> Vec<Hole> {
    let mut out: Vec<Hole> = Vec::new();
    let bytes = cmd.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'<' {
            if let Some(j) = cmd[i + 1..].find('>') {
                let nom = &cmd[i + 1..i + 1 + j];
                let start = i;
                let end = i + j + 2;
                let valide = !nom.is_empty()
                    && nom
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
                if valide {
                    out.push(Hole {
                        name: nom.to_string(),
                        start,
                        end,
                    });
                    i = end;
                    continue;
                }
            }
        }
        i += 1;
    }
    out
}

/// True if byte offset `pos` sits inside a '...' span
fn in_single_quotes(cmd: &str, pos: usize) -> bool {
    let mut single = false;
    let mut double = false;
    for b in cmd[..pos].bytes() {
        match b {
            b'\'' if !double => single = !single,
            b'"' if !single => double = !double,
            _ => {}
        }
    }
    single
}

/// Extracts the `<param>` holes in order of appearance, without duplicates.
/// Reacts **only to `<...>`**: `$VAR` are genuine environment variables, we
/// leave them to the shell.
pub fn params(cmd: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for h in holes(cmd) {
        if !out.iter().any(|x| x == &h.name) {
            out.push(h.name);
        }
    }
    out
}

/// Replaces every `<name>` with the value that was typed in.
pub fn substitute(cmd: &str, values: &HashMap<String, String>) -> (String, Option<usize>) {
    let mut out = String::new();
    let mut last = 0;
    let mut first_missing: Option<usize> = None;
    for h in holes(cmd) {
        out.push_str(&cmd[last..h.start]);
        match values.get(&h.name) {
            Some(value) if in_single_quotes(cmd, h.start) => {
                out.push_str(&value.replace('\'', r"'\''"))
            }
            Some(value) => out.push_str(value),
            None => {
                if first_missing.is_none() {
                    first_missing = Some(out.len());
                }
            }
        }
        last = h.end;
    }
    out.push_str(&cmd[last..]);
    (out, first_missing)
}

/// zsh's CURSOR counts characters, Rust offsets are bytes
pub fn char_offset(cmd: &str, position: usize) -> usize {
    let end = position.min(cmd.len());
    cmd.char_indices().take_while(|&(i, _)| i < end).count()
}

/// Identify the hole type
#[cfg_attr(not(test), expect(dead_code, reason = "used by main.rs at step 2"))]
pub fn kind(name: &str) -> Kind {
    match name {
        "motif" | "regex" | "rempl" | "remplacement" => Kind::Program,
        _ => Kind::Value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_in_order_without_double() {
        let p = params("awk -F'<sep>' '{print substr($<col>,1,<n>)}' <fichier>");
        assert_eq!(p, vec!["sep", "col", "n", "fichier"]);
    }

    #[test]
    fn ignore_env_variables() {
        assert!(params("echo $HOME/$USER").is_empty());
    }

    #[test]
    fn replaces_a_hole_with_its_value() {
        let mut v = HashMap::new();
        v.insert("ip".to_string(), "10.10.10.5".to_string());
        assert_eq!(substitute("ssh root@<ip>", &v).0, "ssh root@10.10.10.5");
    }

    #[test]
    fn does_not_resubstitute_a_value() {
        for _ in 0..50 {
            let mut v = HashMap::new();
            v.insert("a".to_string(), "<b>".to_string());
            v.insert("b".to_string(), "x".to_string());

            assert_eq!(substitute("sed 's/<a>/<b>/' f", &v).0, "sed 's/<b>/x/' f");
        }
    }

    #[test]
    fn escapes_quote_inside_single_quotes() {
        let mut v = HashMap::new();
        v.insert("motif".to_string(), "l'erreur".to_string());
        assert_eq!(
            substitute("awk '/<motif>/' f", &v).0,
            r"awk '/l'\''erreur/' f"
        );
    }

    #[test]
    fn substitute_after_single_quotes() {
        let mut v = HashMap::new();
        v.insert("fichier".to_string(), "l'a.txt".to_string());
        assert_eq!(
            substitute("awk '{print}' <fichier>", &v).0,
            r"awk '{print}' l'a.txt"
        );
    }

    #[test]
    fn program_holes_are_known_by_name() {
        assert_eq!(kind("motif"), Kind::Program);
    }

    #[test]
    fn cursor_count_chars_not_bytes() {
        // "é" fait 2 octets mais 1 caractère
        assert_eq!(char_offset("cp é <src>", 6), 5);
        assert_eq!(char_offset("cp a <src>", 6), 6);
    }

    #[test]
    fn reports_first_unfilled_hole_without_keeping_placeholder() {
        // Aucune valeur fournie : le trou <ip> disparait de la sortie,
        // et on signale la position (en octets) où il commençait dans `out`
        let v = HashMap::new();
        let (out, pos) = substitute("ssh root@<ip> -p 22", &v);

        assert_eq!(out, "ssh root@ -p 22"); //"ssh root@" = 9 octets
        assert_eq!(pos, Some(9));
    }

    #[test]
    fn only_first_unfilled_hole_is_reported() {
        let mut v = HashMap::new();
        v.insert("a".to_string(), "X".to_string());
        let (out, pos) = substitute("<a>-<b>-<c>", &v);

        assert_eq!(out, "X--"); // <b> et <c> disparaissent aussi
        assert_eq!(pos, Some(2)); //position après "X-"
    }

    #[test]
    fn all_filled_means_none() {
        let mut v = HashMap::new();
        v.insert("ip".to_string(), "10.0.0.1".to_string());
        let (out, pos) = substitute("ssh <ip>", &v);

        assert_eq!(out, "ssh 10.0.0.1");
        assert_eq!(pos, None)
    }
}
