//! The fighter's aerodynamics: a 68° cropped delta of aspect ratio ~1.2,
//! sharp-edged and slatted, with bays in its wing roots, and close-coupled
//! canards or, on the tailless aircraft, strakes on its intakes. Below
//! Mach 5 a drag polar: zero-lift drag, as a drag area that does not scale
//! with the wing, of ~0.99 m2 subsonic and ~0.77 m2 of skin friction
//! supersonic for the 14.2 m airframe with a 45 m2 wing the polar was set
//! for, what the aircraft adds to that, and wave drag; and drag due to
//! lift, trimmed. Hypersonic, modified Newtonian flow on the lower surface
//! at the angle of attack. Calibrated against SU2's Euler solutions of the
//! airframe (cfd/), with and without its canards, slats in and drooped.
//!
//! Lift: the wing and body's is CL0, the cambered body's at no angle of
//! attack, and Polhamus's, potential lift Kp sin a cos^2 a and vortex lift
//! Kv cos a sin^2 a. Kp is the attached flow's slope:
//! DATCOM's subsonic, a delta's linear theory supersonic (2 pi cot(sweep) /
//! E(k), its leading edge inside the Mach cone), the wing and body making
//! SUPERSONIC_SLOPE of it. Kv is KV subsonic, fading as the leading edge
//! nears the Mach cone; at other sweeps and aspect ratios it scales as
//! Polhamus's suction analogy has it. The wing's lift sets its angle of
//! attack.
//!
//! Its maximum is the lift where the leading-edge vortex bursts over the
//! wing: at 25° on the bare wing and body, at 32° behind close-coupled
//! canards, whose vortices hold it; ~1.2° later for each degree more of
//! sweep. The canards stall at ~1.2 on their own area.
//!
//! Trim: the wing's lift acts at its centre of lift, behind or ahead of
//! the centre of gravity, and the trimmer balances it. With canards, ahead
//! of the centre of gravity, they lift, and once they reach their stall the
//! elevons take the rest; tailless, the elevons alone, pushing down when
//! the centre of gravity is ahead, the wing lifting more to make up for
//! them. The wing's basic lift, before its elevons take theirs away, sets
//! its angle of attack.
//!
//! Drag due to lift: the sharp leading edge keeps none of its suction, so
//! the lift tilts back with the angle of attack, L tan a. Subsonic, the
//! slats, drooped by the flight computer, win back a share s of it, the
//! ideal L CL / (pi A): ~0.30 at low lift, ~0.18 at the wing's maximum.
//! The canards the same way on their own area, with no suction.
//!
//! The wave drag is the airframe's own, its Mach-1 D/q: rising through the
//! transonic from nothing at Mach 0.9 to half at Mach 1 and all of it by
//! 1.05; falling off with Mach as Raymer's fit for a wing of this sweep has
//! it, to Mach 2.5, the edge of the aircraft it was fitted to; and beyond
//! that as linear theory has it, with 1/sqrt(M^2 - 1).

use std::f64::consts::PI;

/// The complete elliptic integral of the second kind, E(k), by the
/// arithmetic-geometric mean.
fn ellip_e(k: f64) -> f64 {
    if k >= 1. {
        return 1.;
    }
    let (mut a, mut b, mut c) = (1., (1. - k * k).sqrt(), k);
    let (mut sum, mut weight) = (c * c / 2., 0.5);
    while c.abs() > 1e-15 {
        c = (a - b) / 2.;
        (a, b) = ((a + b) / 2., (a * b).sqrt());
        weight *= 2.;
        sum += weight * c * c;
    }
    PI / (2. * a) * (1. - sum)
}

/// The airframe's Mach-1 wave drag, D/q, m2: Euler finds ~0.6 from Mach
/// 1.05 to 2, where the area rule's slender-body estimate from
/// modeling/fighter.py is 1.0.
pub const WAVE: f64 = 0.62;
/// Zero-lift drag area but wave drag, D/q m2, subsonic and supersonic.
const FRICTION: (f64, f64) = (0.022 * 45., 0.017 * 45.);
/// Where Raymer's fit gives way to linear theory.
const FIT_MACH: f64 = 2.5;
/// The wing and body's vortex-lift factor subsonic, and how fast it fades
/// supersonic: Kv (1 - KV_FADE beta cot(sweep)).
const KV: f64 = 2.9;
const KV_FADE: f64 = 1.2;
/// The wing and body's supersonic lift slope over a delta's linear theory;
/// their lift at no angle of attack, the low wing and flat belly's camber.
const SUPERSONIC_SLOPE: f64 = 0.85;
const CL0: f64 = 0.03;
/// The planform Euler was fitted on: sweep, degrees, and aspect ratio; and
/// how much later the vortex bursts for each degree more of sweep.
const FIT_PLANFORM: (f64, f64) = (68., 1.21);
const BURST_PER_DEGREE: f64 = 1.2;

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
    /// Share of the leading-edge suction the slats win back subsonic, at
    /// no lift and at the wing's maximum.
    pub suction: (f64, f64),
    /// Angle of attack, degrees, at which the leading-edge vortex bursts
    /// over the wing, at the fitted sweep.
    pub burst: f64,
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
            suction: (0.30, 0.18),
            burst: 32.,
            ac: (12.44, 13.2),
            elevon: 16.71,
            cg: 12.0,
            canard: Some(Canard::default()),
        }
    }
}

impl Aero {
    /// The same aircraft without canards, strakes on its intakes instead.
    pub fn tailless(self) -> Self {
        Self { burst: 25., canard: None, ..self }
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

    /// Lift-curve slope, per radian, of a surface's attached flow. Subsonic,
    /// DATCOM's, the sweep taken at the thickest point, 40% of the chord.
    /// Supersonic, a delta's: with its leading edges inside the Mach cone,
    /// 2 pi cot(sweep) / E(k), k^2 = 1 - (beta cot(sweep))^2; outside it,
    /// Ackeret's 4 / beta. Joined through the transonic.
    fn lift_slope(aspect: f64, sweep: f64, taper: f64, mach: f64) -> f64 {
        let sub = |m: f64| {
            let beta2 = 1. - m.min(0.95).powi(2);
            let tan_t = sweep.to_radians().tan() - 4. * 0.4 / aspect * (1. - taper) / (1. + taper);
            2. * PI * aspect / (2. + (4. + aspect * aspect * (beta2 + tan_t * tan_t)).sqrt())
        };
        let sup = |m: f64| {
            let beta = (m * m - 1.).sqrt();
            let cot = 1. / sweep.to_radians().tan();
            let edge = beta * cot;
            if edge < 1. { 2. * PI * cot / ellip_e((1. - edge * edge).sqrt()) } else { 4. / beta }
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

    /// The wing and body's potential- and vortex-lift factors, Kp and Kv.
    fn factors(&self, mach: f64) -> (f64, f64) {
        let share = 1. - (1. - SUPERSONIC_SLOPE) * ((mach - 1.) / 0.2).clamp(0., 1.);
        let kp = Self::lift_slope(self.aspect, self.sweep, self.taper, mach) * share;
        let analogy = |sweep: f64, aspect: f64| {
            let k = Self::lift_slope(aspect, sweep, self.taper, 0.3);
            (k - k * k / (PI * aspect)) / sweep.to_radians().cos()
        };
        let scale = analogy(self.sweep, self.aspect) / analogy(FIT_PLANFORM.0, FIT_PLANFORM.1);
        let fade = if mach > 1. {
            let edge = (mach * mach - 1.).sqrt() / self.sweep.to_radians().tan();
            (1. - KV_FADE * edge).max(0.)
        } else {
            1.
        };
        (kp, KV * scale * fade)
    }

    fn polhamus((kp, kv): (f64, f64), a: f64) -> f64 {
        let (s, c) = a.sin_cos();
        kp * s * c * c + kv * c * s * s
    }

    /// The wing and body's angle of attack, radians, for lift `cl`.
    fn alpha(&self, mach: f64, cl: f64) -> f64 {
        let (kp, kv) = self.factors(mach);
        let target = (cl - CL0).abs();
        let mut a = target / kp;
        for _ in 0..30 {
            let (s, c) = a.sin_cos();
            let slope = kp * (c * c * c - 2. * s * s * c) + kv * (2. * s * c * c - s * s * s);
            a = (a - (Self::polhamus((kp, kv), a) - target) / slope).clamp(0., 1.);
        }
        a.copysign(cl - CL0)
    }

    /// The wing's maximum lift coefficient, untrimmed: where its vortex
    /// bursts.
    pub fn cl_max(&self) -> f64 {
        let burst = self.burst + BURST_PER_DEGREE * (self.sweep - FIT_PLANFORM.0);
        CL0 + Self::polhamus(self.factors(0.3), burst.to_radians())
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
        let elevon = self.elevon - self.cg;
        match self.canard {
            None => (cl, cl / (1. - e / elevon), 0.),
            Some(c) => {
                let arm = self.cg - c.x_ac;
                let wing = cl * arm / (arm + e);
                let limit = c.cl_max * c.area / self.wing;
                if (cl - wing).abs() <= limit {
                    (wing, wing, (cl - wing) * self.wing / c.area)
                } else {
                    let lc = limit.copysign(cl - wing);
                    let basic = (cl * elevon - lc * (arm + elevon)) / (elevon - e);
                    (cl - lc, basic, lc * self.wing / c.area)
                }
            }
        }
    }

    /// The largest trimmed lift coefficient: the wing's basic lift at its
    /// maximum.
    pub fn trimmed_cl_max(&self, mach: f64) -> f64 {
        let (mut lo, mut hi) = (0., 3.);
        for _ in 0..60 {
            let cl = (lo + hi) / 2.;
            if self.split(mach, cl).1 <= self.cl_max() {
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
        let s = if mach < 1. {
            let (s0, s1) = self.suction;
            s0 + (s1 - s0) * (basic / self.cl_max()).clamp(0., 1.)
        } else {
            0.
        };
        let wing = s * net * net / (PI * self.aspect) + (1. - s) * net * self.alpha(mach, basic).tan();
        let canard = self.canard.map_or(0., |c| {
            c.area / self.wing * cl_c * cl_c / Self::lift_slope(c.aspect, c.sweep, 0.35, mach)
        });
        q * self.wing * (self.cd0(mach) + wing + canard)
    }

    /// Lift and drag coefficients at angle of attack `alpha`, hypersonic.
    pub fn newtonian(&self, mach: f64, alpha: f64) -> (f64, f64) {
        let cn = self.cp_max * alpha.sin().powi(2);
        (cn * alpha.cos(), cn * alpha.sin() + self.cd0(mach))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// SU2 Euler, the wing and body without canards (cfd/, the "bare"
    /// mesh): (Mach, alpha degrees, CL, CD less its drag at zero alpha).
    const BARE: [(f64, f64, f64, f64); 6] = [
        (0.5, 4., 0.1564, 0.0107),
        (0.5, 10., 0.3852, 0.0653),
        (0.5, 20., 0.7671, 0.2725),
        (1.2, 10., 0.4328, 0.0777),
        (1.5, 10., 0.3784, 0.0680),
        (2.0, 10., 0.3063, 0.0562),
    ];

    #[test]
    fn elliptic() {
        assert!((ellip_e(0.) - PI / 2.).abs() < 1e-12);
        assert!((ellip_e(0.5) - 1.467_462_209_339_427).abs() < 1e-12);
        assert!((ellip_e(1.) - 1.).abs() < 1e-9);
    }

    #[test]
    fn supersonic_slope_meets_euler() {
        for (mach, cfd) in [(1.2, 2.315), (1.5, 2.161), (2.0, 1.793)] {
            let slope = Aero::lift_slope(1.21, 68., 0.144, mach);
            assert!((slope / cfd - 1.).abs() < 0.06, "Mach {mach}: {slope} against {cfd}");
        }
    }

    #[test]
    fn lift_curve_meets_euler() {
        let a = Aero::default().tailless();
        for (mach, alpha, cl, _) in BARE {
            let model = CL0 + Aero::polhamus(a.factors(mach), alpha.to_radians());
            assert!((model / cl - 1.).abs() < 0.10, "Mach {mach} at {alpha}°: {model:.4} against {cl:.4}");
        }
    }

    #[test]
    fn drag_due_to_lift_meets_euler() {
        let a = Aero::default().tailless();
        for (mach, alpha, cl, cd) in BARE {
            let model = cl * a.alpha(mach, cl).tan();
            assert!((model / cd - 1.).abs() < 0.15, "Mach {mach} at {alpha}°: {model:.4} against {cd:.4}");
        }
    }

    #[test]
    fn maximum_lift_meets_euler() {
        let bare = Aero::default().tailless().cl_max();
        assert!((bare / 1.02 - 1.).abs() < 0.08, "bare {bare:.3}");
        let canards = Aero::default().cl_max();
        assert!((canards / 1.32 - 1.).abs() < 0.08, "with canards {canards:.3}");
    }

    #[test]
    fn trim_balances() {
        let a = Aero { cg: 11.9, ..Aero::default() };
        let c = a.canard.unwrap();
        for (mach, cl) in [(0.5, 0.4), (1.5, 0.3), (1.5, 1.2)] {
            let (net, basic, cl_c) = a.split(mach, cl);
            let lc = cl_c * c.area / a.wing;
            let e = a.offset(mach);
            let moment = lc * (a.cg - c.x_ac) - basic * e - (net - basic) * (a.elevon - a.cg);
            assert!(moment.abs() < 1e-9 && (net + lc - cl).abs() < 1e-9, "Mach {mach}, CL {cl}");
            assert!(cl_c.abs() <= c.cl_max + 1e-9);
        }
    }
}
