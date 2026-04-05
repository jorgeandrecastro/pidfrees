// Copyright (C) 2026 Jorge Andre Castro
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 or the License, or
// (at your option) any later version.
#![no_std] // We use no_std to ensure our library can be used in embedded environments without the Rust standard library.
//! 
//! A robust PID controller implementation with Anti-Windup and Derivative filtering.
//! Designed for reliability and protection against code privatization.

/// Core PID Controller structure.
/// Maintains internal state for integral and derivative calculations.
/// # Example
///
/// ```
/// 
/// // This example demonstrates how to create a PID controller and update it with a measurement.
/// use pidfrees::PidController;
/// 
/// let mut pid = PidController::new(1.0, 0.1, 0.01, 100.0, -100.0, 100.0);
/// let output = pid.update(50.0, 0.1);
/// assert!(output > 0.0);
/// ```
// This optionally uses f32 for reduced memory usage, but defaults to f64 for better precision.
#[cfg(feature = "f32")]
pub type Float = f32;
// By default, we use f64 for better precision in control calculations.
#[cfg(not(feature = "f32"))]
pub type Float = f64;

pub struct PidController {
    pub kp: Float,
    pub ki: Float,
    pub kd: Float,
    first_run: bool,// We can use this flag to handle the first update call for derivative calculation.
    setpoint: Float,
    integral: Float,
    last_measurement: Float,
    output_min: Float,
    output_max: Float,
}

impl PidController {
    /// Creates a new PID controller instance.
    /// 
    /// # Arguments
    /// * `kp`, `ki`, `kd` - PID gains.
    /// * `setpoint` - The target value to reach.
    /// * `min`, `max` - Hard limits for the output and integral term (Anti-Windup).
    pub fn new(kp: Float, ki: Float, kd: Float, setpoint: Float, min: Float, max: Float) -> Self {
        Self {
            kp,
            ki,
            kd,
            setpoint,
            integral: 0.0,
            last_measurement: 0.0,
            output_min: min,
            output_max: max,
            first_run: true,// We initialize the first_run flag to true to handle the first update call correctly.
        }
    }

    /// Computes the control output based on a new measurement.
    /// 
    /// # Arguments
    /// * `measurement` - The current process value (PV).
    /// * `dt` - Delta time since the last update.
    pub fn update(&mut self, measurement: Float, dt: Float) -> Float {
        // Prevent division by zero and handle invalid time steps
        if dt <= 0.0 {
            return 0.0;
        }

        // Handle the first run to initialize last_measurement without calculating a derivative kick.
        if self.first_run {
            self.last_measurement = measurement;
            self.first_run = false;
        }

        let error = self.setpoint - measurement;

        // 1. Proportional Term
        let p = self.kp * error;

        // 2. Integral Term with Anti-Windup protection
        self.integral += self.ki * error * dt;
        self.integral = self.integral.clamp(self.output_min, self.output_max);

        // 3. Derivative Term on Measurement
        // Avoids "derivative kicks" during setpoint changes.
        let d = -self.kd * (measurement - self.last_measurement) / dt;
        
        self.last_measurement = measurement;

        // Final Output calculation with global clamping
        (p + self.integral + d).clamp(self.output_min, self.output_max)
    }

    /// Updates the target setpoint dynamically.
    pub fn set_target(&mut self, setpoint: Float) {
        self.setpoint = setpoint;
    }

    /// Returns the current setpoint.
    pub fn get_target(&self) -> Float {
        self.setpoint
    }



    pub fn set_kp(&mut self, kp: Float) {
        self.kp = kp;
    }

    /// Updates the Integral gain at runtime.
    pub fn set_ki(&mut self, ki: Float) {
        self.ki = ki;
    }

    /// Updates the Derivative gain at runtime.
    pub fn set_kd(&mut self, kd: Float) {
        self.kd = kd;
    }
}

#[cfg(test)]
mod tests {
   // We need to import the standard library for testing purposes, as `no_std` is used for the main crate.
    extern crate std;
    use std::println;
    use super::*;

    #[test]
    fn test_pid_stability() {
        let mut pid = PidController::new(2.0, 0.5, 0.1, 50.0, 0.0, 100.0);
        let mut system_value = 0.0; 
        let dt = 0.1;

        for _ in 0..100 {
            let power = pid.update(system_value, dt);
            system_value += (power * 0.1) - (system_value * 0.05);
        }

        println!("Final system value: {}", system_value);
        
        assert!((50.0 - system_value).abs() < 2.0);
    }

    #[test]
    fn test_zero_dt_safety() {
        let mut pid = PidController::new(1.0, 1.0, 1.0, 100.0, -100.0, 100.0);
        let output = pid.update(50.0, 0.0);
        assert_eq!(output, 0.0);
    }

    #[test]
    fn test_dynamic_tuning() {
      let mut pid = PidController::new(1.0, 0.0, 0.0, 100.0, -100.0, 100.0);
      let out1 = pid.update(50.0, 0.1); // Erreur 50, Kp 1.0 -> Out 50
      assert_eq!(out1, 50.0);

      pid.set_kp(2.0); // On double le gain
      let out2 = pid.update(50.0, 0.1); // Erreur 50, Kp 2.0 -> Out 100
      assert_eq!(out2, 100.0);
}
}