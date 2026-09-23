use std::collections::HashMap;

/// Extracts the `<param>` holes in order of appearance, without duplicates.
/// Reacts **only to `<...>`**: `$VAR` are genuine environment variables, we
/// leave them to the shell.
pub fn params(cmd: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let bytes = cmd.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'<' {
            if let Some(j) = cmd[i + 1..].find('>') {
                let nom = &cmd[i + 1..i + 1 + j];
                let valide = !nom.is_empty()
                    && nom
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
                if valide {
                    if !out.iter().any(|x| x == nom) {
                        out.push(nom.to_string());
                    }
                    i += 1 + j + 1;
                    continue;
                }
            }
        }
        i += 1;
    }
    out
}

/// Replaces every `<name>` with the value that was typed in.
pub fn substitute(cmd: &str, valeurs: &HashMap<String, String>) -> String {
    let mut s = cmd.to_string();
    for (k, v) in valeurs {
        s = s.replace(&format!("<{k}>"), v);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extrait_dans_l_ordre_sans_doublon() {
        let p = params("awk -F'<sep>' '{print substr($<col>,1,<n>)}' <fichier>");
        assert_eq!(p, vec!["sep", "col", "n", "fichier"]);
    }

    #[test]
    fn ignore_les_variables_env() {
        assert!(params("echo $HOME/$USER").is_empty());
    }

    #[test]
    fn substitue() {
        let mut v = HashMap::new();
        v.insert("ip".to_string(), "10.10.10.5".to_string());
        assert_eq!(substitute("ssh root@<ip>", &v), "ssh root@10.10.10.5");
    }
}
