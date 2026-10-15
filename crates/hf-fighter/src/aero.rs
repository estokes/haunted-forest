//! The fighter's aerodynamics, Mirage 2000-like: 41 m2 of delta wing,
//! aspect ratio ~2, plus conformal tanks. Below Mach 5 a drag polar: zero-
//! lift drag 0.022 subsonic, skin friction plus wave drag falling as
//! 1/sqrt(M^2 - 1) supersonic, and drag due to lift from the subsonic
//! polar or linear supersonic theory, whichever is larger. Hypersonic,
//! modified Newtonian flow on the lower surface at the angle of attack.

use std::f64::consts::PI;

pub struct Aero {
    pub wing: f64,
    pub aspect: f64,
    /// Zero-lift drag the conformal tanks add.
    pub cd0_tanks: f64,
    /// Peak pressure coefficient behind a normal shock, hypersonic.
    pub cp_max: f64,
}

impl Default for Aero {
    fn default() -> Self {
        Self { wing: 41., aspect: 2.03, cd0_tanks: 0.002, cp_max: 1.84 }
    }
}

impl Aero {
    pub fn cd0(&self, mach: f64) -> f64 {
        self.cd0_tanks
            + if mach < 1.1 { 0.022 } else { 0.017 + 0.02 / (mach * mach - 1.).sqrt() }
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
