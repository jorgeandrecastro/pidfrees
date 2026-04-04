# 🦅 pidfrees v0.1.2
**A Robust, High-Precision PID Controller for Rust.**

Created by **Jorge Andre Castro**.

---

## 🛡️ The Mission
`pidfreeS` is designed to provide industrial-grade control logic while remaining strictly **Open Source**. 
This crate is built to prevent the privatization of essential control techniques by using the **GPL-2.0-or-later** license.

## 🚀 Key Features
* **no_std Compatible**: Designed for bare-metal kernels (like JC-OS) and embedded systems.
* **Industrial Robustness**: Integrated **Anti-Windup** (clamping) and **Derivative filtering** (PV-based).
* **Ultra-Optimized**: Configured for minimal binary size (`opt-level = "z"`) and zero debug symbols.
* **Thread-Safe**: Implements `Send` and `Sync` for multi-threaded robotics.

## 🛠️ Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
pidfrees = "0.1.2"
Quick Start
Rust
use pidfrees::PidController;

// New PID: Kp, Ki, Kd, Target, Min_Output, Max_Output
let mut pid = PidController::new(2.0, 0.5, 0.1, 50.0, 0.0, 100.0);

// Update with current measurement and delta time
let output = pid.update(45.0, 0.1);
⚖️ License
This project is licensed under the GNU General Public License v2.0 or later.
You are free to use it, but you MUST share any modifications or derivative works.