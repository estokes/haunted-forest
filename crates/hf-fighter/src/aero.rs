//! The fighter's aerodynamics, Mirage 2000-like: a 58° cropped delta of
//! aspect ratio ~1.9, slatted, with strakes on its intakes and bays in its
//! wing roots. Below Mach 5 a drag polar: zero-lift drag, as a drag area
//! that does not scale with the wing, of ~0.99 m2 subsonic and ~0.77 m2 of
//! skin friction supersonic for the 14.2 m airframe with a 45 m2 wing the
//! polar was set for, what the aircraft adds to that, and wave drag; and
//! drag due to lift. Hypersonic, modified Newtonian flow on the lower
//! surface at the angle of attack.
//!
//! Drag due to lift, subsonic, lies between an ideal wing's, 1/(pi A), and
//! that of a wing with no suction at its leading edge, 1/CL_alpha
//! (Raymer), by how much of the suction the leading edge keeps: most of
//! it at low lift, none at the stall. A sharp delta's flow separates at
//! its leading edge early, losing the suction from a lift coefficient of
//! ~0.3; slats, set by the flight computer, hold it to ~0.6. Supersonic
//! from Mach 1.2, Raymer's A (M^2 - 1) cos(sweep) / (4 A sqrt(M^2 - 1) - 2),
//! joined linearly to the subsonic through the transonic.
//!
//! The bare delta's maximum lift coefficient is ~1.0 at 58°; at other
//! sweeps it follows Polhamus' suction analogy, potential lift and vortex
//! lift, at the angle where the leading-edge vortex bursts over the
//! trailing edge, ~25° at 60° of sweep and ~1.2° more for each degree
//! more. Slats and the strakes' vortices raise it by ~0.1 and ~0.05. All
//! of it estimates, for CFD to settle.
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
pub const WAVE: f64 = 1.25;
/// Zero-lift drag area but wave drag, D/q m2, subsonic and supersonic.
const FRICTION: (f64, f64) = (0.022 * 45., 0.017 * 45.);
/// Where Raymer's fit gives way to linear theory.
const FIT_MACH: f64 = 2.5;

pub struct Aero {
    pub wing: f64,
    pub aspect: f64,
    /// Zero-lift drag area the aircraft adds to the polar's, D/q, m2.
    pub drag_area: f64,
    /// Peak pressure coefficient behind a normal shock, hypersonic.
    pub cp_max: f64,
    /// Wave drag at Mach 1, D/q, m2.
    pub wave: f64,
    /// Leading-edge sweep, degrees.
    pub sweep: f64,
    /// Tip chord over root chord.
    pub taper: f64,
    /// Share of the leading-edge suction kept at low lift, and the lift
    /// coefficient to which it holds.
    pub suction: (f64, f64),
    /// What the slats and strakes add to the maximum lift coefficient.
    pub devices: f64,
}

impl Default for Aero {
    fn default() -> Self {
        Self {
            wing: 55.,
            aspect: 1.88,
            drag_area: 0.045,
            cp_max: 1.84,
            wave: WAVE,
            sweep: 58.,
            taper: 0.144,
            suction: (0.85, 0.6),
            devices: 0.15,
        }
    }
}

impl Aero {
    pub fn cd0(&self, mach: f64) -> f64 {
        let friction = FRICTION.0 - (FRICTION.0 - FRICTION.1) * ((mach - 1.) / 0.2).clamp(0., 1.);
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
        (friction + self.drag_area + wave) / self.wing
    }

    /// The same wing without its slats or the intakes' strakes.
    pub fn plain(self) -> Self {
        Self { suction: (self.suction.0, 0.3), devices: 0., ..self }
    }

    /// The bare delta's lift coefficient at the vortex's burst, Polhamus.
    fn vortex_lift(&self) -> f64 {
        let alpha = (25. + 1.2 * (self.sweep - 60.)).to_radians();
        let (s, c) = alpha.sin_cos();
        self.lift_slope(0.3) * s * c * c + PI * c * s * s
    }

    pub fn cl_max(&self) -> f64 {
        let at_58 = Self { sweep: 58., aspect: 1.88, ..*self };
        self.vortex_lift() / at_58.vortex_lift() + self.devices
    }

    /// Lift-curve slope, per radian, subsonic (DATCOM), the sweep taken
    /// at the thickest point, 40% of the chord.
    pub fn lift_slope(&self, mach: f64) -> f64 {
        let a = self.aspect;
        let beta2 = 1. - mach.min(0.95).powi(2);
        let tan_t = self.sweep.to_radians().tan()
            - 4. * 0.4 / a * (1. - self.taper) / (1. + self.taper);
        2. * PI * a / (2. + (4. + a * a * (beta2 + tan_t * tan_t)).sqrt())
    }

    /// Drag due to lift over CL^2, at Mach `mach` and lift coefficient `cl`.
    pub fn k(&self, mach: f64, cl: f64) -> f64 {
        let (kept, holds) = self.suction;
        let sub = |m: f64| {
            let s = kept * (1. - (cl - holds).max(0.) / (self.cl_max() - holds)).clamp(0., 1.);
            s / (PI * self.aspect) + (1. - s) / self.lift_slope(m)
        };
        let sup = |m: f64| {
            let b = (m * m - 1.).sqrt();
            self.aspect * b * b * self.sweep.to_radians().cos() / (4. * self.aspect * b - 2.)
        };
        if mach <= 1. {
            sub(mach)
        } else if mach < 1.2 {
            let t = (mach - 1.) / 0.2;
            sub(1.) * (1. - t) + sup(1.2) * t
        } else {
            sup(mach)
        }
    }

    /// Drag, N, at Mach `mach` and dynamic pressure `q`, carrying `lift` N.
    pub fn drag(&self, mach: f64, q: f64, lift: f64) -> f64 {
        let cl = lift / (q * self.wing);
        q * self.wing * (self.cd0(mach) + self.k(mach, cl) * cl * cl)
    }

    /// Lift and drag coefficients at angle of attack `alpha`, hypersonic.
    pub fn newtonian(&self, mach: f64, alpha: f64) -> (f64, f64) {
        let cn = self.cp_max * alpha.sin().powi(2);
        (cn * alpha.cos(), cn * alpha.sin() + self.cd0(mach))
    }
}
