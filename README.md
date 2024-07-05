# <p align="center">Générateur de mot de passe en Rust</p>
  
Ce projet est un générateur de mots de passe en ligne de commande écrit en Rust. Il permet de générer des mots de passe sécurisés avec des options pour spécifier la longueur, le nombre de mots de passe à générer, et de vérifier la sécurité des mots de passe générés. De plus, les mots de passe peuvent être sauvegardés dans un fichier avec des informations supplémentaires comme la date et l'heure de génération.


## 🧐 Fonctionnalités principales : 
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

3. Compilez le projet :
   ```sh
   cargo build
    ```


  



## 🛠️Utilisation
  
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
Attention : il est important d'avoir rust installé sur la machine 
