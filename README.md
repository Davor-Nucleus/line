# Line - Routeur audio WASAPI

Application Rust avec interface graphique permettant de capturer le son de sortie d'un périphérique audio Windows et de le rediriger vers un autre périphérique de sortie en temps réel.

## 🎯 Fonctionnalités

- **Capture loopback** : Capture le son de sortie d'un périphérique audio Windows (mode loopback WASAPI)
- **Redirection audio** : Transfère le flux audio capturé vers un autre périphérique de sortie
- **Interface graphique** : Interface utilisateur intuitive avec `egui` pour sélectionner les périphériques
- **Temps réel** : Latence faible (~20-40 ms) grâce au mode événementiel WASAPI
- **Multi-périphériques** : Support de tous les périphériques audio Windows disponibles

## 📋 Prérequis

- **Windows** (WASAPI est spécifique à Windows)
- **Rust** (version 1.70 ou supérieure)
- **Cargo** (inclus avec Rust)

## 🚀 Installation

1. Clonez le dépôt ou téléchargez le code source :
```bash
git clone <url-du-repo>
cd line
```

2. Compilez le projet :
```bash
cargo build --release
```

L'exécutable sera généré dans `target/release/line.exe`

## 💻 Utilisation

### Lancement

```bash
cargo run --release
```

Ou directement l'exécutable :
```bash
target/release/line.exe
```

### Interface utilisateur

1. **Sélection du périphérique source** : Choisissez dans la liste déroulante le périphérique de lecture dont vous souhaitez capturer la sortie audio.

2. **Sélection du périphérique de destination** : Choisissez dans la deuxième liste déroulante vers quel périphérique de sortie vous voulez rediriger le son. Le périphérique par défaut est présélectionné.

3. **Démarrer la capture** : Cliquez sur le bouton "Démarrer la capture" pour commencer la redirection audio.

4. **Arrêter la capture** : Cliquez sur "Arrêter" pour interrompre la capture et la redirection.

### Exemple d'utilisation

- **Casque virtuel** : Capturer le son d'un périphérique (ex: haut-parleurs) et le rediriger vers un casque virtuel pour l'enregistrement ou le streaming.
- **Routage audio** : Rediriger le son d'une application spécifique vers un autre périphérique sans modifier les paramètres système.
- **Monitoring** : Écouter sur un casque le son qui sort normalement sur des haut-parleurs.

## 🔧 Architecture technique

### Technologies utilisées

- **wasapi** (0.22) : Bindings Rust pour l'API WASAPI de Windows
- **eframe/egui** (0.33.3) : Framework GUI multiplateforme
- **anyhow** (1.x) : Gestion d'erreurs simplifiée

### Fonctionnement

1. **Initialisation COM** : Initialise COM (Component Object Model) pour WASAPI
2. **Énumération des périphériques** : Liste tous les périphériques de rendu (Render) disponibles
3. **Capture loopback** : Crée un client audio en mode loopback sur le périphérique source
4. **Rendu audio** : Crée un client audio de rendu sur le périphérique de destination
5. **Boucle de transfert** : Lit les données capturées et les écrit dans le buffer de sortie en temps réel

### Mode de fonctionnement

- **Mode partagé** : Utilise le mode partagé WASAPI pour la compatibilité maximale
- **Événementiel** : Mode événementiel pour une latence réduite (~20 ms de buffer)
- **Conversion automatique** : Conversion automatique du format audio si nécessaire

## 📝 Structure du projet

```
line/
├── Cargo.toml          # Configuration du projet et dépendances
├── README.md           # Ce fichier
└── src/
    └── main.rs         # Code source principal
```

## ⚠️ Limitations

- **Windows uniquement** : Cette application fonctionne uniquement sur Windows (WASAPI)
- **Latence** : Latence minimale d'environ 20-40 ms due au buffer audio
- **Exclusivité** : Ne peut pas capturer en mode exclusif (seulement mode partagé)

## 🐛 Dépannage

### Aucun périphérique détecté
- Vérifiez que vos périphériques audio sont correctement connectés et reconnus par Windows
- Redémarrez l'application

### Pas de son en sortie
- Vérifiez que le périphérique de sortie sélectionné est actif
- Vérifiez le volume du périphérique dans les paramètres Windows
- Assurez-vous que le périphérique source produit bien du son

### Erreurs de compilation
- Assurez-vous d'avoir Rust installé : `rustc --version`
- Mettez à jour Rust : `rustup update`
- Vérifiez que vous êtes sur Windows

## 📄 Licence

Ce projet est fourni tel quel, sans garantie. Libre d'utilisation et de modification.

## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou une pull request.

## 📚 Ressources

- [Documentation WASAPI](https://docs.microsoft.com/en-us/windows/win32/coreaudio/wasapi)
- [Documentation wasapi crate](https://docs.rs/wasapi/)
- [Documentation egui](https://docs.rs/egui/)
