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

## P1 — le widget, clé de voûte

Décision : runIt **vit à travers le widget** (pas d'usage « lancé seul » à
soigner). Une seule mécanique de widget débloque trois capacités d'un coup.

- [ ] **Widget zsh — insérer au curseur** : le raccourci insère la commande
      assemblée à la position du curseur (`LBUFFER`), il ne *remplace* pas la
      ligne. De cette seule règle découlent :
  - **saisie à la volée** — la commande atterrit dans le prompt, prête ;
  - **complétion de chemin par le shell (Philosophie 2)** — le `<param>` de type
    chemin est laissé vide, curseur posé dessus, et c'est **ton zsh** qui complète
    (Tab, `~`, globs, tes fonctions) ; on ne réimplémente pas la complétion ;
  - **composition de pipes** — comme on insère au curseur et que le widget est
    ré-invocable : `foo |` → widget → `grep …` s'insère → `|` → widget → … Rien
    de plus à coder, ça tombe de « insérer au curseur » + « ré-invocable ».
- [ ] **Protocole curseur/trou** : comment le widget sait où poser le curseur.
      Pistes : runIt émet `offset\tcommande`, ou un caractère sentinelle retiré par
      le widget. Curseur sur le 1er trou si présent, sinon en bout de ligne.
- [ ] **Garde-fou** : runIt insère du texte, **le shell possède le pipe**. Ne pas
      chercher à « comprendre » le pipeline (colonnes de l'étage n vers n+1…) —
      c'est le piège navi sous une autre forme.
- [ ] **Config de la racine** : `~/.config/runit/config` ou défaut, pour ne plus
      passer `RUNIT_ROOT` à chaque appel.
- [ ] **Erreurs claires** : fzf absent, `/dev/tty` indisponible, racine vide.

## P2 — remplissage des variables (le cœur différenciant)

**Trois types de trous, pas un.** Le type vient du **nom** du paramètre, déjà
écrit dans la fiche : aucune syntaxe nouvelle à inventer.

| type | exemples | traitement |
|---|---|---|
| **valeur** | `<seuil>`, `<col>`, `<n>`, `<ip>` | substitution brute, ça suffit |
| **syntaxe** | `<sep>`, `<delim>` | liste fermée → menu (`,` `;` `\t` `:` …) |
| **programme / motif** | `<motif>`, `<regex>`, `<remplacement>` | pas de remplissage : trou vide, curseur posé dessus |

La règle qui en découle :

> **runIt ne remplit pas ce qu'il ne saurait pas citer — il y pose le curseur.**

C'est la Philosophie 2 (P1) étendue des chemins aux regex. Elle tient parce
qu'on **insère** au lieu d'exécuter : une commande mal citée se voit avant
Entrée. navi fait la même substitution brute que nous — le terrain est libre.

Constaté sur les fiches sed/awk. L'extraction des `<param>`, elle, tient déjà
(`awk 'NR>=10 && NR<=20'` et `awk '$3 > 100'` ne produisent aucun faux positif :
c'est la validation du nom en `alnum|_|-` qui filtre). Ce qui casse, c'est la
valeur injectée dans un contexte qui a sa propre syntaxe :

```
sed 's/<motif>/…/g'  + /usr/local     → sed: unknown option to `s'
awk '/<motif>/ …'    + erreur d'accès → sh: Unterminated quoted string
sed 's/<motif>/[&]/' + a              → & = tout le match, erreur SILENCIEUSE
```

- [ ] **Typer le trou d'après son nom** : table nom → type (`motif`, `regex`,
      `remplacement` → programme ; `sep`, `delim` → syntaxe ; le reste → valeur).
      Zéro syntaxe nouvelle dans les fiches ; en contrepartie ça oblige à **nommer
      juste** côté memento — ce qu'on veut de toute façon (cf. P4).
- [ ] **Suggestions pour les variables énumérables** (type *syntaxe*) : `<sep>`,
      `<niveau>`… → menu d'une liste fixe. Les variables de type **chemin** ne sont
      PAS ici : elles passent par la complétion du shell (Ph. 2, cf. P1), pas par
      un menu `fd`.
- [ ] **Trous de programme laissés vides** (type *programme*) : ne rien demander,
      sortir le trou vide et laisser le widget y poser le curseur — même protocole
      curseur/trou qu'en P1.
- [ ] **Échappement minimal et déterministe** : savoir si le trou est à l'intérieur
      d'un `'…'` (scan des quotes jusqu'à l'offset) et, si oui, transformer `'` en
      `'\''` dans la valeur saisie. ~15 lignes, règle le cas awk proprement. Le
      délimiteur de `sed s///` n'est **pas** du code : c'est une convention de fiche
      (`s|<motif>|<rempl>|`, déjà documentée dans `fiches/shell/sed.md`).
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
