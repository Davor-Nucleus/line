<div align="center">
  <!-- <img src="public/logo.png" alt="Line Logo" width="200"/> -->

  # Line - Routeur audio WASAPI

  **Capture et redirige le son de sortie d'un périphérique audio Windows en temps réel.**

  [![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](#)
  [![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)](#)
</div>

---

Application Rust avec interface graphique permettant de capturer le son de sortie d'un périphérique audio Windows et de le rediriger vers un autre périphérique de sortie en temps réel.

## 📋 Sommaire

- [Fonctionnalités](#-fonctionnalités)
- [Prérequis](#-prérequis)
- [Installation](#-installation)
- [Utilisation](#-utilisation)
- [Architecture Technique](#-architecture-technique)
- [Limitations](#-limitations)
- [Dépannage](#-dépannage)

---

## 🎯 Fonctionnalités

- **Capture loopback** : Capture le son de sortie d'un périphérique audio Windows (mode loopback WASAPI).
- **Redirection audio** : Transfère le flux audio capturé vers un autre périphérique de sortie.
- **Interface graphique** : Interface utilisateur intuitive avec `egui` pour sélectionner les périphériques.
- **Temps réel** : Latence faible (~20-40 ms) grâce au mode événementiel WASAPI.
- **Multi-périphériques** : Support de tous les périphériques audio Windows disponibles.

---

## ⚡ Prérequis

> [!IMPORTANT]
> - **Windows** (WASAPI est spécifique à Windows)
> - **Rust** (version 1.70 ou supérieure)
> - **Cargo** (inclus avec Rust)

---

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
   *L'exécutable sera généré dans `target/release/line.exe`*

---

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

### Cas d'usage
- **Casque virtuel** : Capturer le son d'un périphérique (ex: haut-parleurs) et le rediriger vers un casque virtuel pour l'enregistrement ou le streaming.
- **Routage audio** : Rediriger le son d'une application spécifique vers un autre périphérique sans modifier les paramètres système.
- **Monitoring** : Écouter sur un casque le son qui sort normalement sur des haut-parleurs.

---

## 🏗️ Architecture Technique

<details>
<summary><b>Cliquez pour dérouler l'arborescence du projet</b></summary>

```text
line/
├── Cargo.toml          # Configuration du projet et dépendances
├── README.md           # Ce fichier
└── src/
    └── main.rs         # Code source principal
```
</details>

### Technologies utilisées
- **wasapi** (0.22) : Bindings Rust pour l'API WASAPI de Windows.
- **eframe/egui** (0.33.3) : Framework GUI multiplateforme.
- **anyhow** (1.x) : Gestion d'erreurs simplifiée.

### Fonctionnement Interne
1. **Initialisation COM** : Initialise COM (Component Object Model) pour WASAPI.
2. **Énumération des périphériques** : Liste tous les périphériques de rendu (Render) disponibles.
3. **Capture loopback** : Crée un client audio en mode loopback sur le périphérique source.
4. **Rendu audio** : Crée un client audio de rendu sur le périphérique de destination.
5. **Boucle de transfert** : Lit les données capturées et les écrit dans le buffer de sortie en temps réel.

### Mode de fonctionnement
- **Mode partagé** : Utilise le mode partagé WASAPI pour la compatibilité maximale.
- **Événementiel** : Mode événementiel pour une latence réduite (~20 ms de buffer).
- **Conversion automatique** : Conversion automatique du format audio si nécessaire.

---

## ⚠️ Limitations

- **Windows uniquement** : Cette application fonctionne uniquement sur Windows (WASAPI).
- **Latence** : Latence minimale d'environ 20-40 ms due au buffer audio.
- **Exclusivité** : Ne peut pas capturer en mode exclusif (seulement mode partagé).

---

## 🐛 Dépannage

- **Aucun périphérique détecté** : Vérifiez que vos périphériques audio sont correctement connectés et reconnus par Windows, puis redémarrez l'application.
- **Pas de son en sortie** :
  - Vérifiez que le périphérique de sortie sélectionné est actif et son volume suffisant.
  - Assurez-vous que le périphérique source produit bien du son.
- **Erreurs de compilation** :
  - Vérifiez votre installation de Rust : `rustc --version`
  - Mettez à jour Rust : `rustup update`
  - Vérifiez que vous êtes bien sous Windows.

---

## 📚 Ressources et Contribution

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou une pull request.

- [Documentation WASAPI](https://docs.microsoft.com/en-us/windows/win32/coreaudio/wasapi)
- [Documentation wasapi crate](https://docs.rs/wasapi/)
- [Documentation egui](https://docs.rs/egui/)

<div align="center">
  <i>Développé avec ❤️ en Rust - Libre d'utilisation et de modification</i>
</div>
