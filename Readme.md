[![Docs.rs](https://docs.rs/pidfrees/badge.svg)](https://docs.rs/pidfrees/)
[![Crates.io](https://img.shields.io/crates/v/pidfrees.svg)](https://crates.io/crates/pidfrees)
[![License: GPL-2.0-or-later](https://img.shields.io/badge/License-GPL--2.0--or--later-blue.svg)](LICENSE)

# Pidfrees

Un contrôleur PID **robuste**, **haute performance** et **sécurisé** pour Rust (std & no_std).

Optimisé pour les systèmes embarqués comme **RP2040** (Pico), **Pico 2**, **STM32** et **ESP32**.

## 🛡️ La Mission

Pidfrees fournit une logique de contrôle robuste tout en restant strictement Open Source. Cette crate utilise la licence **GPL-2.0-or-later** pour assurer que les algorithmes de contrôle essentiels restent un bien commun et ne soient jamais privatisés.

## ✨ Caractéristiques Principales

- **Précision Adaptative** : Choisissez entre `f64` pour une haute précision (Cloud/PC) ou `f32` pour une performance éclair sur les microcontrôleurs
- **Tuning Dynamique** : Mettez à jour Kp, Ki, Kd et le Setpoint à l'exécution sans réinitialiser le contrôleur
- **Native no_std** : Construit pour les environnements bare-metal (Embassy, RTIC, noyaux personnalisés)
- **Robustesse Garantie** : Protection anti-windup native et dérivée basée sur la mesure (PV) pour éviter les "derivative kicks"
- **Zéro Surcharge** : Abstraction zéro-coût, pas d'overhead binaire indépendamment de la précision choisie
- **Sécurité Totale** : `#![forbid(unsafe_code)]` pour une confiance absolue

## 📦 Installation

### Standard (f64 par défaut)

```toml
[dependencies]
pidfrees = "0.2.1"
```

### Ultra-Rapide (f32 pour RP2040 / Pico / ESP32)

```toml
[dependencies]
pidfrees = { version = "0.2.1", features = ["f32"] }
```

## 💡 Démarrage Rapide

```rust
use pidfrees::PidController;

// 1. Initialisation : Kp, Ki, Kd, Cible, Min_Sortie, Max_Sortie
let mut pid = PidController::new(2.0, 0.5, 0.1, 50.0, 0.0, 100.0);

// 2. Calculer la sortie
let power = pid.update(45.0, 0.1);

// 3. Tuning Dynamique - Changez les gains à la volée
pid.set_kp(1.5);
pid.set_target(60.0);
```

### 🎮 Exemple Avancé : Planification des Gains

```rust
// Si votre robot porte une charge lourde, ajustez dynamiquement les gains
if robot.is_carrying_load() {
    pid.set_kp(3.5);  // Augmentez le gain proportionnel
    pid.set_ki(0.8);  // Augmentez l'intégral pour compenser la gravité
} else {
    pid.set_kp(2.0);  // Retour à la normale
    pid.set_ki(0.5);
}
```

## 📚 Documentation Complète

Consultez la [documentation officielle](https://docs.rs/pidfrees/) pour des exemples détaillés et une API complète.

## ⚖️ Licence

Ce projet est licencié sous la **Licence Publique Générale GNU v2.0 ou ultérieure** ([GPL-2.0-or-later](LICENSE)).

Vous êtes libre d'utiliser ce code, mais toute modification ou amélioration **DOIT être partagée** avec la communauté. C'est le contrat de la liberté logicielle.

## 📝 Historique des Changements

Pour voir l'historique complet des versions, consultez le [CHANGELOG](CHANGELOG.md).

## 👨‍💻 Auteur

Créé par **Jorge Andre Castro**

---

### Pourquoi Pidfrees ?

Dans les systèmes embarqués, chaque cycle CPU et chaque octet de RAM comptent. Avec la fonctionnalité `f32` et le tuning dynamique, pidfrees est l'un des outils les plus **légers**, **flexibles** et **sécurisés** disponibles pour contrôler vos moteurs et capteurs sans sacrifier la sécurité.
