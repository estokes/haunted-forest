//! The fighter's lift fan and the drag it cruises against.
//!
//! The lift fan is ducted, with its exit the size of its disk; its ideal
//! power is T^1.5 / sqrt(4 rho A). Its efficiency against that ideal is
//! calibrated on the F-35B's lift fan: 89 kN from a 1.27 m fan for
//! ~21.6 MW (29,000 shp).

use std::f64::consts::PI;

pub const RHO_SL: f64 = 1.225;

fn ideal_power(thrust: f64, diameter: f64, rho: f64) -> f64 {
    thrust.powf(1.5) / (4. * rho * PI * diameter * diameter / 4.).sqrt()
}

/// The F-35B lift fan's power over the ducted ideal.
pub fn calibrated_efficiency() -> f64 {
    ideal_power(89e3, 1.27, RHO_SL) / 21.6e6
}

/// Shaft power, W, and exit jet speed, m/s, for a lift fan of `diameter`
/// making `thrust` at sea level.
pub fn lift_fan(thrust: f64, diameter: f64) -> (f64, f64) {
    let area = PI * diameter * diameter / 4.;
    (
        ideal_power(thrust, diameter, RHO_SL) / calibrated_efficiency(),
        (thrust / (RHO_SL * area)).sqrt(),
    )
}

/// A Mirage 2000-like delta: 41 m2 of wing, aspect ratio ~2. Zero-lift
/// drag is 0.022 subsonic; supersonic it is skin friction plus wave drag
/// falling as 1/sqrt(M^2 - 1), ~0.029 at Mach 2. Drag due to lift is the
/// subsonic polar or linear supersonic theory, whichever is larger.
pub struct Airframe {
    pub wing: f64,
    pub aspect: f64,
}

impl Default for Airframe {
    fn default() -> Self {
        Self { wing: 41., aspect: 2.03 }
    }
}

impl Airframe {
    pub fn cd0(&self, mach: f64) -> f64 {
        if mach < 1.1 {
            0.022
        } else {
            0.017 + 0.02 / (mach * mach - 1.).sqrt()
        }
    }

    /// Drag, N, in level flight at mass `kg` and Mach `mach` at altitude `z`.
    pub fn drag(&self, kg: f64, z: f64, mach: f64) -> f64 {
        let p = crate::atmos::isa(z).1;
        let q = 0.7 * p * mach * mach;
        let cl = kg * 9.80665 / (q * self.wing);
        let k_sub = 1. / (PI * 0.8 * self.aspect);
        let k = if mach > 1. { k_sub.max((mach * mach - 1.).sqrt() / 4.) } else { k_sub };
        q * self.wing * (self.cd0(mach) + k * cl * cl)
    }
}
