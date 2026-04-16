# Pidfrees v 0.2.0

- LA Version 0.2.0 , introduit le safety avec #![forbid(unsafe_code)] et une optimisation pour la rapiditée avec opt-level = 3  . 

- La  Version 0.1.5 introduit le  Le Correctif "Démarrage Lisse"
Un Contrôleur PID Robuste et Haute Performance pour Rust (std & no_std).
Optimisé pour les systèmes embarqués comme RP2040 (Pico), Pico 2, STM32 et ESP32.

# 🛡️ La Mission
pidfrees fournit une logique de contrôle robuste tout en restant strictement Open Source. Cette crate utilise la licence GPL-2.0-or-later pour assurer que les algorithmes de contrôle essentiels restent un bien commun et ne soient jamais privatisés.

# 🚀 Caractéristiques Principales
Précision Adaptative : Choisissez entre f64 pour une haute précision (Cloud/PC) ou f32 pour une performance éclair sur les microcontrôleurs.

Tuning Dynamique (Nouveau ! ✨) : Mettez à jour Kp, Ki, Kd et le Setpoint à l'exécution sans réinitialiser le contrôleur.

📋 Journal des Modifications & Mises à Jour
🦅 Version 0.1.5  Le Correctif "Démarrage Lisse"
Correction : Anti-Derivative Kick : Un flag de synchronisation first_run a été ajouté.

Pourquoi ? Auparavant, le premier appel update() pouvait causer un énorme pic de sortie si la mesure initiale était loin de zéro. Maintenant, le contrôleur initialise last_measurement avec la première valeur qu'il voit, assurant un démarrage très lisse pour vos moteurs et actionneurs.

Crédit : Un grand merci à la communauté (Luc E.Brunet) pour vos retours !

no_std Natif : Construit pour les environnements bare-metal comme Embassy, RTIC ou des noyaux personnalisés.

Robustesse : Protection native Anti-Windup et dérivée basée sur la mesure (PV) pour éviter les "derivative kicks" lors des changements de setpoint.

Abstraction Zéro-Coût : Zéro surcharge binaire indépendamment de la précision choisie.

🛠️ Utilisation
Standard (f64 par défaut)
Ini, TOML
[dependencies]
pidfrees = "0.2.0"
Ultra-Rapide (f32 pour RP2040 / Pico / ESP32)
Ini, TOML
[dependencies]
pidfrees = { version = "0.1.4", features = ["f32"] }
💡 Démarrage Rapide
````rust
use pidfrees::PidController;

// 1. Initialisation : Kp, Ki, Kd, Cible, Min_Sortie, Max_Sortie
let mut pid = PidController::new(2.0, 0.5, 0.1, 50.0, 0.0, 100.0);

// 2. Calculer la sortie
let power = pid.update(45.0, 0.1); 

// 3. Tuning Dynamique (Nouveau en 0.1.4)
// Changez les gains à la volée si l'état de votre système change !
pid.set_kp(1.5);
pid.set_target(60.0);
🎮 Exemple : Planification des Gains
Rust
// Si votre robot porte une charge lourde, vous pourriez avoir besoin de gains plus agressifs
if robot.is_carrying_load() {
    pid.set_kp(3.5); // Augmentez le gain proportionnel
    pid.set_ki(0.8); // Augmentez l'intégral pour compenser la gravité
} else {
    pid.set_kp(2.0); // Retour à la normale
}

````
# ⚖️ Licence
Ce projet est licencié sous la Licence Publique Générale GNU v2.0 ou ultérieure.
Vous êtes libre de l'utiliser, mais toute modification ou amélioration DOIT être partagée avec la communauté. C'est le contrat de la liberté.

Créé par Jorge Andre Castro.

# Pourquoi mettre à jour ?
Parce que dans les systèmes embarqués, chaque cycle CPU et chaque octet de RAM compte. Avec la fonctionnalité f32 et le Tuning Dynamique, pidfrees est l'un des outils les plus légers et les plus flexibles disponibles pour dompter vos moteurs et capteurs sans sacrifier la sécurité.
