🦅 pidfrees version 0.1.3
A Robust,  PID Controller for Rust std and no std  embedded systems pico 2040 or pico2.

Created by Jorge Andre Castro.

🛡️ The Mission
pidfrees provides industrial-grade control logic while remaining strictly Open Source. This crate uses the GPL-2.0-or-later license to ensure that essential control algorithms remain a common good and are never privatized.

🚀 Key Features

Adaptive Precision (New!): Choose between f64 for high precision (Cloud/PC) or f32 for blazing-fast performance on microcontrollers.

no_std Native: Built for bare-metal  like embassy and pure embedded systems (RP2040, STM32, ESP32, pico2).

Industrial Robustness: Native Anti-Windup protection and derivative based on measurement (PV) to avoid shocks during setpoint changes.

Zero-Cost Abstraction: The use of type aliases (Float) guarantees zero binary overhead, regardless of the chosen precision.

🛠️ Usage

Standard (f64 by default)
Ideal for servers, complex simulations, or systems with double-precision FPU.

[dependencies]
pidfrees = "0.1.3"

Ultra-Fast (f32 for RP2040 / Pico)
Optimized for processors without an FPU or when every CPU cycle matters.

[dependencies]
pidfrees = { version = "0.1.3", features = ["f32"] }

💡 Quick Start

use pidfrees::PidController;

// Create your PID: Kp, Ki, Kd, Target, Min, Max
// The type (f32 or f64) automatically adapts to your configuration!
let mut pid = PidController::new(2.0, 0.5, 0.1, 50.0, 0.0, 100.0);

// Update with current measurement and delta time
let power = pid.update(45.0, 0.1);

⚖️ License
This project is licensed under the GNU General Public License v2.0 or later.
You are free to use it, but any modification or improvement MUST be shared with the community. That’s the contract of freedom.

🦅 Why upgrade to 0.1.3?
Because in embedded systems, every cycle counts. With the f32 feature, pidfrees becomes one of the lightest tools available to tame your motors without sacrificing safety.