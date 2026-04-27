# Changelog

Tous les changements notables de ce projet sont documentés dans ce fichier.

Le format est basé sur [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
et ce projet adhère à [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2026-04-27

### Added
- Amélioration de la documentation et du style du README

## [0.2.0] - 2026-XX-XX

### Added
- **Safety**: Protection complète avec `#![forbid(unsafe_code)]`
- **Performance**: Optimisation agressive avec `opt-level = 3`
- **Tuning Dynamique** ✨: Mise à jour de Kp, Ki, Kd et du Setpoint à l'exécution sans réinitialisation

### Changed
- Compilation optimisée pour la vitesse maximale

## [0.1.5] - 2026-XX-XX

### Fixed
- **Anti-Derivative Kick**: Ajout du flag `first_run` pour éviter les pics de sortie au premier appel `update()`
- Le contrôleur initialise maintenant `last_measurement` avec la première valeur observée
- Démarrage très lisse pour les moteurs et actionneurs

### Added
- Support natif de `no_std` pour environnements bare-metal
- Robustesse native anti-windup

## [0.1.4] - 2026-XX-XX

### Added
- Feature `f32` pour une performance éclair sur les microcontrôleurs (RP2040, Pico, ESP32)

## Versions Antérieures

- **0.1.3** et antérieures: Voir l'historique Git pour les détails
