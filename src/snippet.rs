use std::path::PathBuf;

/// L'unité, c'est **la ligne** de commande — ta convention « chaque ligne
/// collable ». La description vient du commentaire de fin de ligne (`# ...`),
/// le contexte vient du dernier titre `## ` rencontré au-dessus.
pub struct Snippet {
    pub command: String,     // "awk -F'<sep>' '{print substr($<col>,1,<n>)}' <fichier>"
    pub description: String, // le texte après `#`, affiché dans fzf
    pub file: PathBuf,       // fiches/shell/awk.md
    pub line: usize,         // 1-indexé — sert au preview et au repérage
    #[allow(dead_code)]
    pub section: String, // le `## ...` englobant (réservé pour un affichage futur)
}
