//! The fighter's aerodynamics: a 68° cropped delta of aspect ratio ~1.2,
//! slatted, with bays in its wing roots, and close-coupled canards or, on
//! the tailless aircraft, strakes on its intakes. Below Mach 5 a drag
//! polar: zero-lift drag, as a drag area that does not scale with the
//! wing, of ~0.99 m2 subsonic and ~0.77 m2 of skin friction supersonic for
//! the 14.2 m airframe with a 45 m2 wing the polar was set for, what the
//! aircraft adds to that, and wave drag; and drag due to lift, trimmed.
//! Hypersonic, modified Newtonian flow on the lower surface at the angle
//! of attack.
//!
//! Trim: the wing's lift acts at its centre of lift, behind or ahead of
//! the centre of gravity, and the trimmer balances it. Tailless, the
//! elevons, at the trailing edge, push down when the centre of gravity is
//! ahead, and the wing must lift more to make up for them; with canards,
//! ahead of the centre of gravity, they lift too. The wing's basic lift,
//! before its elevons take theirs away, sets its angle of attack.
//!
//! Drag due to lift, subsonic, lies between an ideal wing's and one
//! keeping no suction at its leading edge (Raymer): L CL / (pi A) for the
//! share s of the suction kept, and L alpha, the lift tilted back with the
//! angle of attack, for the rest. A sharp delta's flow separates at its
//! leading edge early, losing the suction from a lift coefficient of ~0.3;
//! slats, set by the flight computer, hold it to ~0.6, and close-coupled
//! canards' downwash and vortices to ~0.7. Supersonic from Mach 1.2, with
//! no suction, Raymer's A (M^2 - 1) cos(sweep) / (4 A sqrt(M^2 - 1) - 2)
//! for 1/CL_alpha, joined to the subsonic through the transonic. The
//! canards' own drag due to lift on their area, the same way.
//!
//! The bare delta's maximum lift coefficient is ~1.0 at 58°; at other
//! sweeps it follows Polhamus' suction analogy, potential lift and vortex
//! lift, at the angle where the leading-edge vortex bursts over the
//! trailing edge, ~25° at 60° of sweep and ~1.2° more for each degree
//! more. Slats add ~0.1, the strakes' vortices ~0.05, close-coupled
//! canards' ~0.15; the canards themselves stall at ~1.2 on their own
//! area. All of it estimates, for CFD to settle.
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
pub const WAVE: f64 = 1.04;
/// Zero-lift drag area but wave drag, D/q m2, subsonic and supersonic.
const FRICTION: (f64, f64) = (0.022 * 45., 0.017 * 45.);
/// Where Raymer's fit gives way to linear theory.
const FIT_MACH: f64 = 2.5;

#[derive(Clone, Copy)]
pub struct Canard {
    /// Exposed area, both sides, m2; aspect ratio; leading-edge sweep,
    /// degrees.
    pub area: f64,
    pub aspect: f64,
    pub sweep: f64,
    /// x of its centre of lift, m.
    pub x_ac: f64,
    pub cl_max: f64,
}

impl Default for Canard {
    fn default() -> Self {
        Self { area: 6.05, aspect: 2.51, sweep: 50., x_ac: 8.49, cl_max: 1.2 }
    }
}

#[derive(Clone, Copy)]
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
    /// What the slats and the strakes or canards add to the wing's maximum
    /// lift coefficient.
    pub devices: f64,
    /// x, m, of the wing's centre of lift subsonic and supersonic, of the
    /// elevons' lift, and of the centre of gravity.
    pub ac: (f64, f64),
    pub elevon: f64,
    pub cg: f64,
    pub canard: Option<Canard>,
}

impl Default for Aero {
    fn default() -> Self {
        Self {
            wing: 55.35,
            aspect: 1.21,
            drag_area: 0.045,
            cp_max: 1.84,
            wave: WAVE,
            sweep: 68.,
            taper: 0.144,
            suction: (0.85, 0.7),
            devices: 0.25,
            ac: (12.32, 12.98),
            elevon: 17.46,
            cg: 12.0,
            canard: Some(Canard::default()),
        }
    }
}

impl Aero {
    /// The same aircraft without canards, strakes on its intakes instead.
    pub fn tailless(self) -> Self {
        Self { suction: (self.suction.0, 0.6), devices: 0.15, canard: None, ..self }
    }

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

    /// Lift-curve slope, per radian, subsonic (DATCOM), the sweep taken
    /// at the thickest point, 40% of the chord.
    fn lift_slope(aspect: f64, sweep: f64, taper: f64, mach: f64) -> f64 {
        let beta2 = 1. - mach.min(0.95).powi(2);
        let tan_t = sweep.to_radians().tan() - 4. * 0.4 / aspect * (1. - taper) / (1. + taper);
        2. * PI * aspect / (2. + (4. + aspect * aspect * (beta2 + tan_t * tan_t)).sqrt())
    }

    /// The bare delta's lift coefficient at the vortex's burst, Polhamus.
    fn vortex_lift(&self) -> f64 {
        let alpha = (25. + 1.2 * (self.sweep - 60.)).to_radians();
        let (s, c) = alpha.sin_cos();
        Self::lift_slope(self.aspect, self.sweep, self.taper, 0.3) * s * c * c + PI * c * s * s
    }

    /// The wing's maximum lift coefficient, untrimmed.
    pub fn cl_max(&self) -> f64 {
        let at_58 = Self { sweep: 58., aspect: 1.88, ..*self };
        self.vortex_lift() / at_58.vortex_lift() + self.devices
    }

    /// 1/CL_alpha with no leading-edge suction, and the ideal 1/(pi A), of
    /// a surface; joined through the transonic.
    fn no_suction(aspect: f64, sweep: f64, taper: f64, mach: f64) -> f64 {
        let sup = |m: f64| {
            let b = (m * m - 1.).sqrt();
            aspect * b * b * sweep.to_radians().cos() / (4. * aspect * b - 2.)
        };
        let sub = |m: f64| 1. / Self::lift_slope(aspect, sweep, taper, m);
        if mach <= 1. {
            sub(mach)
        } else if mach < 1.2 {
            let t = (mach - 1.) / 0.2;
            sub(1.) * (1. - t) + sup(1.2) * t
        } else {
            sup(mach)
        }
    }

    /// How far the wing's centre of lift lies behind the centre of
    /// gravity, m.
    fn offset(&self, mach: f64) -> f64 {
        let t = ((mach - 0.9) / 0.3).clamp(0., 1.);
        self.ac.0 * (1. - t) + self.ac.1 * t - self.cg
    }

    /// The lift coefficient split for trim: the wing's net, its basic
    /// (before the elevons' share, which sets its angle of attack), and the
    /// canards' on their own area.
    fn split(&self, mach: f64, cl: f64) -> (f64, f64, f64) {
        let e = self.offset(mach);
        match self.canard {
            None => (cl, cl / (1. - e / (self.elevon - self.cg)), 0.),
            Some(c) => {
                let arm = self.cg - c.x_ac;
                let wing = cl * arm / (arm + e);
                (wing, wing, (cl - wing) * self.wing / c.area)
            }
        }
    }

    /// The largest trimmed lift coefficient: the wing's at its maximum, or
    /// the canards' at theirs.
    pub fn trimmed_cl_max(&self, mach: f64) -> f64 {
        let (lo, mut hi) = (0., 3.);
        let mut lo = lo;
        for _ in 0..60 {
            let cl = (lo + hi) / 2.;
            let (_, basic, canard) = self.split(mach, cl);
            let ok = basic <= self.cl_max() && self.canard.is_none_or(|c| canard.abs() <= c.cl_max);
            if ok {
                lo = cl;
            } else {
                hi = cl;
            }
        }
        lo
    }

    /// Drag, N, at Mach `mach` and dynamic pressure `q`, carrying `lift` N,
    /// trimmed.
    pub fn drag(&self, mach: f64, q: f64, lift: f64) -> f64 {
        let cl = lift / (q * self.wing);
        let (net, basic, cl_c) = self.split(mach, cl);
        let (kept, holds) = self.suction;
        let s = if mach < 1. {
            kept * (1. - (basic - holds).max(0.) / (self.cl_max() - holds)).clamp(0., 1.)
        } else {
            0.
        };
        let wing = s * net * net / (PI * self.aspect)
            + (1. - s) * net * basic * Self::no_suction(self.aspect, self.sweep, self.taper, mach);
        let canard = self.canard.map_or(0., |c| {
            let k = if mach < 1. {
                1. / (PI * 0.8 * c.aspect)
            } else {
                Self::no_suction(c.aspect, c.sweep, 0.35, mach)
            };
            c.area / self.wing * k * cl_c * cl_c
        });
        q * self.wing * (self.cd0(mach) + wing + canard)
    }

    /// Lift and drag coefficients at angle of attack `alpha`, hypersonic.
    pub fn newtonian(&self, mach: f64, alpha: f64) -> (f64, f64) {
        let cn = self.cp_max * alpha.sin().powi(2);
        (cn * alpha.cos(), cn * alpha.sin() + self.cd0(mach))
    }
}
