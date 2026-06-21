//! Shared instrument noise model, lifted from reactor.rs so every published
//! system reads back through the same sensor character: gentle multi-sine drift
//! when nominal, higher-frequency jitter as a system degrades. Instruments lie a
//! little, and they lie *more* when the machine is in trouble.

/// Cheap deterministic drift from sine waves at irrational frequencies.
/// Different seeds give different noise character.
fn sensor_noise(phase: f64, seed: f64, amplitude: f64) -> f64 {
    let n = (phase * 1.7 + seed).sin() * 0.5
        + (phase * 3.1 + seed * 2.3).sin() * 0.3
        + (phase * 7.9 + seed * 0.7).sin() * 0.2;
    n * amplitude
}

/// Higher-frequency jitter that appears during degraded conditions. More
/// erratic, suggesting instability.
fn instability_noise(phase: f64, seed: f64, severity: f64) -> f64 {
    let n = (phase * 13.7 + seed).sin() * 0.4
        + (phase * 23.1 + seed * 1.1).sin() * 0.3
        + (phase * 37.3 + seed * 3.7).sin() * 0.2
        + (phase * 59.0 + seed * 0.3).sin() * 0.1;
    n * severity
}

const BASE_NOISE: f64 = 0.002;
const INSTAB_GAIN: f64 = 0.02;

/// Apply sensor drift plus instability jitter to a true `value`. `instability`
/// is 0 for a healthy system and rises toward 1 as it degrades.
pub fn read(phase: f64, seed: f64, value: f64, instability: f64) -> f64 {
    value
        + sensor_noise(phase, seed, value * BASE_NOISE)
        + instability_noise(phase, seed, value * instability * INSTAB_GAIN)
}

/// A per-system noise phase accumulator. Advance it once per tick in
/// `*_physics`; sample through it in `*_publish`.
#[derive(Clone, Copy, Default)]
pub struct NoiseClock(f64);

impl NoiseClock {
    pub fn advance(&mut self, dt: f64) {
        self.0 += dt;
    }

    pub fn read(self, seed: f64, value: f64, instability: f64) -> f64 {
        read(self.0, seed, value, instability)
    }
}
