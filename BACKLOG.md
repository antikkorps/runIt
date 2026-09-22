# runIt — backlog

Petit outil : fuzzy-pick une commande depuis des fiches markdown, remplir ses
`<paramètres>`, la sortir prête à lancer. La fiche reste la **source unique** —
pas de `.cheat` intermédiaire. Ce que navi ne fait pas : garder **le détail de
la fiche derrière la commande** (preview), le tout dans une interface de saisie.

## État actuel (MVP)

- [x] parse markdown → snippets shell (une ligne = un snippet, `#` = description)
- [x] fzf pour choisir, avec preview de la section de fiche
- [x] remplissage des `<param>` (jamais les `$VAR`), saisie au clavier
- [x] commande assemblée **imprimée** sur stdout
- [x] zéro dépendance (std + fzf externe), 3 tests unitaires

Limite assumée du MVP : ça **imprime**, ça n'insère pas encore dans le prompt.

## P1 — le rendre utilisable au quotidien

- [ ] **Widget shell (zsh)** : un raccourci clavier qui *insère* la commande dans
      la ligne de commande au lieu de l'imprimer. C'est LE cran qui fait passer de
      démo à outil. (~10 lignes de zsh, cf. `navi widget zsh`.)
- [ ] **Config de la racine** : un fichier `~/.config/runit/config` ou un défaut
      sensé, pour ne plus passer `RUNIT_ROOT` à chaque appel.
- [ ] **Erreurs claires** : fzf absent, `/dev/tty` indisponible, racine vide →
      message utile plutôt qu'un panic.

## P2 — remplissage des variables (le cœur différenciant)

- [ ] **Suggestions par variable** : `<fichier>` / `<path>` → menu `fd`. Heuristique
      par nom de variable ; free-text sinon. (Le manque qu'on a vu dans navi.)
- [ ] **Mémoire des saisies** : reproposer la dernière valeur utilisée pour un
      `<param>` donné.
- [ ] **Annulation propre** au milieu du remplissage (Échap → on ne sort rien).

## P3 — robustesse du parsing

- [ ] **Commandes multi-lignes** : heredoc, SQL sur plusieurs lignes,
      continuations `\`. Aujourd'hui une ligne = un snippet.
- [ ] **Heuristique `  #`** : une commande qui contient elle-même `  #` casse la
      séparation desc/commande. Gérer les quotes, ou une syntaxe de desc explicite.
- [ ] **Langages** : décider quoi faire des blocs `sql`, `toml`… (les afficher en
      preview sans les proposer au lancement ?).

## P4 — convention côté fiches (à trancher avec le memento)

- [ ] **Placeholders vs exemples concrets** : la convention « valeurs d'exemple »
      est une règle de *sûreté* (pas de vraie valeur collable), pas une préférence
      pour le concret. Donc `<IP>` est OK, et même plus sûr. Décider : placeholder
      ce qui **varie par usage**, garder concret ce qui **illustre** (`chmod 600`).
- [ ] Éventuel mode « repérer les valeurs réservées » (10.10.10.5, johndoe) comme
      params candidats, sans toucher aux fiches.

## P5 — qualité & projet

- [ ] **Renommer le binaire** (collision avec l'init `runit`).
- [ ] **README** : quoi, pourquoi, install, le widget.
- [ ] **Tests d'intégration** sur un corpus fixture (dossier de fiches d'exemple).
- [ ] **CI** : `cargo fmt --check`, `clippy`, `test` (Forgejo Actions).

## Idées / plus loin

- [ ] **TUI natif** (`ratatui`) avec rendu markdown stylé dans le preview, quand
      fzf montrera ses limites.
- [ ] **Mode `--run`** : exécuter directement (avec confirmation) au lieu d'imprimer.
- [ ] Fonctionner sur **n'importe quelle base markdown**, pas que le memento.
