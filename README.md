# runIt

Fuzzy-pick une commande depuis un dossier de fiches **markdown**, remplis ses
`<paramètres>`, récupère-la prête à lancer.

La fiche reste la **source unique** : pas de fichier `.cheat` à maintenir à côté,
runIt lit le markdown directement. Et contrairement à un gestionnaire de
cheatsheets classique, il garde **le détail de la fiche derrière la commande** :
au moment de choisir, le volet de preview affiche la section de fiche autour de
la ligne — le contexte et la saisie au même endroit.

## Comment ça marche

1. runIt parcourt tes fiches et indexe chaque **ligne** de bloc ` ```sh ` comme
   une commande. Le commentaire de fin de ligne (`commande   # description`) sert
   de libellé.
2. `fzf` te laisse chercher par description ; le volet de droite montre la fiche.
3. Les `<param>` de la ligne choisie te sont demandés un par un (les `$VAR`, elles,
   sont laissées au shell).
4. La commande assemblée sort sur `stdout`.

## Installer

```sh
cargo build --release
# le binaire : target/release/runit
```

Dépend de [`fzf`](https://github.com/junegunn/fzf) (dans le PATH).

## Utiliser

```sh
# indiquer où sont les fiches, puis lancer
RUNIT_ROOT=~/documents/memento/fiches runit
```

`RUNIT_ROOT` peut aussi être passé en premier argument ; à défaut, runIt cherche
`./fiches`.

## Format attendu des fiches

- des blocs ` ```sh ` / ` ```bash ` contenant **une commande par ligne** ;
- une description optionnelle après `#`, séparée par **deux espaces ou plus** ;
- un `<paramètre>` là où une valeur change à chaque usage (`ssh root@<ip>`) —
  jamais une vraie valeur collable ni un `$VAR` d'environnement.

```sh
awk -F'<sep>' '{print substr($<col>,1,<n>)}' <fichier>   # extraire une colonne
```

## État

MVP fonctionnel : parse, choix avec preview, remplissage, sortie sur stdout.
Il **imprime** la commande — l'insertion directe dans le prompt (widget shell)
et les suggestions de variable (picker de fichier) sont au [backlog](BACKLOG.md).

## Licence

Projet perso, à définir.
