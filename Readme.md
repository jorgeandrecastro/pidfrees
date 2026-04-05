🦅 pidfrees version 0.1.4
A Robust, High-Performance PID Controller for Rust (std & no_std).
Optimized for embedded systems like RP2040 (Pico), Pico 2, STM32, and ESP32.

Created by Jorge Andre Castro.

🛡️ The Mission
pidfrees provides a Robust control logic while remaining strictly Open Source. This crate uses the GPL-2.0-or-later license to ensure that essential control algorithms remain a common good and are never privatized.

🚀 Key Features
Adaptive Precision: Choose between f64 for high precision (Cloud/PC) or f32 for blazing-fast performance on microcontrollers.

Dynamic Tuning (New! ✨): Update Kp, Ki, Kd, and Setpoint at runtime without re-initializing the controller.

no_std Native: Built for bare-metal environments like Embassy, RTIC, or custom kernels.

Industrial Robustness: Native Anti-Windup protection and derivative based on measurement (PV) to avoid "derivative kicks" during setpoint changes.

Zero-Cost Abstraction: Zero binary overhead regardless of the chosen precision.

🛠️ Usage
Standard (f64 by default)
Ini, TOML
[dependencies]
pidfrees = "0.1.4"
Ultra-Fast (f32 for RP2040 / Pico / ESP32)
Ini, TOML
[dependencies]
pidfrees = { version = "0.1.4", features = ["f32"] }
💡 Quick Start
Rust
use pidfrees::PidController;

// 1. Initialisation: Kp, Ki, Kd, Target, Min_Out, Max_Out
let mut pid = PidController::new(2.0, 0.5, 0.1, 50.0, 0.0, 100.0);

// 2. Compute output
let power = pid.update(45.0, 0.1); 

// 3. Dynamic Tuning (New in 0.1.4)
// Change gains on the fly if your system state changes!
pid.set_kp(1.5);
pid.set_target(60.0);
🎮 Example: Gain Scheduling
Rust
// If your robot picks up a heavy load, you might need more aggressive gains
if robot.is_carrying_load() {
    pid.set_kp(3.5); // Increase proportional gain
    pid.set_ki(0.8); // Increase integral to counter gravity
} else {
    pid.set_kp(2.0); // Back to normal
}
⚖️ License
This project is licensed under the GNU General Public License v2.0 or later.
You are free to use it, but any modification or improvement MUST be shared with the community. That’s the contract of freedom.

🦅 Why upgrade?
Because in embedded systems, every CPU cycle and every byte of RAM counts. With the f32 feature and Dynamic Tuning, pidfrees is one of the lightest and most flexible tools available to tame your motors and sensors without sacrificing safety.