//! A thin-shell model of one ICF blast in a ring's field. The blast is a
//! perfectly conducting shell of plasma expanding ballistically from the
//! burn point at the origin: field cannot enter it and is compressed
//! against it. The field outside is the rings' plus that of currents on the
//! shell, which are found each step by making the flux function vanish on
//! the shell — it vanishes on the axis, which the shell meets at its poles.
//! The currents are represented by loops just inside the shell (the method
//! of fundamental solutions). Superconducting rings keep their flux.
//! Magnetic pressure pushes each piece of shell inward along its normal;
//! nothing else acts on it. The poles meet no field — it parts around
//! them — so they run on along the axis as the flanks are held, drawing
//! the shell into fingers. Where a finger necks down to the axis its tip
//! pinches off and flies on as a free jet.

use crate::field::{loop_field, loop_psi, MU0};
use anyhow::{bail, Result};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{f64::consts::PI, time::Instant};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ShellRing {
    pub r: f64,
    pub z: f64,
    /// Current before the blast, MA.
    pub ma: f64,
}

impl Default for ShellRing {
    fn default() -> Self {
        Self { r: 100.0, z: 60.0, ma: 25.0 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ShellSpec {
    pub rings: Vec<ShellRing>,
    /// A uniform field along +z, T, for checks against closed forms.
    pub uniform_t: f64,
    /// Rings keep their current instead of their flux.
    pub fixed_current: bool,
    /// Kinetic energy and mass of the blast's plasma.
    pub energy_j: f64,
    pub mass_kg: f64,
    /// Starting radius of the shell, m.
    pub r0: f64,
    pub nodes: usize,
    pub t_end: f64,
    /// Fraction of the local node spacing a node moves in one step.
    pub courant: f64,
    /// Depth of the shell's current loops inside it, in node spacings.
    pub depth: f64,
    /// Conductor radius: where ring flux is taken, and how near is a hit.
    pub wire_r: f64,
    /// Stop once every node is this far out and moving away.
    pub r_far: f64,
    /// Shapes recorded, evenly in time.
    pub snapshots: usize,
    /// A finger pinches off where it necks to this fraction of its widest.
    pub pinch: f64,
}

impl Default for ShellSpec {
    fn default() -> Self {
        Self {
            rings: vec![ShellRing::default()],
            uniform_t: 0.0,
            fixed_current: false,
            energy_j: 7.7e9,
            mass_kg: 0.01,
            r0: 5.0,
            nodes: 161,
            t_end: 2e-3,
            courant: 0.05,
            depth: 1.0,
            wire_r: 0.5,
            r_far: 1500.0,
            snapshots: 12,
            pinch: 0.3,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ShellReport {
    pub spec: ShellSpec,
    /// Axial momentum carried rearward over the plasma's total momentum.
    pub collimation: f64,
    pub forward_mass: f64,
    pub forward_energy: f64,
    pub plasma_pz: f64,
    /// Impulse the shell's currents gave the rings, N s; the plasma's
    /// momentum and this should cancel.
    pub ring_impulse_z: f64,
    pub momentum_error: f64,
    /// Mass pinched off into jets along the axis, forward and astern, as
    /// fractions of the whole.
    pub jet_forward: f64,
    pub jet_astern: f64,
    /// Farthest the shell reached sideways, forward, and astern, m.
    pub max_r: f64,
    pub max_z: f64,
    pub min_z: f64,
    /// Closest the shell came to a conductor, m.
    pub min_clearance: f64,
    pub peak_pressure_pa: f64,
    pub ring_ma_peak: Vec<f64>,
    /// Worst |B . n| on the shell as a fraction of the strongest field on
    /// it, a check on the field solve, over the steps where the field still
    /// pushes (peak pressure above 1% of the run's).
    pub worst_normal_b: f64,
    pub end: &'static str,
    pub t: f64,
    pub steps: usize,
    pub shapes: Vec<Shape>,
    pub secs: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Shape {
    pub t: f64,
    pub r: Vec<f64>,
    pub z: Vec<f64>,
}

/// Shell geometry: per-node outward normal, tangent, area, and the
/// spacing to its nearer neighbour.
struct Geometry {
    normal: Vec<[f64; 2]>,
    tangent: Vec<[f64; 2]>,
    area: Vec<f64>,
    spacing: Vec<f64>,
}

fn geometry(pos: &[[f64; 2]]) -> Geometry {
    let n = pos.len();
    let mut g = Geometry {
        normal: vec![[0.0; 2]; n],
        tangent: vec![[0.0; 2]; n],
        area: vec![0.0; n],
        spacing: vec![0.0; n],
    };
    let dist = |a: [f64; 2], b: [f64; 2]| (a[0] - b[0]).hypot(a[1] - b[1]);
    let mid = |a: [f64; 2], b: [f64; 2]| [0.5 * (a[0] + b[0]), 0.5 * (a[1] + b[1])];
    for i in 0..n {
        let prev = if i == 0 { [-pos[1][0], pos[1][1]] } else { pos[i - 1] };
        let next = if i == n - 1 { [-pos[n - 2][0], pos[n - 2][1]] } else { pos[i + 1] };
        let (tr, tz) = (next[0] - prev[0], next[1] - prev[1]);
        let l = tr.hypot(tz);
        g.tangent[i] = [tr / l, tz / l];
        g.normal[i] = [-tz / l, tr / l];
        g.spacing[i] = dist(prev, pos[i]).min(dist(pos[i], next));
        let (ma, mb) = (mid(prev, pos[i]), mid(pos[i], next));
        let (ma, mb) = (if i == 0 { pos[i] } else { ma }, if i == n - 1 { pos[i] } else { mb });
        g.area[i] = PI * (ma[0] + pos[i][0]) * dist(ma, pos[i])
            + PI * (pos[i][0] + mb[0]) * dist(pos[i], mb);
    }
    g
}

/// Radius below which a node has reached the axis.
const AXIS_R: f64 = 0.02;

/// The outermost neck, walking in from either pole: a node on the axis, or
/// one whose radius has fallen below `pinch` of the widest between it and
/// its pole once that widest exceeds `min_width`. Returns the node range
/// [lo, hi) to cut off; the node at the cut becomes the new pole.
fn neck(pos: &[[f64; 2]], pinch: f64, min_width: f64) -> Option<(usize, usize)> {
    let n = pos.len();
    let widest = (0..n).max_by(|&a, &b| pos[a][0].total_cmp(&pos[b][0]))?;
    let necked = |r: f64, top: f64| r < AXIS_R || (top > min_width && r < pinch * top);
    let mut top = 0.0f64;
    let mut cut = None;
    for (i, p) in pos.iter().enumerate().take(widest).skip(1) {
        top = top.max(p[0]);
        if necked(p[0], top) {
            cut = Some((0, i));
        }
    }
    if cut.is_some() {
        return cut;
    }
    top = 0.0;
    for i in (widest + 1..n - 1).rev() {
        top = top.max(pos[i][0]);
        if necked(pos[i][0], top) {
            cut = Some((i + 1, n));
        }
    }
    cut
}

/// Currents of the shell's loops and of the rings, and where the loops are.
struct Currents {
    loops: Vec<([f64; 2], f64)>,
    rings: Vec<f64>,
}

impl Currents {
    /// (B_r, B_z) at p from everything, and from the shell's loops alone.
    fn field(&self, spec: &ShellSpec, p: [f64; 2]) -> ([f64; 2], [f64; 2]) {
        let mut own = [0.0; 2];
        for &(q, i) in &self.loops {
            let (br, bz) = loop_field(q[0], i, p[0], p[1] - q[1]);
            own[0] += br;
            own[1] += bz;
        }
        let mut all = [own[0], own[1] + spec.uniform_t];
        for (ring, &i) in spec.rings.iter().zip(&self.rings) {
            let (br, bz) = loop_field(ring.r, i, p[0], p[1] - ring.z);
            all[0] += br;
            all[1] += bz;
        }
        (all, own)
    }
}

fn wire(ring: &ShellRing, spec: &ShellSpec) -> [f64; 2] {
    [ring.r - spec.wire_r, ring.z]
}

fn psi_uniform(spec: &ShellSpec, p: [f64; 2]) -> f64 {
    0.5 * spec.uniform_t * p[0] * p[0]
}

/// Ring flux functions at their wires before the blast.
fn ring_flux(spec: &ShellSpec) -> Vec<f64> {
    spec.rings
        .iter()
        .map(|c| {
            let w = wire(c, spec);
            psi_uniform(spec, w)
                + spec.rings.iter().map(|d| loop_psi(d.r, d.ma * 1e6, w[0], w[1] - d.z)).sum::<f64>()
        })
        .collect()
}

/// Solve for the shell's loop currents, and the rings' currents if they
/// keep their flux, so that psi vanishes at the shell's interior nodes.
fn solve_field(spec: &ShellSpec, pos: &[[f64; 2]], g: &Geometry, flux: &[f64]) -> Result<Currents> {
    let n = pos.len();
    let inner: Vec<usize> = (1..n - 1).collect();
    let sources: Vec<[f64; 2]> = inner
        .iter()
        .map(|&j| {
            let d = (spec.depth * g.spacing[j]).min(0.7 * pos[j][0]);
            let q = [pos[j][0] - d * g.normal[j][0], pos[j][1] - d * g.normal[j][1]];
            [q[0].max(0.25 * pos[j][0]), q[1]]
        })
        .collect();
    let nr = if spec.fixed_current { 0 } else { spec.rings.len() };
    let m = inner.len() + nr;
    let fixed: Vec<f64> = spec.rings.iter().map(|c| c.ma * 1e6).collect();
    let rows: Vec<(Vec<f64>, f64)> = (0..m)
        .into_par_iter()
        .map(|row| {
            let p = if row < inner.len() { pos[inner[row]] } else { wire(&spec.rings[row - inner.len()], spec) };
            let mut a: Vec<f64> =
                sources.iter().map(|q| loop_psi(q[0], 1.0, p[0], p[1] - q[1])).collect();
            let mut rhs = -psi_uniform(spec, p);
            for (c, ring) in spec.rings.iter().enumerate() {
                let unit = loop_psi(ring.r, 1.0, p[0], p[1] - ring.z);
                if spec.fixed_current {
                    rhs -= fixed[c] * unit;
                } else {
                    a.push(unit);
                }
            }
            if row >= inner.len() {
                rhs += flux[row - inner.len()];
            }
            (a, rhs)
        })
        .collect();
    let mut a: Vec<f64> = rows.iter().flat_map(|(r, _)| r.iter().copied()).collect();
    let mut b: Vec<f64> = rows.iter().map(|(_, r)| *r).collect();
    gauss(&mut a, &mut b, m)?;
    Ok(Currents {
        loops: sources.into_iter().zip(b.iter().copied()).collect(),
        rings: if spec.fixed_current { fixed } else { b[inner.len()..].to_vec() },
    })
}

/// Solve a x = b in place by Gaussian elimination with partial pivoting; a
/// is n x n, row-major.
fn gauss(a: &mut [f64], b: &mut [f64], n: usize) -> Result<()> {
    for k in 0..n {
        let p = (k..n).max_by(|&i, &j| a[i * n + k].abs().total_cmp(&a[j * n + k].abs())).unwrap();
        if a[p * n + k] == 0.0 {
            bail!("singular field solve at column {k}")
        }
        if p != k {
            for j in 0..n {
                a.swap(k * n + j, p * n + j);
            }
            b.swap(k, p);
        }
        for i in k + 1..n {
            let f = a[i * n + k] / a[k * n + k];
            if f != 0.0 {
                for j in k..n {
                    a[i * n + j] -= f * a[k * n + j];
                }
                b[i] -= f * b[k];
            }
        }
    }
    for k in (0..n).rev() {
        let s: f64 = (k + 1..n).map(|j| a[k * n + j] * b[j]).sum();
        b[k] = (b[k] - s) / a[k * n + k];
    }
    Ok(())
}

/// Tangential field at the shell's nodes, and the worst normal field as a
/// fraction of the strongest.
fn surface_field(spec: &ShellSpec, cur: &Currents, pos: &[[f64; 2]], g: &Geometry) -> (Vec<f64>, f64) {
    let out: Vec<(f64, f64)> = (0..pos.len())
        .into_par_iter()
        .map(|i| {
            let (b, _) = cur.field(spec, pos[i]);
            let bt = b[0] * g.tangent[i][0] + b[1] * g.tangent[i][1];
            let bn = b[0] * g.normal[i][0] + b[1] * g.normal[i][1];
            (bt, bn.abs())
        })
        .collect();
    let strongest = out.iter().map(|o| o.0.abs()).fold(0.0, f64::max);
    let worst = out[1..out.len() - 1].iter().map(|o| o.1).fold(0.0, f64::max);
    (out.into_iter().map(|o| o.0).collect(), if strongest > 0.0 { worst / strongest } else { 0.0 })
}

/// Axial force on each ring from the shell's currents, N.
fn ring_force(spec: &ShellSpec, cur: &Currents) -> f64 {
    spec.rings
        .iter()
        .zip(&cur.rings)
        .map(|(ring, &i)| {
            let (_, own) = cur.field(spec, [ring.r, ring.z]);
            -i * 2.0 * PI * ring.r * own[0]
        })
        .sum()
}

impl ShellSpec {
    pub fn evaluate(&self) -> Result<ShellReport> {
        if self.nodes < 5 || self.mass_kg <= 0.0 || self.energy_j <= 0.0 {
            bail!("a shell needs at least 5 nodes and positive mass and energy")
        }
        let start = Instant::now();
        let n = self.nodes;
        let v0 = (2.0 * self.energy_j / self.mass_kg).sqrt();
        let theta = |i: usize| PI * i as f64 / (n - 1) as f64;
        let mut pos: Vec<[f64; 2]> =
            (0..n).map(|i| [self.r0 * theta(i).sin(), self.r0 * theta(i).cos()]).collect();
        let mut vel: Vec<[f64; 2]> = (0..n).map(|i| [v0 * theta(i).sin(), v0 * theta(i).cos()]).collect();
        let half = 0.5 * PI / (n - 1) as f64;
        let solid: Vec<f64> = (0..n)
            .map(|i| {
                let (lo, hi) = ((theta(i) - half).max(0.0), (theta(i) + half).min(PI));
                2.0 * PI * (lo.cos() - hi.cos())
            })
            .collect();
        let total: f64 = solid.iter().sum();
        let mut mass: Vec<f64> = solid.iter().map(|s| self.mass_kg * s / total).collect();
        let mut jets: Vec<(f64, f64)> = vec![]; // (mass, axial velocity)
        let flux = ring_flux(self);
        let conductors: Vec<[f64; 2]> = self.rings.iter().map(|c| [c.r, c.z]).collect();

        let mut rep = ShellReport {
            spec: self.clone(),
            collimation: 0.0,
            forward_mass: 0.0,
            forward_energy: 0.0,
            plasma_pz: 0.0,
            ring_impulse_z: 0.0,
            momentum_error: 0.0,
            jet_forward: 0.0,
            jet_astern: 0.0,
            max_r: 0.0,
            max_z: 0.0,
            min_z: 0.0,
            min_clearance: f64::INFINITY,
            peak_pressure_pa: 0.0,
            ring_ma_peak: self.rings.iter().map(|c| c.ma).collect(),
            worst_normal_b: 0.0,
            end: "time",
            t: 0.0,
            steps: 0,
            shapes: vec![],
            secs: 0.0,
        };
        let shot_every = self.t_end / self.snapshots.max(1) as f64;
        let mut next_shot = 0.0;
        while rep.t < self.t_end {
            if let Some((lo, hi)) = neck(&pos, self.pinch, 2.0 * self.r0) {
                for k in (lo..hi).rev() {
                    jets.push((mass[k], vel[k][1]));
                    pos.remove(k);
                    vel.remove(k);
                    mass.remove(k);
                }
                let pole = if lo == 0 { 0 } else { pos.len() - 1 };
                pos[pole][0] = 0.0;
                vel[pole][0] = 0.0;
                if pos.len() < 5 {
                    rep.end = "dispersed";
                    break;
                }
                continue;
            }
            let n = pos.len();
            let g = geometry(&pos);
            let cur = match solve_field(self, &pos, &g, &flux) {
                Ok(c) => c,
                Err(_) => {
                    rep.end = "field solve failed";
                    rep.shapes.push(Shape {
                        t: rep.t,
                        r: pos.iter().map(|p| p[0]).collect(),
                        z: pos.iter().map(|p| p[1]).collect(),
                    });
                    break;
                }
            };
            let (bt, worst) = surface_field(self, &cur, &pos, &g);
            let step_peak = bt.iter().map(|b| b * b / (2.0 * MU0)).fold(0.0, f64::max);
            if step_peak > 0.01 * rep.peak_pressure_pa {
                rep.worst_normal_b = rep.worst_normal_b.max(worst);
            }
            for (peak, &i) in rep.ring_ma_peak.iter_mut().zip(&cur.rings) {
                *peak = peak.max(i / 1e6);
            }
            if rep.t >= next_shot {
                rep.shapes.push(Shape {
                    t: rep.t,
                    r: pos.iter().map(|p| p[0]).collect(),
                    z: pos.iter().map(|p| p[1]).collect(),
                });
                next_shot += shot_every;
            }
            let speed = vel.iter().map(|v| v[0].hypot(v[1])).fold(0.0, f64::max).max(1.0);
            let ds = g.spacing.iter().copied().fold(f64::INFINITY, f64::min);
            let dt = (self.courant * ds / speed).min(self.t_end - rep.t);
            rep.ring_impulse_z += ring_force(self, &cur) * dt;
            for i in 0..n {
                let p = bt[i] * bt[i] / (2.0 * MU0);
                rep.peak_pressure_pa = rep.peak_pressure_pa.max(p);
                let f = p * g.area[i] / mass[i];
                vel[i][0] -= f * g.normal[i][0] * dt;
                vel[i][1] -= f * g.normal[i][1] * dt;
                if i == 0 || i == n - 1 {
                    vel[i][0] = 0.0;
                }
                pos[i][0] = (pos[i][0] + vel[i][0] * dt).max(0.0);
                pos[i][1] += vel[i][1] * dt;
                rep.max_r = rep.max_r.max(pos[i][0]);
                rep.max_z = rep.max_z.max(pos[i][1]);
                rep.min_z = rep.min_z.min(pos[i][1]);
                for c in &conductors {
                    rep.min_clearance = rep.min_clearance.min((pos[i][0] - c[0]).hypot(pos[i][1] - c[1]));
                }
            }
            rep.t += dt;
            rep.steps += 1;
            if rep.min_clearance < self.wire_r {
                rep.end = "hit a conductor";
                break;
            }
            if g.spacing.iter().any(|&s| s < 1e-3 * self.r0) {
                rep.end = "shell folded";
                break;
            }
            let receding = pos.iter().zip(&vel).all(|(p, v)| {
                p[0].hypot(p[1]) > self.r_far && p[0] * v[0] + p[1] * v[1] > 0.0
            });
            if receding {
                rep.end = "escaped";
                break;
            }
        }
        rep.jet_forward = jets.iter().filter(|j| j.1 > 0.0).map(|j| j.0).sum::<f64>() / self.mass_kg;
        rep.jet_astern = jets.iter().filter(|j| j.1 <= 0.0).map(|j| j.0).sum::<f64>() / self.mass_kg;
        for &(m, vz) in &jets {
            vel.push([0.0, vz]);
            mass.push(m);
        }
        rep.plasma_pz = vel.iter().zip(&mass).map(|(v, m)| m * v[1]).sum();
        let p_total = self.mass_kg * v0;
        rep.collimation = -rep.plasma_pz / p_total;
        rep.forward_mass = vel.iter().zip(&mass).filter(|(v, _)| v[1] > 0.0).map(|(_, m)| m).sum::<f64>()
            / self.mass_kg;
        let ke = |v: &[f64; 2], m: f64| 0.5 * m * (v[0] * v[0] + v[1] * v[1]);
        let ke_total: f64 = vel.iter().zip(&mass).map(|(v, &m)| ke(v, m)).sum();
        rep.forward_energy =
            vel.iter().zip(&mass).filter(|(v, _)| v[1] > 0.0).map(|(v, &m)| ke(v, m)).sum::<f64>() / ke_total;
        rep.momentum_error = (rep.plasma_pz + rep.ring_impulse_z).abs() / p_total;
        rep.secs = start.elapsed().as_secs_f64();
        Ok(rep)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A superconducting sphere of radius a in a uniform field B0 has
    /// tangential field 3/2 B0 sin(theta) on its surface.
    #[test]
    fn sphere_in_uniform_field_matches_closed_form() {
        let spec = ShellSpec { rings: vec![], uniform_t: 0.1, nodes: 121, ..ShellSpec::default() };
        let a = 20.0;
        let n = spec.nodes;
        let pos: Vec<[f64; 2]> = (0..n)
            .map(|i| {
                let th = PI * i as f64 / (n - 1) as f64;
                [a * th.sin(), a * th.cos()]
            })
            .collect();
        let g = geometry(&pos);
        let cur = solve_field(&spec, &pos, &g, &ring_flux(&spec)).unwrap();
        let (bt, worst) = surface_field(&spec, &cur, &pos, &g);
        for (i, b) in bt.iter().enumerate().take(n - 1).skip(1) {
            let th = PI * i as f64 / (n - 1) as f64;
            let want = 1.5 * spec.uniform_t * th.sin();
            assert!((b.abs() - want).abs() < 0.01 * spec.uniform_t, "theta {th}: {b} vs {want}");
        }
        assert!(worst < 0.02, "normal field fraction {worst}");
        let area: f64 = g.area.iter().sum();
        assert!((area - 4.0 * PI * a * a).abs() < 1e-3 * area, "area {area}");
    }

    /// Whatever momentum the plasma gains, the rings take the opposite.
    #[test]
    fn plasma_and_ring_momenta_cancel() {
        let spec = ShellSpec { nodes: 61, t_end: 3e-4, ..ShellSpec::default() };
        let r = spec.evaluate().unwrap();
        assert!(r.plasma_pz.abs() > 1e-3 * spec.mass_kg * 1e6, "no momentum exchanged: {}", r.plasma_pz);
        assert!(r.momentum_error < 0.02, "momentum error {}", r.momentum_error);
    }
}
