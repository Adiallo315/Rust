# <p align="center">Générateur de mot de passe en Rust</p>
  
Ce projet est un générateur de mots de passe en ligne de commande écrit en Rust. Il permet de générer des mots de passe sécurisés avec des options pour spécifier la longueur, le nombre de mots de passe à générer, et de vérifier la sécurité des mots de passe générés. De plus, les mots de passe peuvent être sauvegardés dans un fichier avec des informations supplémentaires comme la date et l'heure de génération.


## 🧐 Fonctionnalités principales : 
<<<<<<< Adiallo315-patch-1
- Génération de mots de passe aléatoires avec une longueur spécifiée.
- Possibilité de générer plusieurs mots de passe à la fois.
- Évaluation de la sécurité des mots de passe générés.
- Sauvegarde des mots de passe générés dans un fichier avec des informations supplémentaires.

## Prérequis
- Rust (https://www.rust-lang.org/)


## Installation

1. Clonez le dépôt :

   ```sh
   git clone git@github.com:Adiallo315/Rust.git

2. Accédez au répertoire du projet:
   ```sh
   cd generateur_mdp
=======

- Génération de mots de passe aléatoires
>>>>>>> main

3. Compilez le projet :
   ```sh
   cargo build
    ```


<<<<<<< Adiallo315-patch-1
  

=======
- Affichage et stockage 

## Contenu
- un dossier src dans lequel se trouve le main.rs, qui est  le corps de notre porgramme

- un dossier git ignore dans lequel se trouve le dossier target
  
- un fichier Cargo.lock  est un fichier généré par Cargo, le gestionnaire de paquets et de builds pour Rust. Il est utilisé pour garantir que les dépendances d'un projet Rust sont déterministes et reproductibles.qui est le gestionnaire de paquets et de builds pour Rust.

- un fichier Cargo.toml est un fichier de configuration utilisé par Cargo, le gestionnaire de paquets et de builds pour le langage de programmation Rust. Ce fichier définit les métadonnées du projet, les dépendances, les configurations de build, et d'autres paramètres importants pour le projet Rust.

- un fichier README.md qui décris le repot et explique comment utiliser le programme

- generated_password.txt est un fichier texte qui va stocker tous les mots passe qui vont être généré pas l'utilisateur
>>>>>>> main


## 🛠️Utilisation
  
<<<<<<< Adiallo315-patch-1
  Pour exécuter le générateur de mots de passe, utilisez la commande suivante :
  cargo run

  Vous serez invité à entrer la longueur des mots de passe et le nombre de mots de passe à générer. Ensuite, vous aurez la possibilité de sauvegarder les mots de passe générés dans un fichier.

### Exemple

 ```sh
$ cargo run
Entrez la longueur du mot de passe: 12
Entrez le nombre de mots de passe à générer: 5

Mot de passe 1: Abc123!@#
Mot de passe 2: Def456$%^
Mot de passe 3: Ghi789&*(
Mot de passe 4: Jkl012)(_
Mot de passe 5: Mno345_+-

Les mots de passe ont été sauvegardés dans le fichier passwords.txt
 ```
=======
copier le repot vers votre machine en local et déplacer vous dans le dossier puis lancez la commande ci-dessous:
    
```bash
Cargo run
```
>>>>>>> main
Attention : il est important d'avoir rust installé sur la machine 
