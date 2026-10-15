//! Ideal-gas mixtures, with condensed graphite, and chemical equilibrium.
//!
//! Equilibrium at fixed temperature and pressure minimises Gibbs energy
//! under element conservation by the iteration of CEA (Gordon & McBride,
//! NASA RP-1311, eqs. 2.24-2.26 and 2.36, with its step control, eqs.
//! 3.1-3.2, and its tests for adding and removing condensed species).
//! Fixed enthalpy or entropy is a root find on temperature around it.
//! Condensed carbon is taken to move with the gas at its temperature.

use crate::thermo::{
    el, CH4_MW, DUST_H, H2O_MW, H_MW, NC, NE, NH3_MW, NS, P0, R, THERMO,
};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// What a reacting mixture conserves: element totals (mol/kg) and
/// specific enthalpy (J/kg).
#[derive(Clone, Copy, Debug, Default)]
pub struct Feed {
    pub b: [f64; NE],
    pub h: f64,
}

impl Feed {
    pub fn dust() -> Self {
        let mut b = [0.; NE];
        b[el::H] = 1. / H_MW;
        Self { b, h: DUST_H }
    }

    /// The mass-weighted blend of `(kg, feed)` parts, per kg of the blend.
    pub fn blend(parts: &[(f64, Feed)]) -> Self {
        let m: f64 = parts.iter().map(|(m, _)| m).sum();
        let mut out = Self::default();
        for (mi, f) in parts {
            for e in 0..NE {
                out.b[e] += mi * f.b[e] / m;
            }
            out.h += mi * f.h / m;
        }
        out
    }
}

/// The liquid that carries, cools and dilutes the dust.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Diluent {
    #[default]
    Ammonia,
    Methane,
    /// Ammonia in water, by mass fraction of ammonia: 0 is pure water. Its
    /// enthalpy takes the ammonia as dissolved at infinite dilution,
    /// -80.3 kJ/mol; its density, ~1000 - 330 x kg/m3, fits 0-35%.
    Aqueous(f64),
}

impl std::fmt::Display for Diluent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Diluent::Ammonia => write!(f, "ammonia"),
            Diluent::Methane => write!(f, "methane"),
            Diluent::Aqueous(x) if *x == 0. => write!(f, "water"),
            Diluent::Aqueous(x) => write!(f, "{:.0}% ammonia-water", 100. * x),
        }
    }
}

impl Diluent {
    pub const ALL: [Diluent; 2] = [Diluent::Ammonia, Diluent::Methane];
    pub const WATER: Diluent = Diluent::Aqueous(0.);

    /// The liquid from its tank, plus `heat` J/kg picked up on the way.
    pub fn feed(self, heat: f64) -> Feed {
        let mut b = [0.; NE];
        let h = match self {
            Diluent::Ammonia => {
                b[el::H] = 3. / NH3_MW;
                b[el::N] = 1. / NH3_MW;
                THERMO.nh3_liquid_h
            }
            Diluent::Methane => {
                b[el::H] = 4. / CH4_MW;
                b[el::C] = 1. / CH4_MW;
                THERMO.ch4_liquid_h
            }
            Diluent::Aqueous(x) => {
                b[el::H] = 3. * x / NH3_MW + 2. * (1. - x) / H2O_MW;
                b[el::N] = x / NH3_MW;
                b[el::O] = (1. - x) / H2O_MW;
                x * -80.29e3 / NH3_MW + (1. - x) * -285.83e3 / H2O_MW
            }
        };
        Feed { b, h: h + heat }
    }

    /// kg/m3, as stored.
    pub fn density(self) -> f64 {
        match self {
            Diluent::Ammonia => 682.,
            Diluent::Methane => 422.,
            Diluent::Aqueous(x) => 1000. - 330. * x,
        }
    }

    /// K, the temperature it is stored at: its 1 atm boiling point, or room
    /// temperature for the aqueous liquids.
    pub fn storage(self) -> f64 {
        match self {
            Diluent::Ammonia => 239.7,
            Diluent::Methane => 111.6,
            Diluent::Aqueous(_) => 298.,
        }
    }

    /// K, where it freezes: ammonia-water from its phase diagram, to the
    /// eutectic near 33%.
    pub fn freezing(self) -> f64 {
        match self {
            Diluent::Ammonia => 195.4,
            Diluent::Methane => 90.7,
            Diluent::Aqueous(x) => {
                const PTS: [(f64, f64); 5] =
                    [(0., 273.), (0.1, 259.), (0.2, 238.), (0.28, 196.), (0.33, 173.)];
                let x = x.min(0.33);
                let w = PTS.windows(2).find(|w| x <= w[1].0).unwrap_or(&PTS[3..5]);
                w[0].1 + (x - w[0].0) / (w[1].0 - w[0].0) * (w[1].1 - w[0].1)
            }
        }
    }

    /// J/kg to boil it off a tank.
    pub fn latent(self) -> f64 {
        match self {
            Diluent::Ammonia => 1.37e6,
            Diluent::Methane => 0.51e6,
            Diluent::Aqueous(x) => x * 1.37e6 + (1. - x) * 2.26e6,
        }
    }
}

/// An ideal-gas mixture, with condensed species: mol/kg of each,
/// temperature, pressure.
#[derive(Clone, Copy, Debug)]
pub struct Gas {
    pub n: [f64; NS],
    pub nc: [f64; NC],
    pub t: f64,
    pub p: f64,
}

impl Default for Gas {
    fn default() -> Self {
        Self { n: [0.; NS], nc: [0.; NC], t: 0., p: 0. }
    }
}

impl Gas {
    /// Moles of gas per kg of mixture.
    pub fn n_total(&self) -> f64 {
        self.n.iter().sum()
    }

    /// kg of mixture per mole of gas.
    pub fn mw(&self) -> f64 {
        1. / self.n_total()
    }

    pub fn x(&self, j: usize) -> f64 {
        self.n[j] / self.n_total()
    }

    /// Mass fraction of condensed carbon.
    pub fn soot(&self) -> f64 {
        let th = &*THERMO;
        (0..NC).map(|c| self.nc[c] * th.condensed[c].mw).sum()
    }

    /// J/kg
    pub fn h(&self) -> f64 {
        let th = &*THERMO;
        let gas: f64 = (0..NS).map(|j| self.n[j] * th.gas[j].h_rt(self.t)).sum();
        let cond: f64 = (0..NC).map(|c| self.nc[c] * th.condensed[c].h_rt(self.t)).sum();
        (gas + cond) * R * self.t
    }

    /// J/kg K
    pub fn s(&self) -> f64 {
        let th = &*THERMO;
        let (nt, lp) = (self.n_total(), (self.p / P0).ln());
        let mut s = 0.;
        for j in 0..NS {
            if self.n[j] > 0. {
                s += self.n[j] * (th.gas[j].s_r(self.t) - (self.n[j] / nt).ln() - lp);
            }
        }
        for c in 0..NC {
            s += self.nc[c] * th.condensed[c].s_r(self.t);
        }
        s * R
    }

    /// Frozen heat capacity, J/kg K.
    pub fn cp(&self) -> f64 {
        let th = &*THERMO;
        let gas: f64 = (0..NS).map(|j| self.n[j] * th.gas[j].cp_r(self.t)).sum();
        let cond: f64 = (0..NC).map(|c| self.nc[c] * th.condensed[c].cp_r(self.t)).sum();
        (gas + cond) * R
    }

    pub fn gamma(&self) -> f64 {
        let cp = self.cp();
        cp / (cp - self.n_total() * R)
    }

    /// kg of mixture per m3.
    pub fn rho(&self) -> f64 {
        self.p * self.mw() / (R * self.t)
    }

    pub fn feed(&self) -> Feed {
        let th = &*THERMO;
        let mut b = [0.; NE];
        for e in 0..NE {
            b[e] = (0..NS).map(|j| th.gas[j].atoms[e] * self.n[j]).sum::<f64>()
                + (0..NC).map(|c| th.condensed[c].atoms[e] * self.nc[c]).sum::<f64>();
        }
        Feed { b, h: self.h() }
    }

    /// The same composition at another temperature and pressure.
    pub fn at(self, t: f64, p: f64) -> Self {
        Self { t, p, ..self }
    }

    /// Frozen composition at enthalpy `h` and pressure `p`.
    pub fn frozen_h(self, h: f64, p: f64) -> Result<Self> {
        let t = brent(|t| Ok(self.at(t, p).h() - h), 50., 20000., 1e-6)?;
        Ok(self.at(t, p))
    }

    /// Frozen composition at entropy `s` and pressure `p`.
    pub fn frozen_s(self, s: f64, p: f64) -> Result<Self> {
        let t = brent(|t| Ok(self.at(t, p).s() - s), 50., 20000., 1e-6)?;
        Ok(self.at(t, p))
    }

    /// Equilibrium of this gas's elements at fixed enthalpy and pressure.
    pub fn eq_h(self, h: f64, p: f64) -> Result<Self> {
        let b = self.feed().b;
        eq_hp(&Feed { b, h }, p, Some(&self))
    }

    /// Equilibrium of this gas's elements at fixed entropy and pressure.
    pub fn eq_s(self, s: f64, p: f64) -> Result<Self> {
        let b = self.feed().b;
        let mut warm = self;
        let t = brent(
            |t| {
                warm = eq_tp(&b, t, p, Some(&warm))?;
                Ok(warm.s() - s)
            },
            100.,
            12000.,
            1e-7,
        )?;
        eq_tp(&b, t, p, Some(&warm))
    }
}

/// Equilibrium at fixed enthalpy and pressure.
pub fn eq_hp(feed: &Feed, p: f64, start: Option<&Gas>) -> Result<Gas> {
    let mut warm = start.copied();
    let mut f = |t: f64| -> Result<f64> {
        let g = eq_tp(&feed.b, t, p, warm.as_ref())?;
        warm = Some(g);
        Ok(g.h() - feed.h)
    };
    // Bracket outward from 1,500 K, so the solver never visits
    // temperatures far colder than any burner.
    let (mut lo, mut hi) = (1500., 1500.);
    while f(lo)? > 0. {
        if lo <= 200. {
            bail!("enthalpy {:.3e} J/kg is below the gas at 200 K", feed.h);
        }
        hi = lo;
        lo = (lo * 0.7).max(200.);
    }
    if hi == lo {
        hi = lo * 1.5;
        while f(hi)? < 0. {
            if hi >= 12000. {
                bail!("enthalpy {:.3e} J/kg is above the gas at 12,000 K", feed.h);
            }
            lo = hi;
            hi = (hi * 1.5).min(12000.);
        }
    }
    let t = brent(&mut f, lo, hi, 1e-7)?;
    eq_tp(&feed.b, t, p, warm.as_ref())
}

const LN_TRACE: f64 = -18.420681;

/// Equilibrium at fixed temperature and pressure for element totals `b`.
/// A condensed species joins when its Gibbs energy is below what the
/// element potentials allow it, and leaves when its amount goes negative.
pub fn eq_tp(b: &[f64; NE], t: f64, p: f64, start: Option<&Gas>) -> Result<Gas> {
    let th = &*THERMO;
    let present: [bool; NE] = std::array::from_fn(|e| b[e] > 1e-12);
    let possible: [bool; NC] = std::array::from_fn(|c| {
        (0..NE).all(|e| th.condensed[c].atoms[e] == 0. || present[e])
    });
    let mut included: [bool; NC] =
        std::array::from_fn(|c| possible[c] && start.is_some_and(|s| s.nc[c] > 0.));
    let mut start = start.copied();
    for _ in 0..2 * NC + 2 {
        // A warm start carrying a condensed phase the state no longer
        // holds can fail to converge; start cold without it and retest.
        let (gas, pi) = match newton(b, t, p, start.as_ref(), &included) {
            Ok(r) => r,
            Err(_) => {
                included = [false; NC];
                newton(b, t, p, None, &included)?
            }
        };
        let mut changed = false;
        for c in 0..NC {
            if included[c] && gas.nc[c] <= 0. {
                included[c] = false;
                changed = true;
            } else if !included[c] && possible[c] {
                let allowed: f64 =
                    (0..NE).map(|e| th.condensed[c].atoms[e] * pi[e]).sum();
                if th.condensed[c].g_rt(t) - allowed < -1e-8 {
                    included[c] = true;
                    changed = true;
                }
            }
        }
        if !changed {
            return Ok(gas);
        }
        start = Some(Gas { nc: [0.; NC], ..gas });
    }
    bail!("condensed phases did not settle at {t:.0} K, {p:.3e} Pa")
}

/// The CEA iteration with the condensed species in `included`. Returns
/// the mixture and the element potentials (Lagrange multipliers).
fn newton(
    b: &[f64; NE],
    t: f64,
    p: f64,
    start: Option<&Gas>,
    included: &[bool; NC],
) -> Result<(Gas, [f64; NE])> {
    let th = &*THERMO;
    let present: [bool; NE] = std::array::from_fn(|e| b[e] > 1e-12);
    let mut active = [false; NS];
    let mut n_active = 0;
    for j in 0..NS {
        active[j] = (0..NE).all(|e| th.gas[j].atoms[e] == 0. || present[e]);
        n_active += active[j] as usize;
    }
    let mut elems = [0; NE];
    let mut l = 0;
    for e in 0..NE {
        if present[e] {
            elems[l] = e;
            l += 1;
        }
    }
    let mut conds = [0; NC];
    let mut nci = 0;
    for c in 0..NC {
        if included[c] {
            conds[nci] = c;
            nci += 1;
        }
    }
    let size = l + nci + 1;
    let last = size - 1;
    let g0: [f64; NS] = std::array::from_fn(|j| th.gas[j].g_rt(t));
    let gc: [f64; NC] = std::array::from_fn(|c| th.condensed[c].g_rt(t));
    let lp = (p / P0).ln();
    let b_scale = b.iter().cloned().fold(0., f64::max);
    let mut ln_nj = [f64::NEG_INFINITY; NS];
    let mut ln_n;
    let mut nc = [0.; NC];
    match start {
        Some(s) if (0..NS).all(|j| !active[j] || s.n[j] > 0.) => {
            for j in 0..NS {
                if active[j] {
                    ln_nj[j] = s.n[j].ln();
                }
            }
            ln_n = s.n_total().ln();
            for c in 0..NC {
                if included[c] {
                    nc[c] = s.nc[c].max(0.);
                }
            }
        }
        _ => {
            let n0 = b.iter().sum::<f64>() / 2.;
            for j in 0..NS {
                if active[j] {
                    ln_nj[j] = (n0 / n_active as f64).ln();
                }
            }
            ln_n = n0.ln();
        }
    }
    for _ in 0..500 {
        let nj: [f64; NS] =
            std::array::from_fn(|j| if active[j] { ln_nj[j].exp() } else { 0. });
        let n = ln_n.exp();
        let mu: [f64; NS] = std::array::from_fn(|j| {
            if active[j] {
                g0[j] + ln_nj[j] - ln_n + lp
            } else {
                0.
            }
        });
        let mut m = [[0.; NE + NC + 2]; NE + NC + 1];
        for (r, &k) in elems[..l].iter().enumerate() {
            let mut rhs = b[k];
            for j in 0..NS {
                let akj = th.gas[j].atoms[k];
                if !active[j] || akj == 0. {
                    continue;
                }
                for (c, &i) in elems[..l].iter().enumerate() {
                    m[r][c] += akj * th.gas[j].atoms[i] * nj[j];
                }
                m[r][last] += akj * nj[j];
                rhs += akj * nj[j] * (mu[j] - 1.);
            }
            for (ci, &c) in conds[..nci].iter().enumerate() {
                let akc = th.condensed[c].atoms[k];
                m[r][l + ci] = akc;
                rhs -= akc * nc[c];
            }
            m[r][size] = rhs;
        }
        for (ci, &c) in conds[..nci].iter().enumerate() {
            for (col, &i) in elems[..l].iter().enumerate() {
                m[l + ci][col] = th.condensed[c].atoms[i];
            }
            m[l + ci][size] = gc[c];
        }
        let sum_n: f64 = nj.iter().sum();
        let mut rhs = n - sum_n;
        for j in 0..NS {
            if !active[j] {
                continue;
            }
            for (c, &i) in elems[..l].iter().enumerate() {
                m[last][c] += th.gas[j].atoms[i] * nj[j];
            }
            rhs += nj[j] * mu[j];
        }
        m[last][last] = sum_n - n;
        m[last][size] = rhs;
        let x = solve(&mut m, size)?;
        let dln_n = x[last];
        let mut dln = [0.; NS];
        let mut lam1_den = 5. * dln_n.abs();
        let mut lam2 = f64::INFINITY;
        for j in 0..NS {
            if !active[j] {
                continue;
            }
            let pi: f64 = elems[..l]
                .iter()
                .enumerate()
                .map(|(c, &i)| th.gas[j].atoms[i] * x[c])
                .sum();
            dln[j] = -mu[j] + pi + dln_n;
            let rel = ln_nj[j] - ln_n;
            if rel > LN_TRACE {
                if dln[j] > 0. {
                    lam1_den = lam1_den.max(dln[j]);
                }
            } else if dln[j] >= 0. && (dln[j] - dln_n).abs() > 1e-300 {
                lam2 = lam2.min(((-rel - 9.2103404) / (dln[j] - dln_n)).abs());
            }
        }
        let lam = 1f64.min(if lam1_den > 0. { 2. / lam1_den } else { 1. }).min(lam2);
        let mut converged = n * dln_n.abs() / sum_n <= 0.5e-5;
        for j in 0..NS {
            if active[j] {
                converged &= nj[j] * dln[j].abs() / sum_n <= 0.5e-5;
                ln_nj[j] = (ln_nj[j] + lam * dln[j]).max(ln_n - 200.);
            }
        }
        for (ci, &c) in conds[..nci].iter().enumerate() {
            converged &= x[l + ci].abs() / sum_n <= 0.5e-5;
            nc[c] += lam * x[l + ci];
        }
        ln_n += lam * dln_n;
        if converged {
            let gas = Gas {
                n: std::array::from_fn(|j| if active[j] { ln_nj[j].exp() } else { 0. }),
                nc,
                t,
                p,
            };
            let fb = gas.feed().b;
            if (0..NE).all(|e| (fb[e] - b[e]).abs() <= 1e-6 * b_scale) {
                let mut pi = [0.; NE];
                for (c, &e) in elems[..l].iter().enumerate() {
                    pi[e] = x[c];
                }
                return Ok((gas, pi));
            }
        }
    }
    bail!("equilibrium did not converge at {t:.0} K, {p:.3e} Pa")
}

/// Gaussian elimination with partial pivoting on an n x (n+1) augmented
/// matrix.
fn solve(
    m: &mut [[f64; NE + NC + 2]; NE + NC + 1],
    n: usize,
) -> Result<[f64; NE + NC + 1]> {
    for c in 0..n {
        let piv =
            (c..n).max_by(|a, b| m[*a][c].abs().total_cmp(&m[*b][c].abs())).unwrap();
        if m[piv][c].abs() < 1e-300 {
            bail!("singular equilibrium matrix");
        }
        m.swap(c, piv);
        for r in c + 1..n {
            let f = m[r][c] / m[c][c];
            for k in c..=n {
                m[r][k] -= f * m[c][k];
            }
        }
    }
    let mut x = [0.; NE + NC + 1];
    for r in (0..n).rev() {
        let s: f64 = (r + 1..n).map(|k| m[r][k] * x[k]).sum();
        x[r] = (m[r][n] - s) / m[r][r];
    }
    Ok(x)
}

/// Brent's method for a root of `f` bracketed by `[a, b]`, to relative
/// tolerance `tol` in x.
pub fn brent(
    mut f: impl FnMut(f64) -> Result<f64>,
    a: f64,
    b: f64,
    tol: f64,
) -> Result<f64> {
    let (mut a, mut b) = (a, b);
    let (mut fa, mut fb) = (f(a)?, f(b)?);
    if fa * fb > 0. {
        bail!("root not bracketed in [{a}, {b}]: f = {fa:.4e}, {fb:.4e}");
    }
    if fa.abs() < fb.abs() {
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut fa, &mut fb);
    }
    let (mut c, mut fc, mut d) = (a, fa, a);
    let mut bisected = true;
    for _ in 0..200 {
        if fb == 0. || (b - a).abs() <= tol * b.abs().max(1e-12) {
            return Ok(b);
        }
        let mut s = if fa != fc && fb != fc {
            a * fb * fc / ((fa - fb) * (fa - fc))
                + b * fa * fc / ((fb - fa) * (fb - fc))
                + c * fa * fb / ((fc - fa) * (fc - fb))
        } else {
            b - fb * (b - a) / (fb - fa)
        };
        let lo = (3. * a + b) / 4.;
        let out = !((s > lo.min(b)) && (s < lo.max(b)));
        if out
            || (bisected && (s - b).abs() >= (b - c).abs() / 2.)
            || (!bisected && (s - b).abs() >= (c - d).abs() / 2.)
        {
            s = (a + b) / 2.;
            bisected = true;
        } else {
            bisected = false;
        }
        let fs = f(s)?;
        d = c;
        c = b;
        fc = fb;
        if fa * fs < 0. {
            b = s;
            fb = fs;
        } else {
            a = s;
            fa = fs;
        }
        if fa.abs() < fb.abs() {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut fa, &mut fb);
        }
    }
    bail!("brent did not converge")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::thermo::sp;

    #[test]
    fn hydrogen_air_flame() {
        // Stoichiometric H2/air from 298 K at 1 atm burns at ~2,390 K.
        let mut b = [0.; NE];
        let (o2, n2, ar) = (0.20946, 0.78084, 0.00934);
        let m_air = 2. * 15.9994e-3 * o2 + 2. * 14.0067e-3 * n2 + 39.948e-3 * ar;
        let m = m_air + 2. * o2 * 2.01588e-3;
        b[el::H] = 4. * o2 / m;
        b[el::O] = 2. * o2 / m;
        b[el::N] = 2. * n2 / m;
        b[el::AR] = ar / m;
        let h = -0.; // reactants are reference elements at 298 K
        let g = eq_hp(&Feed { b, h }, 101325., None).unwrap();
        assert!((g.t - 2390.).abs() < 20., "{}", g.t);
        assert!(g.x(sp::H2O) > 0.29 && g.x(sp::H2O) < 0.33);
    }

    #[test]
    fn methane_drops_soot() {
        // Methane at 1,500 K and 1 bar is nearly all graphite and hydrogen.
        let f = Diluent::Methane.feed(0.);
        let g = eq_tp(&f.b, 1500., 1e5, None).unwrap();
        let c_frac = 12.0107e-3 / CH4_MW;
        assert!(g.soot() > 0.95 * c_frac, "soot {}", g.soot());
        assert!(g.x(sp::H2) > 0.95);
    }
}
