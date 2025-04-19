# CriteriGen

> Générateur automatique de tests unitaires Criterion pour des fonctions écrites en C.

CriteriGen est un outil en ligne de commande développé en Rust, capable de recevoir une fonction en langage C en entrée (via un fichier ou un prompt interactif), puis de générer automatiquement des tests unitaires au format [Criterion](https://github.com/Snaipe/Criterion). Il permet de gagner du temps lors de la phase de test et de garantir une meilleure couverture du code C.

---

## 🚀 Fonctionnalités

- 🔍 Analyse de la signature de la fonction C.
- 🧠 Compréhension des types primitifs (int, float, char, etc.).
- 🧪 Génération automatique de cas de test simples au format Criterion.
- 📂 Lecture depuis un fichier `.c` ou saisie directe via le terminal.
- 📄 Export des tests dans un fichier `.c` prêt à être compilé avec Criterion.

---

## 🔧 Installation

Assurez-vous d’avoir [Rust](https://www.rust-lang.org/tools/install) installé.

```bash
git clone https://github.com/votre-utilisateur/CriteriGen.git
cd CriteriGen
cargo build --release

Le binaire sera disponible dans target/release/criterigen.
🖥️ Utilisation
Lancer le programme :

./criterigen

Vous serez invité à :

    Saisir une fonction C directement dans le terminal, ou

    Fournir le chemin vers un fichier .c contenant la fonction.

Exemple de saisie directe :

int add(int a, int b) {
    return a + b;
}

Le programme générera ensuite un fichier de test du style :

Test(add_function, basic_addition) {
    cr_assert_eq(add(2, 3), 5);
}
```

📁 Structure du projet

CriteriGen/
├── src/
│   ├── main.rs         # Entrée principale
│   ├── parser.rs       # Analyse de la fonction C
│   ├── generator.rs    # Génération des tests Criterion
│   └── templates/      # Templates des fichiers de test
├── tests/              # Tests du générateur
├── examples/           # Fonctions C d’exemple
├── Cargo.toml
└── README.md

📦 Dépendances principales

    clang – Binding à libclang pour analyser le code C.

    tera – Moteur de templates pour générer les fichiers de test.

    clap – Interface en ligne de commande ergonomique.

💡 Idées futures

    Génération de mocks pour les fonctions appelées.

    Support des pointeurs et structures plus complexes.

    Interface web ou GUI (bonus).

    Export en plusieurs frameworks de test (Unity, CuTest...).

📝 Licence

Ce projet est sous licence MIT. Voir LICENSE pour plus de détails.
🙌 Contribuer

Les contributions sont les bienvenues ! Que ce soit pour corriger un bug, proposer une nouvelle fonctionnalité ou améliorer la documentation.
📫 Contact
