//! Cosmic-ray shelter from the ring's field. A direction of arrival at a
//! point is open if a proton arriving that way came from free space: traced
//! backward, with its charge reversed, it escapes. By Liouville's theorem the
//! intensity along every open direction is the free-space intensity, so the
//! open fraction of directions is the fraction of an isotropic flux that
//! reaches the point. Solids are transparent here, except the conductors.

use crate::{
    design::Spec,
    field::Coils,
    track::{boris, isotropic, time_step},
};
use anyhow::{bail, Result};
use glam::DVec3;
use rand::{rngs::SmallRng, SeedableRng};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

const M_P: f64 = 938.272; // MeV

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ShelterSpec {
    pub nozzle: Spec,
    /// Probe points on the first pod's axis, metres from its plate.
    pub along: Vec<f64>,
    /// Proton rigidities, GV (momentum in GeV/c).
    pub rigidity_gv: Vec<f64>,
    /// Directions sampled per point and rigidity.
    pub n: usize,
    pub seed: u64,
    /// A backward track this far from the origin has escaped.
    pub r_esc: f64,
    pub max_steps: usize,
}

impl Default for ShelterSpec {
    fn default() -> Self {
        Self {
            nozzle: Spec::default(),
            along: vec![0.0, 10.0, 20.0, 40.0, 80.0, 120.0, 160.0, 220.0],
            rigidity_gv: vec![0.5, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 20.0, 40.0],
            n: 4000,
            seed: 1,
            r_esc: 30_000.0,
            max_steps: 200_000,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ShelterReport {
    pub spec: ShelterSpec,
    pub points: Vec<[f64; 3]>,
    /// open[i][j]: open fraction at along[i] for rigidity_gv[j].
    pub open: Vec<Vec<f64>>,
    /// Fraction of backward tracks still bound at max_steps, counted shut.
    pub trapped: Vec<Vec<f64>>,
    pub secs: f64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Fate {
    Open,
    Shut,
    Trapped,
}

fn trace(coils: &Coils, x0: DVec3, p_mev: f64, spec: &ShelterSpec, rng: &mut SmallRng) -> Fate {
    let (mut x, mut p) = (x0, isotropic(rng) * p_mev);
    let e = (p_mev * p_mev + M_P * M_P).sqrt();
    let mut clear = coils.clearance(x).0;
    for _ in 0..spec.max_steps {
        let b = coils.field(x);
        let dt = time_step(x, b, p_mev, e, -1.0, clear, spec.nozzle.run.step);
        boris(&mut x, &mut p, e, -1.0, b, dt);
        clear = coils.clearance(x).0;
        if clear < spec.nozzle.run.hit_r {
            return Fate::Shut;
        }
        if x.length() > spec.r_esc {
            return Fate::Open;
        }
    }
    Fate::Trapped
}

/// (open, trapped) fractions at x for each rigidity.
fn probe(coils: &Coils, x: DVec3, spec: &ShelterSpec, k: u64) -> Vec<(f64, f64)> {
    spec.rigidity_gv
        .iter()
        .enumerate()
        .map(|(j, &r)| {
            let fates: Vec<Fate> = (0..spec.n)
                .into_par_iter()
                .map(|i| {
                    let seed = (spec.seed << 40) ^ (k << 32) ^ ((j as u64) << 24) ^ i as u64;
                    trace(coils, x, r * 1e3, spec, &mut SmallRng::seed_from_u64(seed))
                })
                .collect();
            let frac = |f| fates.iter().filter(|&&g| g == f).count() as f64 / spec.n as f64;
            (frac(Fate::Open), frac(Fate::Trapped))
        })
        .collect()
}

impl ShelterSpec {
    pub fn evaluate(&self) -> Result<ShelterReport> {
        let start = Instant::now();
        let (coils, ship) = self.nozzle.build()?;
        let Some(pod) = ship.bodies.first() else { bail!("the ship has no bodies to probe") };
        let (a, b) = (DVec3::from(pod.a), DVec3::from(pod.b));
        let u = (b - a).normalize();
        let points: Vec<DVec3> = self.along.iter().map(|&t| a + t * u).collect();
        let (open, trapped) = points
            .iter()
            .enumerate()
            .map(|(k, &x)| probe(&coils, x, self, k as u64).into_iter().unzip())
            .unzip();
        Ok(ShelterReport {
            spec: self.clone(),
            points: points.iter().map(|&x| x.into()).collect(),
            open,
            trapped,
            secs: start.elapsed().as_secs_f64(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::{Loop, Part};

    /// A small strong loop is a dipole. In its plane at distance r every
    /// direction is open above p/q = M/r² and none below M/(r(1+√2))², with
    /// M = μ0 m / 4π (Størmer).
    #[test]
    fn dipole_matches_stormer_cutoffs() {
        let (r, ring_r, m_tm3) = (1000.0, 10.0, 3.33e6);
        let i = m_tm3 / 1e-7 / (std::f64::consts::PI * ring_r * ring_r);
        let coils = Coils::new(vec![(Loop::new(ring_r, DVec3::ZERO, DVec3::Z, i), Part::Ring)], &[]);
        let spec = ShelterSpec { n: 400, ..ShelterSpec::default() };
        let x = DVec3::new(r, 0.0, 0.0);
        let open = |pq_tm: f64| {
            let p = pq_tm * 299.792458;
            (0..spec.n as u64)
                .filter(|&i| {
                    trace(&coils, x, p, &spec, &mut SmallRng::seed_from_u64(i)) == Fate::Open
                })
                .count() as f64
                / spec.n as f64
        };
        let above = open(1.15 * m_tm3 / (r * r));
        let below = open(0.85 * m_tm3 / (r * (1.0 + 2f64.sqrt())).powi(2));
        assert!(above > 0.99, "open above the cutoff: {above}");
        assert!(below < 0.01, "open below the cutoff: {below}");
    }
}
