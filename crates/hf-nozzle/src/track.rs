//! Charged products of p-pbar annihilation tracked through the nozzle field,
//! with pi -> mu nu and mu -> e nu nu decays, tallied by where their energy
//! and momentum go.

use crate::field::{Coils, Part};
use glam::DVec3;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rand_distr::{Distribution, Gamma};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{f64::consts::PI, ops::Add};

pub const C: f64 = 2.998e8;
const Q: f64 = 1.602176634e-19;
const MEV_C: f64 = 1e6 * Q / C; // kg m/s per MeV/c
const M_PI: f64 = 139.57; // MeV
const M_MU: f64 = 105.66;
const M_E: f64 = 0.511;
const TAU_PI: f64 = 26.03e-9;
const TAU_MU: f64 = 2.197e-6;
pub const ANN_MEV: f64 = 1876.5;
const P_STAR_PI: f64 = 29.79; // muon momentum in the pion frame, MeV/c
const PIONS_PER_ANN: usize = 3;
/// Ship hits are binned by position along the struck body, this long each.
pub const HIT_BIN_M: f64 = 20.0;
pub const HIT_BINS: usize = 16;

/// A solid cylinder of radius r from `a`, the end facing the reaction, to
/// `b`. Its end a is a shield plate if `plate`, and lies `s0` along the pod
/// from the pod's plate.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Body {
    pub a: [f64; 3],
    pub b: [f64; 3],
    pub r: f64,
    #[serde(default)]
    pub s0: f64,
    #[serde(default)]
    pub plate: bool,
}

impl Body {
    /// Distance from x to the solid, and x's position along the axis from a.
    fn locate(&self, x: DVec3) -> (f64, f64) {
        let a = DVec3::from(self.a);
        let ab = DVec3::from(self.b) - a;
        let len = ab.length();
        let u = ab / len;
        let t = (x - a).dot(u);
        let radial = (x - a - t * u).length();
        ((radial - self.r).max(0.0).hypot((-t).max(t - len).max(0.0)), t)
    }

    pub fn distance(&self, x: DVec3) -> f64 {
        self.locate(x).0
    }
}

/// The ship: every solid charged particles must not reach.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Ship {
    pub bodies: Vec<Body>,
}

enum Struck {
    Plate,
    Side,
}

impl Ship {
    pub fn distance(&self, x: DVec3) -> f64 {
        self.bodies.iter().map(|b| b.distance(x)).fold(f64::INFINITY, f64::min)
    }

    /// What a step from `from` to `to` struck, if anything, and where along
    /// the body: a plate if the step came from in front of the reaction end.
    fn struck(&self, from: DVec3, to: DVec3) -> Option<(Struck, f64)> {
        let body = self.bodies.iter().find(|b| b.distance(to) == 0.0)?;
        let s = if body.plate && body.locate(from).1 < 0.0 { Struck::Plate } else { Struck::Side };
        Some((s, body.s0 + body.locate(to).1))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Run {
    pub n_ann: usize,
    pub seed: u64,
    /// Sample pion directions only with cos(theta) > cone toward the ship,
    /// weighting tallies by that solid-angle fraction. Resolves the forward
    /// leak; thrust from such a run is meaningless.
    pub cone: Option<f64>,
    /// Scales every time step, for convergence checks.
    pub step: f64,
    pub hit_r: f64,
    pub r_dom: f64,
    pub max_steps: usize,
    pub source: [f64; 3],
}

impl Default for Run {
    fn default() -> Self {
        Self {
            n_ann: 20_000,
            seed: 1,
            cone: None,
            step: 1.0,
            hit_r: 2.0,
            r_dom: 8000.0,
            max_steps: 200_000,
            source: [0.0; 3],
        }
    }
}

/// Energies in MeV, momenta in MeV/c and angular momenta about the origin
/// in MeV/c m, summed over tracks.
#[derive(Clone, Copy, Debug, Default)]
pub struct Tally {
    pub escape_p: DVec3,
    pub nu_p: DVec3,
    pub escape_l: DVec3,
    pub nu_l: DVec3,
    pub escape_e: f64,
    pub nu_e: f64,
    pub fwd_e: f64,
    pub coil_e: [f64; Part::ALL.len()],
    pub plate_e: f64,
    pub side_e: f64,
    pub ship_n: u64,
    /// Energy striking the ship by position along the body and by species
    /// (pion, muon, electron).
    pub hit_along: [f64; HIT_BINS],
    pub hit_species: [f64; 3],
    pub trapped_e: f64,
}

impl Add for Tally {
    type Output = Tally;
    fn add(mut self, o: Tally) -> Tally {
        self.escape_p += o.escape_p;
        self.nu_p += o.nu_p;
        self.escape_l += o.escape_l;
        self.nu_l += o.nu_l;
        self.escape_e += o.escape_e;
        self.nu_e += o.nu_e;
        self.fwd_e += o.fwd_e;
        for (a, b) in self.coil_e.iter_mut().zip(o.coil_e) {
            *a += b;
        }
        self.plate_e += o.plate_e;
        self.side_e += o.side_e;
        self.ship_n += o.ship_n;
        for (a, b) in self.hit_along.iter_mut().zip(o.hit_along) {
            *a += b;
        }
        for (a, b) in self.hit_species.iter_mut().zip(o.hit_species) {
            *a += b;
        }
        self.trapped_e += o.trapped_e;
        self
    }
}

pub(crate) fn isotropic(rng: &mut SmallRng) -> DVec3 {
    let u: f64 = rng.random_range(-1.0..1.0);
    let phi = rng.random_range(0.0..2.0 * PI);
    let s = (1.0 - u * u).sqrt();
    DVec3::new(s * phi.cos(), s * phi.sin(), u)
}

/// |p| for charged pions from p-pbar at rest: p^2 exp(-p/p0), mean ~350 MeV/c.
fn pion_momentum(rng: &mut SmallRng) -> f64 {
    let g = Gamma::new(3.0, 117.0).unwrap();
    loop {
        let p = g.sample(rng);
        if p < 928.0 {
            return p;
        }
    }
}

/// Electron energy fraction x = 2E/m_mu from the Michel spectrum x^2(3-2x).
fn michel(rng: &mut SmallRng) -> f64 {
    loop {
        let x: f64 = rng.random();
        if rng.random::<f64>() < x * x * (3.0 - 2.0 * x) {
            return x;
        }
    }
}

/// Lorentz-boost a rest-frame (E, p) by velocity beta.
fn boost(e: f64, p: DVec3, beta: DVec3) -> (f64, DVec3) {
    let b2 = beta.length_squared();
    if b2 == 0.0 {
        return (e, p);
    }
    let g = 1.0 / (1.0 - b2).sqrt();
    let bp = beta.dot(p);
    (g * (e + bp), p + ((g - 1.0) * bp / b2 + g * e) * beta)
}

struct Track {
    x: DVec3,
    p: DVec3,
    m: f64,
    charge: f64,
    life: f64,
}

/// Time step for energy e (MeV) at x in field b: a fraction of a gyration,
/// and a path no longer than half the clearance to anything solid.
pub(crate) fn time_step(x: DVec3, b: DVec3, e: f64, clear: f64, scale: f64) -> f64 {
    let omega = Q * (b.length() + 1e-30) / (e * MEV_C / C); // rad/s, relativistic
    let ds = (0.01 * x.length() + 1.0).clamp(1.0, 40.0).min((0.5 * clear).max(1.0));
    scale * (0.15 / omega).min(ds / C)
}

/// Advance momentum p (MeV/c) and position x one Boris step of dt in field b.
pub(crate) fn boris(x: &mut DVec3, p: &mut DVec3, e: f64, charge: f64, b: DVec3, dt: f64) {
    let tv = b * (charge * Q * dt / (2.0 * e * MEV_C / C)); // unit-free in p
    let pp = *p + p.cross(tv);
    *p += pp.cross(2.0 * tv / (1.0 + tv.length_squared()));
    *x += *p / e * C * dt;
}

/// Follow one pion and its decay chain to its fate.
fn follow(coils: &Coils, ship: &Ship, run: &Run, rng: &mut SmallRng) -> Tally {
    let mut t = Tally::default();
    let dir = match run.cone {
        None => isotropic(rng),
        Some(c) => {
            let u: f64 = rng.random_range(c..1.0);
            let phi = rng.random_range(0.0..2.0 * PI);
            let s = (1.0 - u * u).sqrt();
            DVec3::new(s * phi.cos(), s * phi.sin(), u)
        }
    };
    let mut k = Track {
        x: DVec3::from(run.source),
        p: dir * pion_momentum(rng),
        m: M_PI,
        charge: if rng.random::<bool>() { 1.0 } else { -1.0 },
        life: -TAU_PI * rng.random::<f64>().max(1e-300).ln(),
    };
    let mut clear = coils.clearance(k.x).0;
    for _ in 0..run.max_steps {
        let e = (k.p.length_squared() + k.m * k.m).sqrt();
        let b = coils.field(k.x);
        let dt = time_step(k.x, b, e, clear.min(ship.distance(k.x)), run.step);
        let from = k.x;
        boris(&mut k.x, &mut k.p, e, k.charge, b, dt);
        k.life -= dt * k.m / e;

        let (c, part) = coils.clearance(k.x);
        clear = c;
        if c < run.hit_r {
            t.coil_e[part as usize] += e;
            return t;
        }
        if let Some((s, along)) = ship.struck(from, k.x) {
            match s {
                Struck::Plate => t.plate_e += e,
                Struck::Side => t.side_e += e,
            }
            t.ship_n += 1;
            t.hit_along[((along / HIT_BIN_M).max(0.0) as usize).min(HIT_BINS - 1)] += e;
            t.hit_species[if k.m == M_PI {
                0
            } else if k.m == M_MU {
                1
            } else {
                2
            }] += e;
            return t;
        }
        if k.x.length() > run.r_dom {
            t.escape_p += k.p;
            t.escape_l += k.x.cross(k.p);
            t.escape_e += e;
            if k.p.z > 0.0 {
                t.fwd_e += e;
            }
            return t;
        }
        if k.life <= 0.0 {
            let beta = k.p / e;
            let (de, dp) = if k.m == M_PI {
                let e_star = (P_STAR_PI * P_STAR_PI + M_MU * M_MU).sqrt();
                k.m = M_MU;
                k.life = -TAU_MU * rng.random::<f64>().max(1e-300).ln();
                boost(e_star, isotropic(rng) * P_STAR_PI, beta)
            } else if k.m == M_MU {
                let ee = (michel(rng) * M_MU / 2.0).max(M_E * 1.0001);
                k.m = M_E;
                k.life = f64::INFINITY;
                boost(ee, isotropic(rng) * (ee * ee - M_E * M_E).sqrt(), beta)
            } else {
                unreachable!("electrons do not decay")
            };
            t.nu_e += e - de;
            t.nu_p += k.p - dp;
            t.nu_l += k.x.cross(k.p - dp);
            k.p = dp;
        }
    }
    t.trapped_e += (k.p.length_squared() + k.m * k.m).sqrt();
    t
}

/// Track run.n_ann annihilations' worth of charged pions in parallel. Each
/// track has its own seeded generator, so results do not depend on threading.
pub fn simulate(coils: &Coils, ship: &Ship, run: &Run) -> Tally {
    (0..run.n_ann * PIONS_PER_ANN)
        .into_par_iter()
        .map(|i| {
            let mut rng = SmallRng::seed_from_u64((run.seed << 40) ^ i as u64);
            follow(coils, ship, run, &mut rng)
        })
        .reduce(Tally::default, Tally::add)
}

/// Annihilation energy the tally stands for, MeV.
pub fn e_ann(run: &Run) -> f64 {
    let weight = run.cone.map_or(1.0, |c| (1.0 - c) / 2.0);
    run.n_ann as f64 * ANN_MEV / weight
}

#[cfg(test)]
mod tests {
    use super::*;

    fn far_ship() -> Ship {
        Ship::default()
    }

    #[test]
    fn no_field_gives_no_net_thrust_and_known_neutrino_share() {
        let coils = Coils::new(vec![], &[]);
        let run = Run { n_ann: 20_000, r_dom: 2e5, ..Run::default() };
        let t = simulate(&coils, &far_ship(), &run);
        let e = e_ann(&run);
        let f = (t.escape_p + t.nu_p) / e;
        assert!(f.length() < 0.01, "net momentum {f}");
        let nu = t.nu_e / (t.nu_e + t.escape_e);
        assert!((0.6..0.8).contains(&nu), "neutrino share of charged energy {nu}");
        assert_eq!(t.trapped_e, 0.0);
    }

    #[test]
    fn decays_conserve_energy() {
        let coils = Coils::new(vec![], &[]);
        let run = Run { n_ann: 2_000, r_dom: 2e5, ..Run::default() };
        let t = simulate(&coils, &far_ship(), &run);
        let mut want = 0.0;
        for i in 0..run.n_ann * PIONS_PER_ANN {
            let mut r = SmallRng::seed_from_u64((run.seed << 40) ^ i as u64);
            let _ = isotropic(&mut r);
            let p = pion_momentum(&mut r);
            want += (p * p + M_PI * M_PI).sqrt();
        }
        let got = t.escape_e + t.nu_e;
        assert!((got - want).abs() < 1e-6 * want, "{got} vs {want}");
    }
}
