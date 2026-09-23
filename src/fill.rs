use std::collections::HashMap;

struct Hole {
    name: String,
    start: usize,
    end: usize,
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
pub fn substitute(cmd: &str, values: &HashMap<String, String>) -> String {
    let mut out = String::new();
    let mut last = 0;
    for h in holes(cmd) {
        out.push_str(&cmd[last..h.start]);
        match values.get(&h.name) {
            Some(value) => out.push_str(value),
            None => out.push_str(&cmd[h.start..h.end]),
        }
        last = h.end;
    }
    out.push_str(&cmd[last..]);
    out
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
        assert_eq!(substitute("ssh root@<ip>", &v), "ssh root@10.10.10.5");
    }

    #[test]
    fn does_not_resubstitute_a_value() {
        for _ in 0..50 {
            let mut v = HashMap::new();
            v.insert("a".to_string(), "<b>".to_string());
            v.insert("b".to_string(), "x".to_string());

            assert_eq!(substitute("sed 's/<a>/<b>/' f", &v), "sed 's/<b>/x/' f");
        }
    }
}
