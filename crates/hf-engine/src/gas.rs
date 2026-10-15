//! Ideal-gas mixtures and chemical equilibrium.
//!
//! Equilibrium at fixed temperature and pressure minimises Gibbs energy
//! under element conservation by the iteration of CEA (Gordon & McBride,
//! NASA RP-1311, eqs. 2.24-2.26, with its step control, eqs. 3.1-3.2).
//! Fixed enthalpy or entropy is a root find on temperature around it.

use crate::thermo::{el, DUST_H, H_MW, NE, NH3_MW, NS, P0, R, THERMO};
use anyhow::{bail, Result};

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

    /// Liquid ammonia from the tank, plus `heat` J/kg picked up on the way.
    pub fn ammonia(heat: f64) -> Self {
        let mut b = [0.; NE];
        b[el::H] = 3. / NH3_MW;
        b[el::N] = 1. / NH3_MW;
        Self { b, h: THERMO.nh3_liquid_h + heat }
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

/// An ideal-gas mixture: mol/kg of each species, temperature, pressure.
#[derive(Clone, Copy, Debug, Default)]
pub struct Gas {
    pub n: [f64; NS],
    pub t: f64,
    pub p: f64,
}

impl Gas {
    pub fn n_total(&self) -> f64 {
        self.n.iter().sum()
    }

    /// kg/mol
    pub fn mw(&self) -> f64 {
        1. / self.n_total()
    }

    pub fn x(&self, j: usize) -> f64 {
        self.n[j] / self.n_total()
    }

    /// J/kg
    pub fn h(&self) -> f64 {
        let th = &*THERMO;
        (0..NS).map(|j| self.n[j] * th.gas[j].h_rt(self.t)).sum::<f64>() * R * self.t
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
        s * R
    }

    /// Frozen heat capacity, J/kg K.
    pub fn cp(&self) -> f64 {
        let th = &*THERMO;
        (0..NS).map(|j| self.n[j] * th.gas[j].cp_r(self.t)).sum::<f64>() * R
    }

    pub fn gamma(&self) -> f64 {
        let cp = self.cp();
        cp / (cp - self.n_total() * R)
    }

    pub fn rho(&self) -> f64 {
        self.p * self.mw() / (R * self.t)
    }

    pub fn feed(&self) -> Feed {
        let th = &*THERMO;
        let mut b = [0.; NE];
        for j in 0..NS {
            for e in 0..NE {
                b[e] += th.gas[j].atoms[e] * self.n[j];
            }
        }
        Feed { b, h: self.h() }
    }

    /// The same composition at another temperature and pressure.
    pub fn at(self, t: f64, p: f64) -> Self {
        Self { t, p, ..self }
    }

    /// Frozen composition at enthalpy `h` and pressure `p`.
    pub fn frozen_h(self, h: f64, p: f64) -> Result<Self> {
        let t = brent(|t| Ok(self.at(t, p).h() - h), 100., 20000., 1e-6)?;
        Ok(self.at(t, p))
    }

    /// Frozen composition at entropy `s` and pressure `p`.
    pub fn frozen_s(self, s: f64, p: f64) -> Result<Self> {
        let t = brent(|t| Ok(self.at(t, p).s() - s), 100., 20000., 1e-6)?;
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
pub fn eq_tp(b: &[f64; NE], t: f64, p: f64, start: Option<&Gas>) -> Result<Gas> {
    let th = &*THERMO;
    let present: [bool; NE] = std::array::from_fn(|e| b[e] > 1e-12);
    let mut active = [false; NS];
    let mut n_active = 0;
    for j in 0..NS {
        active[j] = (0..NE).all(|e| th.gas[j].atoms[e] == 0. || present[e]);
        n_active += active[j] as usize;
    }
    let elems: [usize; NE] = {
        let mut v = [0; NE];
        let mut k = 0;
        for e in 0..NE {
            if present[e] {
                v[k] = e;
                k += 1;
            }
        }
        v
    };
    let l = present.iter().filter(|p| **p).count();
    let g0: [f64; NS] = std::array::from_fn(|j| th.gas[j].g_rt(t));
    let lp = (p / P0).ln();
    let b_scale = b.iter().cloned().fold(0., f64::max);
    let mut ln_nj = [f64::NEG_INFINITY; NS];
    let mut ln_n;
    match start {
        Some(s) if (0..NS).all(|j| !active[j] || s.n[j] > 0.) => {
            for j in 0..NS {
                if active[j] {
                    ln_nj[j] = s.n[j].ln();
                }
            }
            ln_n = s.n_total().ln();
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
        let mut m = [[0.; NE + 2]; NE + 1];
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
                m[r][l] += akj * nj[j];
                rhs += akj * nj[j] * (mu[j] - 1.);
            }
            m[r][l + 1] = rhs;
        }
        let sum_n: f64 = nj.iter().sum();
        let mut rhs = n - sum_n;
        for j in 0..NS {
            if !active[j] {
                continue;
            }
            for (c, &i) in elems[..l].iter().enumerate() {
                m[l][c] += th.gas[j].atoms[i] * nj[j];
            }
            rhs += nj[j] * mu[j];
        }
        m[l][l] = sum_n - n;
        m[l][l + 1] = rhs;
        let x = solve(&mut m, l + 1)?;
        let dln_n = x[l];
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
        ln_n += lam * dln_n;
        if converged {
            let gas = Gas {
                n: std::array::from_fn(|j| if active[j] { ln_nj[j].exp() } else { 0. }),
                t,
                p,
            };
            let fb = gas.feed().b;
            if (0..NE).all(|e| (fb[e] - b[e]).abs() <= 1e-6 * b_scale) {
                return Ok(gas);
            }
        }
    }
    bail!("equilibrium did not converge at {t:.0} K, {p:.3e} Pa")
}

/// Gaussian elimination with partial pivoting on an n x (n+1) augmented
/// matrix.
fn solve(m: &mut [[f64; NE + 2]; NE + 1], n: usize) -> Result<[f64; NE + 1]> {
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
    let mut x = [0.; NE + 1];
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
}
