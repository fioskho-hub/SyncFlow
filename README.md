# SyncFlow ⚡

SyncFlow est un framework de synchronisation **offline-first** open-source écrit en Rust. Il permet la réplication de données distribuées et résiliente grâce aux structures de données CRDT.

## 🛠️ Architecture du Monorepo

- **`crates/core`** : Types de données fondamentaux (CRDT LWW-Register, Documents).
- **`crates/storage`** : Moteur de stockage (Traits d'abstraction & implémentation mémoire/SQLite).
- **`crates/sync`** : Protocole d'échange et moteur de synchronisation inter-nœuds.
- **`crates/crypto`** : Couche de chiffrement symétrique End-to-End (E2EE).
- **`examples/`** : Démonstrations et scénarios d'utilisation.

## 🚀 Démarrage Rapide

### Prérequis
- Rust 1.80+ (toolchain GNU sous Windows recommendée)

### Lancer la suite de tests
```bash
cargo test --workspace