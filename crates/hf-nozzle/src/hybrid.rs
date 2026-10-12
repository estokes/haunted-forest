//! An axisymmetric hybrid model of one ICF blast in a ring's field: ions
//! are particles moving in three dimensions on an (r, z) grid, electrons a
//! massless fluid. The ring's field is analytic and fixed; the plasma's own
//! field evolves on the grid as a poloidal flux function psi (so div B = 0
//! by construction), by Faraday's law with the electric field from the
//! electrons' Ohm's law,
//!
//!   E = (J x B - J_i x B - grad p_e) / rho + eta J,
//!
//! where rho is the ions' charge density, J_i their current, and J the
//! total current from Ampere's law without displacement current. The Hall
//! term J x B / rho is how the field pushes ions.
//!
//! By default there is no azimuthal field B_phi. Then the flux in plasma
//! cells moves with the ions (E_phi is the motional field), the radial and
//! axial E carry the push, and the exchange of energy between ions and field
//! is exact. Cells with too little plasma are vacuum: their flux is relaxed
//! each step to a current-free field. With `toroidal`, B_phi evolves too and
//! near-vacuum cells take a density floor scaled with the field; that floor
//! carries whistler waves that real vacuum does not, and drains energy into
//! B_phi, so it is kept only for comparison. The field advances in
//! fourth-order Runge-Kutta steps.
//! Electrons are isothermal. The grid's edge sees free space: each step its
//! flux is set by the plasma's azimuthal currents through the loop Green's
//! function, so the plasma neither pushes on nor is held by walls. The
//! current is computed from the field by the
//! same central differences that take the field from psi, and the ions are
//! deposited and pushed with the same weights, so energy passes between
//! ions and field without loss; resistivity is what dissipates it.

use crate::{
    field::{loop_field, loop_psi, MU0},
    shell::ShellRing,
};
use anyhow::{bail, Result};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{f64::consts::PI, time::Instant};

const QE: f64 = 1.602176634e-19;
const AMU: f64 = 1.66053907e-27;

/// One kind of ion in the blast: mass in amu, charge in units of e, and its
/// share of the blast's plasma mass.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Species {
    pub amu: f64,
    pub charge: f64,
    pub mass_frac: f64,
}

/// Fast ions that start at their own energy, such as fusion products.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FastIons {
    pub amu: f64,
    pub charge: f64,
    pub mev: f64,
    /// Real ions in the blast.
    pub count: f64,
    pub particles: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct HybridSpec {
    pub rings: Vec<ShellRing>,
    /// A uniform field along +z, T, for checks.
    pub uniform_t: f64,
    /// Kinetic energy and mass of the blast's bulk plasma.
    pub energy_j: f64,
    pub mass_kg: f64,
    pub species: Vec<Species>,
    pub fast: Vec<FastIons>,
    /// Starting radius of the blast; it expands self-similarly from there.
    pub r0: f64,
    pub dx: f64,
    pub r_max: f64,
    pub z_min: f64,
    pub z_max: f64,
    /// Bulk-plasma macro-particles.
    pub particles: usize,
    pub t_end: f64,
    pub te_ev: f64,
    /// Density floor at the reference field, m^-3; it scales with |B|.
    pub n_floor: f64,
    pub b_ref: f64,
    /// Resistivity everywhere, and the extra in near-vacuum cells, ohm m.
    pub eta: f64,
    pub eta_vac: f64,
    /// Particle step as a fraction of a gyration at b_ref and of a cell.
    pub gyro_frac: f64,
    pub cell_frac: f64,
    /// Field substep as a fraction of the whistler limit at the floor.
    pub whistler_frac: f64,
    /// Conductor radius: cells within this of the ring are inside it.
    pub wire_r: f64,
    pub seed: u64,
    /// Start every ion with a twin mirrored through z = 0, for checks.
    pub mirror: bool,
    /// The grid's edge sees free space; otherwise it is a superconducting box.
    pub open: bool,
    /// Evolve B_phi with a whistler-carrying density floor (see the module
    /// note); otherwise B_phi = 0 and vacuum cells hold a current-free field.
    pub toroidal: bool,
    /// Below this ion density, m^-3, a cell is vacuum.
    pub n_vac: f64,
    /// Over-relaxation sweeps of the vacuum field per particle step.
    pub sor_iters: usize,
    pub sor_omega: f64,
    /// Passes of the 1-2-1 filter on moments, and on the fields the ions
    /// feel; the same filter both ways keeps the exchange of energy fair.
    pub smooth: usize,
    /// Density maps recorded, evenly in time.
    pub snapshots: usize,
}

impl Default for HybridSpec {
    fn default() -> Self {
        Self {
            rings: vec![ShellRing { r: 100.0, z: 100.0, ma: 25.0 }],
            uniform_t: 0.0,
            energy_j: 7.7e9,
            mass_kg: 0.01,
            species: vec![
                Species { amu: 1.008, charge: 1.0, mass_frac: 0.112 },
                Species { amu: 15.999, charge: 1.0, mass_frac: 0.888 },
            ],
            fast: vec![],
            r0: 10.0,
            dx: 5.0,
            r_max: 500.0,
            z_min: -700.0,
            z_max: 400.0,
            particles: 200_000,
            t_end: 1.5e-3,
            te_ev: 5.0,
            n_floor: 1e16,
            b_ref: 0.1,
            eta: 0.0,
            eta_vac: 0.0,
            gyro_frac: 0.2,
            cell_frac: 0.4,
            whistler_frac: 0.5,
            wire_r: 6.0,
            seed: 1,
            mirror: false,
            open: true,
            toroidal: false,
            n_vac: 1e15,
            sor_iters: 40,
            sor_omega: 1.85,
            smooth: 1,
            snapshots: 8,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct HybridReport {
    pub spec: HybridSpec,
    /// Axial momentum carried astern over the ions' total momentum.
    pub collimation: f64,
    /// Fractions of ion mass and kinetic energy moving forward at the end.
    pub forward_mass: f64,
    pub forward_energy: f64,
    /// Fraction of ion energy that struck the conductor.
    pub conductor_energy: f64,
    pub plasma_pz: f64,
    pub ring_impulse_z: f64,
    pub momentum_error: f64,
    /// Kinetic energy left in ions over the start; peak rise in magnetic
    /// energy over the ions' starting energy; and the worst shortfall of the
    /// two together, a check on energy conservation.
    pub energy_left: f64,
    pub peak_field_energy: f64,
    pub worst_energy_error: f64,
    /// Energy ledger over the run, J: the ions' kinetic energy change, the
    /// work J_i . E on them at the grid, the field's energy change, and
    /// -J . E summed over the field substeps.
    pub ledger: [f64; 4],
    /// At each map: time, ions' axial momentum, ring impulse so far, ions'
    /// kinetic energy, rise in poloidal field energy, and B_phi energy, all
    /// in kg m/s or J.
    pub history: Vec<[f64; 6]>,
    /// Radius by which 90% of ions in the |z| < 20 m slab lie, at its peak.
    pub max_equator_r: f64,
    pub particle_steps: usize,
    pub field_steps: usize,
    pub t: f64,
    pub maps: Vec<Map>,
    pub secs: f64,
}

/// Ion density on a coarse grid, m^-3, and B_z at the burn point, T.
#[derive(Clone, Debug, Serialize)]
pub struct Map {
    pub t: f64,
    pub bz_center: f64,
    pub dr: f64,
    pub z0: f64,
    pub nr: usize,
    pub nz: usize,
    pub n: Vec<f64>,
}

#[derive(Clone, Copy)]
struct Ion {
    r: f64,
    z: f64,
    v: [f64; 3], // r, phi, z
    /// Real ions it stands for, their mass and charge.
    w: f64,
    m: f64,
    q: f64,
    alive: bool,
}

/// The (r, z) node grid: r = i dx for i in 0..=nr, z = z0 + k dx.
struct Grid {
    nr: usize,
    nz: usize,
    dx: f64,
    z0: f64,
}

impl Grid {
    fn len(&self) -> usize {
        (self.nr + 1) * (self.nz + 1)
    }
    fn at(&self, i: usize, k: usize) -> usize {
        k * (self.nr + 1) + i
    }
    fn r(&self, i: usize) -> f64 {
        i as f64 * self.dx
    }
    fn z(&self, k: usize) -> f64 {
        self.z0 + k as f64 * self.dx
    }
    /// Node volume of the axisymmetric cell around node i.
    fn volume(&self, i: usize) -> f64 {
        let d = self.dx;
        if i == 0 { PI * 0.25 * d * d * d } else { 2.0 * PI * self.r(i) * d * d }
    }
    /// Lower node and fractions for bilinear weights, if inside.
    fn locate(&self, r: f64, z: f64) -> Option<(usize, usize, f64, f64)> {
        let (fr, fz) = (r / self.dx, (z - self.z0) / self.dx);
        if fr < 0.0 || fz < 0.0 || fr >= self.nr as f64 || fz >= self.nz as f64 {
            return None;
        }
        let (i, k) = (fr as usize, fz as usize);
        Some((i, k, fr - i as f64, fz - k as f64))
    }
}

/// Moments of the ions on the grid.
struct Moments {
    rho: Vec<f64>,
    ji: [Vec<f64>; 3],
}

/// Fields everything needs at nodes: total B and E.
struct Derived {
    b: [Vec<f64>; 3],
    e: [Vec<f64>; 3],
    jphi: Vec<f64>,
}

struct Sim<'a> {
    spec: &'a HybridSpec,
    g: Grid,
    /// The ring's field at nodes: (r, phi, z), phi always zero.
    bc: [Vec<f64>; 3],
    /// Nodes inside a conductor: fields there are held.
    solid: Vec<bool>,
    /// Edge nodes off the axis, interior nodes, and the flux at each edge
    /// node per unit J_phi at each interior node (edge-major), for a
    /// free-space edge.
    edge: Vec<usize>,
    inner: Vec<usize>,
    green: Vec<f64>,
}

impl Sim<'_> {
    fn new<'a>(spec: &'a HybridSpec, g: Grid, bc: [Vec<f64>; 3], solid: Vec<bool>) -> Sim<'a> {
        let (nr, nz) = (g.nr, g.nz);
        let mut edge: Vec<usize> = (0..=nz).map(|k| g.at(nr, k)).collect();
        edge.extend((1..nr).flat_map(|i| [g.at(i, 0), g.at(i, nz)]));
        let inner: Vec<usize> = (1..nz).flat_map(|k| (1..nr).map(move |i| (i, k))).map(|(i, k)| g.at(i, k)).collect();
        let green = if spec.open {
            let area = g.dx * g.dx;
            let pos = |x: usize| (g.r(x % (nr + 1)), g.z(x / (nr + 1)));
            edge.par_iter()
                .flat_map_iter(|&b| {
                    let (rb, zb) = pos(b);
                    inner.iter().map(move |&s| {
                        let (rs, zs) = pos(s);
                        area * loop_psi(rs, 1.0, rb, zb - zs)
                    })
                })
                .collect()
        } else {
            vec![]
        };
        Sim { spec, g, bc, solid, edge, inner, green }
    }

    /// Set the edge's flux to what the real currents give in free space:
    /// those in plasma above the floor or beside a conductor, kept clear of
    /// the edge so the edge cannot feed itself.
    fn open_edge(&self, psi: &mut [f64], jphi: &[f64], rho: &[f64]) {
        if self.green.is_empty() {
            return;
        }
        let g = &self.g;
        let m = self.inner.len();
        let floor = QE * self.spec.n_floor;
        let near_solid = |x: usize| {
            let (i, k) = (x % (g.nr + 1), x / (g.nr + 1));
            (i.saturating_sub(2)..=(i + 2).min(g.nr))
                .any(|ii| (k.saturating_sub(2)..=(k + 2).min(g.nz)).any(|kk| self.solid[g.at(ii, kk)]))
        };
        let src: Vec<f64> = self
            .inner
            .iter()
            .map(|&s| {
                let (i, k) = (s % (g.nr + 1), s / (g.nr + 1));
                let clear = i + 3 < g.nr && k >= 3 && k + 3 < g.nz;
                if clear && (rho[s] > floor || near_solid(s)) { jphi[s] } else { 0.0 }
            })
            .collect();
        let vals: Vec<f64> = (0..self.edge.len())
            .into_par_iter()
            .map(|b| self.green[b * m..(b + 1) * m].iter().zip(&src).map(|(g, j)| g * j).sum())
            .collect();
        for (&b, v) in self.edge.iter().zip(vals) {
            psi[b] = v;
        }
    }
}


impl Sim<'_> {
    fn derived(&self, psi: &[f64], bphi: &[f64], m: &Moments) -> Derived {
        let g = &self.g;
        let (nr, nz, d) = (g.nr, g.nz, g.dx);
        let n = g.len();
        let mut bp = [vec![0.0; n], vec![0.0; n]];
        // plasma's poloidal field from psi
        bp.iter_mut().enumerate().for_each(|(c, out)| {
            out.par_chunks_mut(nr + 1).enumerate().for_each(|(k, row)| {
                if k == 0 || k == nz {
                    return;
                }
                for (i, o) in row.iter_mut().enumerate().take(nr) {
                    let p = |ii: usize, kk: usize| psi[g.at(ii, kk)];
                    *o = if i == 0 {
                        if c == 0 { 0.0 } else { 2.0 * (16.0 * p(1, k) - p(2, k)) / (12.0 * d * d) }
                    } else if c == 0 {
                        -(p(i, k + 1) - p(i, k - 1)) / (2.0 * d) / g.r(i)
                    } else {
                        (p(i + 1, k) - p(i - 1, k)) / (2.0 * d) / g.r(i)
                    };
                }
            })
        });
        let b = [
            (0..n).map(|x| self.bc[0][x] + bp[0][x]).collect::<Vec<_>>(),
            bphi.to_vec(),
            (0..n).map(|x| self.bc[2][x] + bp[1][x] + self.spec.uniform_t).collect::<Vec<_>>(),
        ];
        // the plasma's current, J = curl B_plasma / mu0, by the same central
        // differences that took B from psi: then the field's energy changes
        // by exactly -J . E, summed over the grid
        let mut j = [vec![0.0; n], vec![0.0; n], vec![0.0; n]];
        let rows: Vec<(usize, Vec<[f64; 3]>)> = (1..nz)
            .into_par_iter()
            .map(|k| {
                let row = (0..nr)
                    .map(|i| {
                        let f = |ii: usize, kk: usize| bphi[g.at(ii, kk)];
                        if i == 0 {
                            return [0.0, 0.0, 2.0 * f(1, k) / d / MU0];
                        }
                        let r = g.r(i);
                        let jphi = ((bp[0][g.at(i, k + 1)] - bp[0][g.at(i, k - 1)])
                            - (bp[1][g.at(i + 1, k)] - bp[1][g.at(i - 1, k)]))
                            / (2.0 * d * MU0);
                        let jr = -(f(i, k + 1) - f(i, k - 1)) / (2.0 * d) / MU0;
                        let jz = ((r + d) * f(i + 1, k) - (r - d) * f(i - 1, k)) / (2.0 * d * r) / MU0;
                        [jr, jphi, jz]
                    })
                    .collect();
                (k, row)
            })
            .collect();
        for (k, row) in rows {
            for (i, v) in row.into_iter().enumerate() {
                let x = g.at(i, k);
                if !self.solid[x] {
                    for c in 0..3 {
                        j[c][x] = v[c];
                    }
                }
            }
        }
        // Ohm's law. The pressure term is -T_e grad ln(rho) with the floored
        // rho, the gradient of a scalar, so it adds no curl to E.
        let te = self.spec.te_ev;
        let floor_at = |x: usize| {
            let bmag = (b[0][x] * b[0][x] + b[1][x] * b[1][x] + b[2][x] * b[2][x]).sqrt();
            QE * self.spec.n_floor * (bmag / self.spec.b_ref).max(1e-3)
        };
        let ln_rho: Vec<f64> = (0..n).into_par_iter().map(|x| m.rho[x].max(floor_at(x)).ln()).collect();
        let mut e = [vec![0.0; n], vec![0.0; n], vec![0.0; n]];
        let rows: Vec<(usize, Vec<[f64; 3]>)> = (1..nz)
            .into_par_iter()
            .map(|k| {
                let row = (0..nr)
                    .map(|i| {
                        let x = g.at(i, k);
                        if self.solid[x] {
                            return [0.0; 3];
                        }
                        let bv = [b[0][x], b[1][x], b[2][x]];
                        let floor = floor_at(x);
                        let rho = m.rho[x].max(floor);
                        let jv = [j[0][x], j[1][x], j[2][x]];
                        let ji = [m.ji[0][x], m.ji[1][x], m.ji[2][x]];
                        let cross = |a: [f64; 3], c: [f64; 3]| {
                            [a[1] * c[2] - a[2] * c[1], a[2] * c[0] - a[0] * c[2], a[0] * c[1] - a[1] * c[0]]
                        };
                        let jxb = cross(jv, bv);
                        let jixb = cross(ji, bv);
                        let vac = (1.0 - m.rho[x] / floor).max(0.0);
                        let eta = self.spec.eta + self.spec.eta_vac * vac;
                        let grad_r = if i == 0 { 0.0 } else { (ln_rho[g.at(i + 1, k)] - ln_rho[g.at(i - 1, k)]) / (2.0 * d) };
                        let grad_z = (ln_rho[g.at(i, k + 1)] - ln_rho[g.at(i, k - 1)]) / (2.0 * d);
                        let mut ev = [0.0; 3];
                        for c in 0..3 {
                            ev[c] = (jxb[c] - jixb[c]) / rho + eta * jv[c];
                        }
                        ev[0] -= te * grad_r;
                        ev[2] -= te * grad_z;
                        if i == 0 {
                            ev[0] = 0.0;
                            ev[1] = 0.0;
                        }
                        ev
                    })
                    .collect();
                (k, row)
            })
            .collect();
        for (k, row) in rows {
            for (i, v) in row.into_iter().enumerate() {
                let x = g.at(i, k);
                for c in 0..3 {
                    e[c][x] = v[c];
                }
            }
        }
        let jphi = std::mem::take(&mut j[1]);
        Derived { b, e, jphi }
    }

    /// Time derivatives of psi and B_phi.
    fn rates(&self, psi: &[f64], bphi: &[f64], m: &Moments, vac: &[bool]) -> (Vec<f64>, Vec<f64>) {
        let toroidal = self.spec.toroidal;
        let g = &self.g;
        let (nr, nz, d) = (g.nr, g.nz, g.dx);
        let dv = self.derived(psi, bphi, m);
        let mut dpsi = vec![0.0; g.len()];
        let mut dbphi = vec![0.0; g.len()];
        dpsi.par_chunks_mut(nr + 1).zip(dbphi.par_chunks_mut(nr + 1)).enumerate().for_each(
            |(k, (rp, rb))| {
                if k == 0 || k == nz {
                    return;
                }
                for i in 1..nr {
                    let x = g.at(i, k);
                    if self.solid[x] || (!toroidal && vac[x]) {
                        continue;
                    }
                    rp[i] = -g.r(i) * dv.e[1][x];
                    if !toroidal {
                        continue;
                    }
                    let ez_r = (dv.e[2][g.at(i + 1, k)] - dv.e[2][g.at(i - 1, k)]) / (2.0 * d);
                    let er_z = (dv.e[0][g.at(i, k + 1)] - dv.e[0][g.at(i, k - 1)]) / (2.0 * d);
                    rb[i] = ez_r - er_z;
                }
            },
        );
        (dpsi, dbphi)
    }

    /// One RK4 substep of the field; vacuum cells are left to `relax`
    /// unless B_phi is evolved.
    fn advance(&self, psi: &mut [f64], bphi: &mut [f64], m: &Moments, vac: &[bool], dt: f64) {
        let n = psi.len();
        let step = |base: &[f64], k: &[f64], h: f64| -> Vec<f64> { (0..n).map(|x| base[x] + h * k[x]).collect() };
        let (k1p, k1b) = self.rates(psi, bphi, m, vac);
        let (p2, b2) = (step(psi, &k1p, 0.5 * dt), step(bphi, &k1b, 0.5 * dt));
        let (k2p, k2b) = self.rates(&p2, &b2, m, vac);
        let (p3, b3) = (step(psi, &k2p, 0.5 * dt), step(bphi, &k2b, 0.5 * dt));
        let (k3p, k3b) = self.rates(&p3, &b3, m, vac);
        let (p4, b4) = (step(psi, &k3p, dt), step(bphi, &k3b, dt));
        let (k4p, k4b) = self.rates(&p4, &b4, m, vac);
        for x in 0..n {
            psi[x] += dt / 6.0 * (k1p[x] + 2.0 * k2p[x] + 2.0 * k3p[x] + k4p[x]);
            bphi[x] += dt / 6.0 * (k1b[x] + 2.0 * k2b[x] + 2.0 * k3b[x] + k4b[x]);
        }
    }

    /// Relax the plasma's flux in vacuum cells toward a current-free field,
    /// Delta* psi = 0, holding plasma, conductor, axis and edge values, by
    /// red-black over-relaxation.
    fn relax(&self, psi: &mut [f64], vac: &[bool]) {
        let g = &self.g;
        let (nr, nz) = (g.nr, g.nz);
        let w = self.spec.sor_omega;
        for _ in 0..self.spec.sor_iters {
            for colour in 0..2 {
                let updates: Vec<(usize, f64)> = (1..nz)
                    .into_par_iter()
                    .flat_map_iter(|k| {
                        let psi = &*psi;
                        (1..nr).filter(move |i| (i + k) % 2 == colour).filter_map(move |i| {
                            let x = g.at(i, k);
                            if !vac[x] {
                                return None;
                            }
                            let h = g.dx / (2.0 * g.r(i));
                            let gs = 0.25
                                * ((1.0 - h) * psi[x + 1] + (1.0 + h) * psi[x - 1] + psi[g.at(i, k + 1)] + psi[g.at(i, k - 1)]);
                            Some((x, psi[x] + w * (gs - psi[x])))
                        })
                    })
                    .collect();
                for (x, v) in updates {
                    psi[x] = v;
                }
            }
        }
    }

    fn moments(&self, ions: &[Ion]) -> Moments {
        let g = &self.g;
        let n = g.len();
        let (rho, ji) = ions
            .par_chunks(4096)
            .map(|chunk| {
                let mut rho = vec![0.0; n];
                let mut ji = [vec![0.0; n], vec![0.0; n], vec![0.0; n]];
                for p in chunk.iter().filter(|p| p.alive) {
                    let Some((i, k, fr, fz)) = g.locate(p.r, p.z) else { continue };
                    let q = p.q * QE * p.w;
                    for (di, dk, wt) in [(0, 0, (1.0 - fr) * (1.0 - fz)), (1, 0, fr * (1.0 - fz)), (0, 1, (1.0 - fr) * fz), (1, 1, fr * fz)] {
                        let x = g.at(i + di, k + dk);
                        let s = q * wt / g.volume(i + di);
                        rho[x] += s;
                        for (j, v) in ji.iter_mut().zip(p.v) {
                            j[x] += s * v;
                        }
                    }
                }
                (rho, ji)
            })
            .reduce(
                || (vec![0.0; n], [vec![0.0; n], vec![0.0; n], vec![0.0; n]]),
                |(mut ra, mut ja), (rb, jb)| {
                    for x in 0..n {
                        ra[x] += rb[x];
                        for c in 0..3 {
                            ja[c][x] += jb[c][x];
                        }
                    }
                    (ra, ja)
                },
            );
        let passes = self.spec.smooth;
        Moments {
            rho: smooth(g, &rho, passes),
            ji: [smooth(g, &ji[0], passes), smooth(g, &ji[1], passes), smooth(g, &ji[2], passes)],
        }
    }

    fn interp(&self, f: &[Vec<f64>; 3], r: f64, z: f64) -> [f64; 3] {
        let Some((i, k, fr, fz)) = self.g.locate(r, z) else { return [0.0; 3] };
        let mut out = [0.0; 3];
        for (di, dk, wt) in [(0, 0, (1.0 - fr) * (1.0 - fz)), (1, 0, fr * (1.0 - fz)), (0, 1, (1.0 - fr) * fz), (1, 1, fr * fz)] {
            let x = self.g.at(i + di, k + dk);
            for c in 0..3 {
                out[c] += wt * f[c][x];
            }
        }
        out
    }

    /// Sum over the grid of J . E and J_i . E, W, at the given field.
    fn work(&self, psi: &[f64], bphi: &[f64], m: &Moments) -> (f64, f64) {
        let dv = self.derived(psi, bphi, m);
        let g = &self.g;
        let (nr, nz, d) = (g.nr, g.nz, g.dx);
        let mut je = 0.0;
        for k in 1..nz {
            for i in 1..nr {
                let x = g.at(i, k);
                let bp = |c: usize, ii: usize, kk: usize| {
                    let y = g.at(ii, kk);
                    if c == 0 { dv.b[0][y] - self.bc[0][y] } else { dv.b[2][y] - self.bc[2][y] - self.spec.uniform_t }
                };
                let r = g.r(i);
                let jphi = ((bp(0, i, k + 1) - bp(0, i, k - 1)) - (bp(1, i + 1, k) - bp(1, i - 1, k))) / (2.0 * d * MU0);
                let jr = -(dv.b[1][g.at(i, k + 1)] - dv.b[1][g.at(i, k - 1)]) / (2.0 * d * MU0);
                let jz = ((r + d) * dv.b[1][g.at(i + 1, k)] - (r - d) * dv.b[1][g.at(i - 1, k)]) / (2.0 * d * r * MU0);
                if !self.solid[x] {
                    je += (jr * dv.e[0][x] + jphi * dv.e[1][x] + jz * dv.e[2][x]) * g.volume(i);
                }
            }
        }
        (je, self.work_felt(&dv.e, m).1)
    }

    fn work_felt(&self, e: &[Vec<f64>; 3], m: &Moments) -> (f64, f64) {
        let g = &self.g;
        let mut jie = 0.0;
        for k in 1..g.nz {
            for i in 0..g.nr {
                let x = g.at(i, k);
                jie += (m.ji[0][x] * e[0][x] + m.ji[1][x] * e[1][x] + m.ji[2][x] * e[2][x]) * g.volume(i);
            }
        }
        (0.0, jie)
    }

    /// Axial force on each ring, N, from the Maxwell stress on a square
    /// contour around its conductor: the ring's own field exactly, the
    /// plasma's from the grid. The ring's field alone gives no net force.
    fn ring_force(&self, dv: &Derived) -> f64 {
        let h = self.spec.wire_r + 2.0 * self.g.dx;
        let steps = 64;
        let mut f = 0.0;
        for ring in &self.spec.rings {
            // corners counter-clockwise in (r, z); outward normals
            let sides = [
                ([ring.r - h, ring.z - h], [ring.r + h, ring.z - h], [0.0, -1.0]),
                ([ring.r + h, ring.z - h], [ring.r + h, ring.z + h], [1.0, 0.0]),
                ([ring.r + h, ring.z + h], [ring.r - h, ring.z + h], [0.0, 1.0]),
                ([ring.r - h, ring.z + h], [ring.r - h, ring.z - h], [-1.0, 0.0]),
            ];
            for (a, b, nrm) in sides {
                let len = 2.0 * h;
                for s in 0..steps {
                    let t = (s as f64 + 0.5) / steps as f64;
                    let (r, z) = (a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1]));
                    let grid = self.interp(&dv.b, r, z);
                    let coil = self.interp(&self.bc, r, z);
                    let mut exact = [0.0, self.spec.uniform_t];
                    for c in &self.spec.rings {
                        let (br, bz) = loop_field(c.r, c.ma * 1e6, r, z - c.z);
                        exact[0] += br;
                        exact[1] += bz;
                    }
                    let (br, bphi, bz) = (grid[0] - coil[0] + exact[0], grid[1], grid[2] - coil[2] - self.spec.uniform_t + exact[1]);
                    let tzz = (bz * bz - br * br - bphi * bphi) / (2.0 * MU0);
                    let tzr = br * bz / MU0;
                    f += (tzz * nrm[1] + tzr * nrm[0]) * 2.0 * PI * r * len / steps as f64;
                }
            }
        }
        f
    }
}

/// Passes of a 1-2-1 filter in r and z, keeping boundary values.
fn smooth(g: &Grid, f: &[f64], passes: usize) -> Vec<f64> {
    (0..passes).fold(f.to_vec(), |f, _| smooth_once(g, &f))
}

fn smooth_once(g: &Grid, f: &[f64]) -> Vec<f64> {
    let (nr, nz) = (g.nr, g.nz);
    let mut a = f.to_vec();
    for k in 0..=nz {
        for i in 1..nr {
            let x = g.at(i, k);
            a[x] = 0.25 * f[x - 1] + 0.5 * f[x] + 0.25 * f[x + 1];
        }
    }
    let mut b = a.clone();
    for k in 1..nz {
        for i in 0..=nr {
            let x = g.at(i, k);
            b[x] = 0.25 * a[g.at(i, k - 1)] + 0.5 * a[x] + 0.25 * a[g.at(i, k + 1)];
        }
    }
    b
}

/// Boris push of one ion in fields given in its (r, phi, z) frame, then
/// back onto the (r, z) half-plane.
fn push(p: &mut Ion, e: [f64; 3], b: [f64; 3], dt: f64) {
    let qm = p.q * QE / p.m * 0.5 * dt;
    let vm = [p.v[0] + qm * e[0], p.v[1] + qm * e[1], p.v[2] + qm * e[2]];
    let t = [qm * b[0], qm * b[1], qm * b[2]];
    let t2 = t[0] * t[0] + t[1] * t[1] + t[2] * t[2];
    let cross = |a: [f64; 3], c: [f64; 3]| [a[1] * c[2] - a[2] * c[1], a[2] * c[0] - a[0] * c[2], a[0] * c[1] - a[1] * c[0]];
    let vp = {
        let c = cross(vm, t);
        [vm[0] + c[0], vm[1] + c[1], vm[2] + c[2]]
    };
    let s = [2.0 * t[0] / (1.0 + t2), 2.0 * t[1] / (1.0 + t2), 2.0 * t[2] / (1.0 + t2)];
    let c = cross(vp, s);
    let v = [vm[0] + c[0] + qm * e[0], vm[1] + c[1] + qm * e[1], vm[2] + c[2] + qm * e[2]];
    // move in Cartesian with x along the old r, y along phi
    let (x, y) = (p.r + v[0] * dt, v[1] * dt);
    let r = x.hypot(y);
    let (ca, sa) = if r > 0.0 { (x / r, y / r) } else { (1.0, 0.0) };
    p.v = [v[0] * ca + v[1] * sa, -v[0] * sa + v[1] * ca, v[2]];
    p.r = r;
    p.z += v[2] * dt;
}

impl HybridSpec {
    pub fn evaluate(&self) -> Result<HybridReport> {
        if self.dx <= 0.0 || self.r_max <= 4.0 * self.dx || self.z_max <= self.z_min + 8.0 * self.dx {
            bail!("the grid needs a positive cell and room for a few cells each way")
        }
        if self.species.is_empty() || self.particles == 0 {
            bail!("the blast needs at least one species and some particles")
        }
        let start = Instant::now();
        let g = Grid {
            nr: (self.r_max / self.dx).round() as usize,
            nz: ((self.z_max - self.z_min) / self.dx).round() as usize,
            dx: self.dx,
            z0: self.z_min,
        };
        let n = g.len();
        let mut bc = [vec![0.0; n], vec![0.0; n], vec![0.0; n]];
        let mut solid = vec![false; n];
        for k in 0..=g.nz {
            for i in 0..=g.nr {
                let x = g.at(i, k);
                for ring in &self.rings {
                    let (r, z) = (g.r(i), g.z(k));
                    if (r - ring.r).hypot(z - ring.z) < self.wire_r {
                        solid[x] = true;
                    }
                    let (br, bz) = loop_field(ring.r, ring.ma * 1e6, r, z - ring.z);
                    bc[0][x] += br;
                    bc[2][x] += bz;
                }
            }
        }
        for x in 0..n {
            if solid[x] {
                bc[0][x] = 0.0;
                bc[2][x] = 0.0;
            }
        }
        let sim = Sim::new(self, g, bc, solid);
        let g = &sim.g;

        // the blast: uniform sphere, velocity growing with radius
        let mut rng = SmallRng::seed_from_u64(self.seed);
        let v_edge = (10.0 * self.energy_j / (3.0 * self.mass_kg)).sqrt();
        let frac_total: f64 = self.species.iter().map(|s| s.mass_frac).sum();
        let mut ions = Vec::new();
        for s in &self.species {
            let count = ((self.particles as f64) * s.mass_frac / frac_total).round().max(1.0) as usize;
            let m = s.amu * AMU;
            let w = self.mass_kg * s.mass_frac / frac_total / m / count as f64;
            for _ in 0..count {
                let dir = isotropic(&mut rng);
                let rad = self.r0 * rng.random::<f64>().cbrt();
                ions.push(blast_ion(dir, rad, v_edge * rad / self.r0, w, m, s.charge));
                if self.mirror {
                    ions.push(blast_ion([dir[0], dir[1], -dir[2]], rad, v_edge * rad / self.r0, w, m, s.charge));
                }
            }
        }
        for f in &self.fast {
            let m = f.amu * AMU;
            let speed = (2.0 * f.mev * 1e6 * QE / m).sqrt();
            let w = f.count / f.particles.max(1) as f64;
            for _ in 0..f.particles {
                let dir = isotropic(&mut rng);
                let rad = self.r0 * rng.random::<f64>().cbrt();
                ions.push(blast_ion(dir, rad, speed, w, m, f.charge));
                if self.mirror {
                    ions.push(blast_ion([dir[0], dir[1], -dir[2]], rad, speed, w, m, f.charge));
                }
            }
        }
        let ke = |p: &Ion| 0.5 * p.w * p.m * (p.v[0] * p.v[0] + p.v[1] * p.v[1] + p.v[2] * p.v[2]);
        let mom = |p: &Ion| p.w * p.m * (p.v[0] * p.v[0] + p.v[1] * p.v[1] + p.v[2] * p.v[2]).sqrt();
        let ke0: f64 = ions.iter().map(ke).sum();
        let p0: f64 = ions.iter().map(mom).sum();
        let pz0: f64 = ions.iter().map(|p| p.w * p.m * p.v[2]).sum();

        let mut psi = vec![0.0; n];
        let mut bphi = vec![0.0; n];
        let mut rep = HybridReport {
            spec: self.clone(),
            collimation: 0.0,
            forward_mass: 0.0,
            forward_energy: 0.0,
            conductor_energy: 0.0,
            plasma_pz: 0.0,
            ring_impulse_z: 0.0,
            momentum_error: 0.0,
            energy_left: 0.0,
            peak_field_energy: 0.0,
            worst_energy_error: 0.0,
            ledger: [0.0; 4],
            history: vec![],
            max_equator_r: 0.0,
            particle_steps: 0,
            field_steps: 0,
            t: 0.0,
            maps: vec![],
            secs: 0.0,
        };
        // escaped ions: (mass, pz, ke, forward)
        let mut gone: Vec<(f64, f64, f64, bool)> = vec![];
        let speed_max = ions.iter().map(|p| p.v[0].hypot(p.v[2]).hypot(p.v[1])).fold(0.0, f64::max);
        let qm_max = ions.iter().map(|p| p.q * QE / p.m).fold(0.0, f64::max);
        let dt_p = (self.gyro_frac / (qm_max * self.b_ref)).min(self.cell_frac * self.dx / speed_max);
        // whistler: omega = k^2 B / (mu0 e n); with the floor scaling as B the limit is B-free
        let k2 = (PI / self.dx).powi(2) * 2.0;
        let omega_w = k2 * self.b_ref / (MU0 * QE * self.n_floor);
        let dt_f_max = self.whistler_frac * 2.0 / omega_w;
        let shot_every = self.t_end / self.snapshots.max(1) as f64;
        let mut next_shot = 0.0;
        while rep.t < self.t_end {
            let m = sim.moments(&ions);
            if rep.t >= next_shot {
                let dv = sim.derived(&psi, &bphi, &m);
                let bz_center = sim.interp(&dv.b, 0.0, 0.0)[2];
                let (mut wpol, mut wphi) = (0.0, 0.0);
                for k in 1..g.nz {
                    for i in 1..g.nr {
                        let x = g.at(i, k);
                        if sim.solid[x] {
                            continue;
                        }
                        let (b0r, b0z) = (sim.bc[0][x], sim.bc[2][x] + self.uniform_t);
                        wpol += (dv.b[0][x].powi(2) + dv.b[2][x].powi(2) - b0r * b0r - b0z * b0z) / (2.0 * MU0) * g.volume(i);
                        wphi += dv.b[1][x].powi(2) / (2.0 * MU0) * g.volume(i);
                    }
                }
                let live_pz: f64 = ions.iter().filter(|p| p.alive).map(|p| p.w * p.m * p.v[2]).sum::<f64>()
                    + gone.iter().map(|g| g.1).sum::<f64>();
                let live_ke: f64 = ions.iter().filter(|p| p.alive).map(ke).sum::<f64>() + gone.iter().map(|g| g.2).sum::<f64>();
                rep.history.push([rep.t, live_pz - pz0, rep.ring_impulse_z, live_ke, wpol, wphi]);
                rep.maps.push(Map { t: rep.t, bz_center, ..map(g, &m) });
                next_shot += shot_every;
            }
            // field substeps over this particle step
            let vac: Vec<bool> =
                (0..n).map(|x| !sim.solid[x] && m.rho[x] < QE * self.n_vac).collect();
            let subs = if self.toroidal { (dt_p / dt_f_max).ceil().max(1.0) as usize } else { 1 };
            let dt_f = dt_p / subs as f64;
            let wb = |psi: &[f64], bphi: &[f64]| {
                let dv = sim.derived(psi, bphi, &m);
                let mut w = 0.0;
                for k in 1..g.nz {
                    for i in 1..g.nr {
                        let x = g.at(i, k);
                        if sim.solid[x] {
                            continue;
                        }
                        let (b0r, b0z) = (sim.bc[0][x], sim.bc[2][x] + self.uniform_t);
                        let b2 = dv.b[0][x].powi(2) + dv.b[1][x].powi(2) + dv.b[2][x].powi(2);
                        w += (b2 - b0r * b0r - b0z * b0z) / (2.0 * MU0) * g.volume(i);
                    }
                }
                w
            };
            let wb0 = wb(&psi, &bphi);
            for _ in 0..subs {
                let (je, _) = sim.work(&psi, &bphi, &m);
                rep.ledger[3] -= je * dt_f;
                sim.advance(&mut psi, &mut bphi, &m, &vac, dt_f);
            }
            rep.ledger[2] += wb(&psi, &bphi) - wb0;
            rep.field_steps += subs;
            let dv = sim.derived(&psi, &bphi, &m);
            sim.open_edge(&mut psi, &dv.jphi, &m.rho);
            if !self.toroidal {
                sim.relax(&mut psi, &vac);
            }
            let dv = sim.derived(&psi, &bphi, &m);
            let felt = Derived {
                b: dv.b.clone().map(|c| smooth(g, &c, self.smooth)),
                e: dv.e.clone().map(|c| smooth(g, &c, self.smooth)),
                jphi: vec![],
            };
            // rise in magnetic energy, and force of the plasma's field on the rings
            let mut wf = 0.0;
            for k in 1..g.nz {
                for i in 1..g.nr {
                    let x = g.at(i, k);
                    if sim.solid[x] {
                        continue;
                    }
                    let (b0r, b0z) = (sim.bc[0][x], sim.bc[2][x] + self.uniform_t);
                    let b2 = dv.b[0][x].powi(2) + dv.b[1][x].powi(2) + dv.b[2][x].powi(2);
                    wf += (b2 - b0r * b0r - b0z * b0z) / (2.0 * MU0) * g.volume(i);
                }
            }
            rep.peak_field_energy = rep.peak_field_energy.max(wf / ke0);
            let ke_now: f64 = ions.iter().filter(|p| p.alive).map(ke).sum::<f64>()
                + gone.iter().map(|g| g.2).sum::<f64>();
            rep.worst_energy_error = rep.worst_energy_error.max(((ke_now + wf) / ke0 - 1.0).abs());
            rep.ring_impulse_z += sim.ring_force(&dv) * dt_p;
            // push
            let rings = &self.rings;
            let wire = self.wire_r;
            let ke_before: f64 = ions.iter().filter(|p| p.alive).map(ke).sum();
            let (_, jie) = sim.work_felt(&felt.e, &m);
            rep.ledger[1] += jie * dt_p;
            let results: Vec<Option<(f64, f64, f64, u8)>> = ions
                .par_iter_mut()
                .map(|p| {
                    if !p.alive {
                        return None;
                    }
                    let e = sim.interp(&felt.e, p.r, p.z);
                    let b = sim.interp(&felt.b, p.r, p.z);
                    push(p, e, b, dt_p);
                    for ring in rings {
                        if (p.r - ring.r).hypot(p.z - ring.z) < wire {
                            p.alive = false;
                            return Some((p.w * p.m, p.w * p.m * p.v[2], ke(p), 2));
                        }
                    }
                    if g.locate(p.r, p.z).is_none() {
                        p.alive = false;
                        return Some((p.w * p.m, p.w * p.m * p.v[2], ke(p), u8::from(p.v[2] > 0.0)));
                    }
                    None
                })
                .collect();
            let ke_after: f64 = ions.iter().filter(|p| p.alive).map(ke).sum::<f64>()
                + results.iter().flatten().map(|r| r.2).sum::<f64>();
            rep.ledger[0] += ke_after - ke_before;
            for (mass, pz, e, kind) in results.into_iter().flatten() {
                if kind == 2 {
                    rep.conductor_energy += e;
                }
                gone.push((mass, pz, e, kind == 1));
            }
            let slab: Vec<f64> = ions.iter().filter(|p| p.alive && p.z.abs() < 20.0).map(|p| p.r).collect();
            if slab.len() > 20 {
                let mut s = slab;
                s.sort_by(f64::total_cmp);
                rep.max_equator_r = rep.max_equator_r.max(s[(0.9 * s.len() as f64) as usize]);
            }
            rep.t += dt_p;
            rep.particle_steps += 1;
            if ions.iter().all(|p| !p.alive) {
                break;
            }
        }
        let live = ions.iter().filter(|p| p.alive);
        let mut mass_total = 0.0;
        let (mut mass_fwd, mut e_fwd, mut e_total, mut pz) = (0.0, 0.0, 0.0, 0.0);
        for p in live {
            let (mm, e) = (p.w * p.m, ke(p));
            mass_total += mm;
            e_total += e;
            pz += mm * p.v[2];
            if p.v[2] > 0.0 {
                mass_fwd += mm;
                e_fwd += e;
            }
        }
        for &(mm, z, e, fwd) in &gone {
            mass_total += mm;
            e_total += e;
            pz += z;
            if fwd {
                mass_fwd += mm;
                e_fwd += e;
            }
        }
        rep.plasma_pz = pz - pz0;
        rep.collimation = -rep.plasma_pz / p0;
        rep.forward_mass = mass_fwd / mass_total;
        rep.forward_energy = e_fwd / e_total;
        rep.conductor_energy /= ke0;
        rep.energy_left = e_total / ke0;
        rep.momentum_error = (rep.plasma_pz + rep.ring_impulse_z).abs() / p0;
        rep.secs = start.elapsed().as_secs_f64();
        Ok(rep)
    }
}

fn isotropic(rng: &mut SmallRng) -> [f64; 3] {
    let u: f64 = rng.random_range(-1.0..1.0);
    let phi = rng.random_range(0.0..2.0 * PI);
    let s = (1.0 - u * u).sqrt();
    [s * phi.cos(), s * phi.sin(), u]
}

/// An ion at distance rad along dir from the origin, moving outward at
/// speed, expressed in its own (r, phi, z) frame.
fn blast_ion(dir: [f64; 3], rad: f64, speed: f64, w: f64, m: f64, q: f64) -> Ion {
    let (x, y, z) = (dir[0] * rad, dir[1] * rad, dir[2] * rad);
    let r = x.hypot(y);
    let (ca, sa) = if r > 0.0 { (x / r, y / r) } else { (1.0, 0.0) };
    let v = [dir[0] * speed, dir[1] * speed, dir[2] * speed];
    Ion { r, z, v: [v[0] * ca + v[1] * sa, -v[0] * sa + v[1] * ca, v[2]], w, m, q, alive: true }
}

fn map(g: &Grid, m: &Moments) -> Map {
    let step = 4;
    let (nr, nz) = (g.nr / step, g.nz / step);
    let mut n = vec![0.0; (nr + 1) * (nz + 1)];
    for k in 0..=nz {
        for i in 0..=nr {
            n[k * (nr + 1) + i] = m.rho[g.at(i * step, k * step)] / QE;
        }
    }
    Map { t: 0.0, bz_center: 0.0, dr: g.dx * step as f64, z0: g.z0, nr, nz, n }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn quiet_sim(spec: &HybridSpec) -> Sim<'_> {
        let g = Grid {
            nr: (spec.r_max / spec.dx).round() as usize,
            nz: ((spec.z_max - spec.z_min) / spec.dx).round() as usize,
            dx: spec.dx,
            z0: spec.z_min,
        };
        let n = g.len();
        Sim::new(spec, g, [vec![0.0; n], vec![0.0; n], vec![0.0; n]], vec![false; n])
    }

    fn field_energy(sim: &Sim, psi: &[f64], bphi: &[f64]) -> f64 {
        let m = Moments { rho: vec![0.0; psi.len()], ji: [vec![0.0; psi.len()], vec![0.0; psi.len()], vec![0.0; psi.len()]] };
        let dv = sim.derived(psi, bphi, &m);
        let g = &sim.g;
        let mut w = 0.0;
        for k in 1..g.nz {
            for i in 1..g.nr {
                let x = g.at(i, k);
                let bz = dv.b[2][x] - sim.spec.uniform_t;
                w += (dv.b[0][x].powi(2) + dv.b[1][x].powi(2) + bz * bz) / (2.0 * MU0) * g.volume(i);
            }
        }
        w
    }

    fn quick(kw: HybridSpec) -> HybridReport {
        HybridSpec { r_max: 200.0, z_min: -200.0, z_max: 200.0, particles: 8000, t_end: 4e-5, mirror: true, snapshots: 1, ..kw }
            .evaluate()
            .unwrap()
    }

    /// With no field and cold electrons the ions coast: energy is kept
    /// exactly and the mirrored blast stays symmetric.
    #[test]
    fn free_blast_coasts() {
        let r = quick(HybridSpec { rings: vec![], te_ev: 0.0, ..HybridSpec::default() });
        assert!((r.energy_left - 1.0).abs() < 1e-9, "energy {}", r.energy_left);
        assert!(r.collimation.abs() < 1e-9, "collimation {}", r.collimation);
    }

    /// In a uniform field, in a superconducting box (an open edge lets field
    /// energy leave), the blast pushes the field aside: ions slow, the field's
    /// energy rises by what they lose, and both agree with the ledger of
    /// J_i . E and J . E.
    #[test]
    fn blast_in_uniform_field_keeps_its_energy() {
        let r = quick(HybridSpec { rings: vec![], uniform_t: 0.1, te_ev: 0.0, smooth: 0, open: false, ..HybridSpec::default() });
        assert!(r.energy_left < 0.9, "the field did not slow the blast: {}", r.energy_left);
        assert!(r.worst_energy_error < 0.03, "energy error {}", r.worst_energy_error);
        let [dke, jie, dw, je] = r.ledger;
        assert!((dke - jie).abs() < 0.03 * dke.abs(), "ions: {dke} vs {jie}");
        assert!((dw - je).abs() < 0.03 * dw.abs(), "field: {dw} vs {je}");
        assert!(r.collimation.abs() < 1e-9, "mirrored blast went {}", r.collimation);
    }

    /// In vacuum, with no resistivity, whistler waves on a bump of flux
    /// carry energy about but do not create it.
    #[test]
    fn vacuum_field_does_not_gain_energy() {
        let spec = HybridSpec { rings: vec![], uniform_t: 0.1, r_max: 200.0, z_min: -200.0, z_max: 200.0, eta_vac: 0.0, toroidal: true, ..HybridSpec::default() };
        let sim = quiet_sim(&spec);
        let g = &sim.g;
        let mut psi = vec![0.0; g.len()];
        let bphi = vec![0.0; g.len()];
        for k in 0..=g.nz {
            for i in 0..=g.nr {
                let (r, z) = (g.r(i), g.z(k));
                psi[g.at(i, k)] = 0.01 * 0.5 * r * r * (-(r * r + z * z) / (40.0f64 * 40.0)).exp();
            }
        }
        let mut bphi = bphi;
        let m = Moments { rho: vec![0.0; g.len()], ji: [vec![0.0; g.len()], vec![0.0; g.len()], vec![0.0; g.len()]] };
        let w0 = field_energy(&sim, &psi, &bphi);
        let k2 = 2.0 * (PI / spec.dx).powi(2);
        let dt = spec.whistler_frac * 2.0 / (k2 * spec.b_ref / (MU0 * QE * spec.n_floor));
        let vac = vec![false; g.len()];
        for _ in 0..400 {
            sim.advance(&mut psi, &mut bphi, &m, &vac, dt);
        }
        let w1 = field_energy(&sim, &psi, &bphi);
        assert!(w1 < 1.05 * w0, "field energy grew from {w0} to {w1}");
    }
}
