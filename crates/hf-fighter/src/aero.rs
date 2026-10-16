//! The fighter's aerodynamics, Mirage 2000-like: 45 m2 of delta wing,
//! aspect ratio ~1.9, with bays in its wing roots. Below Mach 5 a drag polar: zero-
//! lift drag 0.022 subsonic and 0.017 of skin friction supersonic, plus
//! wave drag; and drag due to lift from the subsonic polar or linear
//! supersonic theory, whichever is larger. Hypersonic, modified Newtonian
//! flow on the lower surface at the angle of attack.
//!
//! The wave drag is the airframe's own: its Mach-1 D/q, from its area
//! distribution (modeling/fighter.py), rising through the transonic from
//! nothing at Mach 0.9 to half at Mach 1 and all of it by 1.05; falling
//! off with Mach as Raymer's fit for a wing of this sweep has it, to Mach
//! 2.5, the edge of the aircraft it was fitted to; and beyond that as
//! linear theory has it, with 1/sqrt(M^2 - 1).

use std::f64::consts::PI;

/// The airframe's Mach-1 wave drag, D/q, m2, as modeling/fighter.py
/// measures it.
pub const WAVE: f64 = 1.03;
/// Where Raymer's fit gives way to linear theory.
const FIT_MACH: f64 = 2.5;

pub struct Aero {
    pub wing: f64,
    pub aspect: f64,
    /// Zero-lift drag the aircraft adds to the polar: bays, tanks.
    pub cd0_tanks: f64,
    /// Peak pressure coefficient behind a normal shock, hypersonic.
    pub cp_max: f64,
    /// Wave drag at Mach 1, D/q, m2.
    pub wave: f64,
    /// Leading-edge sweep, degrees.
    pub sweep: f64,
}

impl Default for Aero {
    fn default() -> Self {
        Self { wing: 45., aspect: 1.88, cd0_tanks: 0.001, cp_max: 1.84, wave: WAVE, sweep: 58. }
    }
}

impl Aero {
    pub fn cd0(&self, mach: f64) -> f64 {
        let friction = 0.022 - 0.005 * ((mach - 1.) / 0.2).clamp(0., 1.);
        let rise = if mach < 1. { 5. * (mach - 0.9).max(0.) } else { 0.5 + 10. * (mach - 1.) };
        let fit = |m: f64| {
            1. - 0.386
                * (m - 1.2).max(0.).powf(0.57)
                * (1. - PI * self.sweep.powf(0.77) / 100.)
        };
        let fall = if mach <= FIT_MACH {
            fit(mach)
        } else {
            fit(FIT_MACH) * ((FIT_MACH * FIT_MACH - 1.) / (mach * mach - 1.)).sqrt()
        };
        let wave = self.wave * rise.min(1.) * fall.max(0.);
        self.cd0_tanks + friction + wave / self.wing
    }

    pub fn k(&self, mach: f64) -> f64 {
        let sub = 1. / (PI * 0.8 * self.aspect);
        if mach > 1. {
            sub.max((mach * mach - 1.).sqrt() / 4.)
        } else {
            sub
        }
    }

    /// Drag, N, at Mach `mach` and dynamic pressure `q`, carrying `lift` N.
    pub fn drag(&self, mach: f64, q: f64, lift: f64) -> f64 {
        let cl = lift / (q * self.wing);
        q * self.wing * (self.cd0(mach) + self.k(mach) * cl * cl)
    }

    /// Lift and drag coefficients at angle of attack `alpha`, hypersonic.
    pub fn newtonian(&self, mach: f64, alpha: f64) -> (f64, f64) {
        let cn = self.cp_max * alpha.sin().powi(2);
        (cn * alpha.cos(), cn * alpha.sin() + self.cd0(mach))
    }
}
