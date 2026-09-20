# SyncFlow ⚡

SyncFlow est un framework de synchronisation **offline-first** open-source écrit en Rust. Il permet la réplication de données distribuées et résiliente grâce aux structures de données CRDT (*Conflict-free Replicated Data Types*).

---

## 🛠️ Architecture du Monorepo

Le projet est organisé sous forme de workspace Cargo pour une modularité maximale :

- **`crates/core`** : Types de données fondamentaux, structures CRDT (dont le `LwwRegister`) et gestion des documents.
- **`crates/storage`** : Moteur de stockage local (interface `StorageEngine` avec implémentations mémoire et SQLite).
- **`crates/sync`** : Protocole d'échange, moteurs de synchronisation (`SyncEngine`) et transports réseau (ex: WebSockets).
- **`crates/crypto`** : Couche de chiffrement symétrique End-to-End (E2EE) pour sécuriser les données transmises.
- **`examples/`** : Démonstrations et scénarios d'utilisation complets.

---

## 🚀 Démarrage Rapide

### Prérequis
- **Rust 1.80+** (toolchain GNU recommandée sous Windows)

### Installation
Cloche le dépôt et compile l'ensemble du projet :

```bash
git clone [https://github.com/votre-compte/SyncFlow.git](https://github.com/votre-compte/SyncFlow.git)
cd SyncFlow
cargo build