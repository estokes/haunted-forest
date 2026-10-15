//! Ideal-gas thermodynamics from the NASA Glenn nine-coefficient fits
//! (McBride, Zehe & Gordon, NASA TP-2002-211556), as shipped with CEA in
//! `thermo.inp`. `data/thermo.inp` holds the subset of species used here.
//!
//! Enthalpies are absolute in the CEA sense: zero for the reference
//! elements at 298.15 K, so heats of reaction come out of differences.
//! Graphite is the one condensed product, for the soot that cracked
//! methane can drop.

use anyhow::{bail, Context, Result};
use std::sync::LazyLock;

pub const R: f64 = 8.314462618;
/// Reference pressure of the standard-state data.
pub const P0: f64 = 1e5;

pub const NE: usize = 5;
pub const ELEMENTS: [&str; NE] = ["H", "N", "O", "AR", "C"];
pub mod el {
    pub const H: usize = 0;
    pub const N: usize = 1;
    pub const O: usize = 2;
    pub const AR: usize = 3;
    pub const C: usize = 4;
}

pub const NS: usize = 42;
pub const GASES: [&str; NS] = [
    "H",
    "H2",
    "N",
    "N2",
    "NH",
    "NH2",
    "NH3",
    "O",
    "O2",
    "OH",
    "H2O",
    "HO2",
    "H2O2",
    "NO",
    "NO2",
    "N2O",
    "HNO",
    "Ar",
    "C",
    "CO",
    "CO2",
    "CH4",
    "HCN",
    "CN",
    "C2H2,acetylene",
    "C2",
    "C3",
    "CH",
    "CH2",
    "CH3",
    "C2H",
    "C2H4",
    "C4",
    "C5",
    "C4H2,butadiyne",
    "C6H2",
    "C2N2",
    "HCCN",
    "HNC",
    "CCN",
    "CNC",
    "NCN",
];

pub const NC: usize = 1;
pub const CONDENSED: [&str; NC] = ["C(gr)"];
pub mod sp {
    pub const H: usize = 0;
    pub const H2: usize = 1;
    pub const N: usize = 2;
    pub const N2: usize = 3;
    pub const NH3: usize = 6;
    pub const O: usize = 7;
    pub const O2: usize = 8;
    pub const OH: usize = 9;
    pub const H2O: usize = 10;
    pub const NO: usize = 13;
    pub const AR: usize = 17;
    pub const CO: usize = 19;
    pub const CO2: usize = 20;
    pub const CH4: usize = 21;
    pub const C2H2: usize = 24;
}

#[derive(Clone, Copy, Default)]
struct Interval {
    t_lo: f64,
    t_hi: f64,
    a: [f64; 7],
    b: [f64; 2],
}

#[derive(Clone, Copy, Default)]
pub struct Species {
    /// kg/mol
    pub mw: f64,
    /// Atoms of each element in one molecule.
    pub atoms: [f64; NE],
    intervals: [Interval; 3],
    n_intervals: usize,
}

impl Species {
    fn interval(&self, t: f64) -> &Interval {
        let iv = &self.intervals[..self.n_intervals];
        iv.iter().find(|i| t <= i.t_hi).unwrap_or(&iv[iv.len() - 1])
    }

    pub fn cp_r(&self, t: f64) -> f64 {
        let a = &self.interval(t).a;
        a[0] / (t * t) + a[1] / t + a[2] + t * (a[3] + t * (a[4] + t * (a[5] + t * a[6])))
    }

    pub fn h_rt(&self, t: f64) -> f64 {
        let Interval { a, b, .. } = self.interval(t);
        -a[0] / (t * t)
            + a[1] * t.ln() / t
            + a[2]
            + t * (a[3] / 2. + t * (a[4] / 3. + t * (a[5] / 4. + t * a[6] / 5.)))
            + b[0] / t
    }

    pub fn s_r(&self, t: f64) -> f64 {
        let Interval { a, b, .. } = self.interval(t);
        -a[0] / (2. * t * t) - a[1] / t
            + a[2] * t.ln()
            + t * (a[3] + t * (a[4] / 2. + t * (a[5] / 3. + t * a[6] / 4.)))
            + b[1]
    }

    /// Standard-state Gibbs energy over RT.
    pub fn g_rt(&self, t: f64) -> f64 {
        self.h_rt(t) - self.s_r(t)
    }
}

pub struct Thermo {
    pub gas: [Species; NS],
    pub condensed: [Species; NC],
    /// Liquid ammonia at its 1 atm boiling point, 239.72 K: J/kg.
    pub nh3_liquid_h: f64,
    /// Liquid methane at its 1 atm boiling point, 111.64 K: J/kg.
    pub ch4_liquid_h: f64,
}

/// Dust reverting to molecular hydrogen at 298 K releases 216 MJ/kg.
pub const DUST_H: f64 = 216e6;
pub const H_MW: f64 = 1.00794e-3;
pub const NH3_MW: f64 = 17.03052e-3;
pub const CH4_MW: f64 = 16.04246e-3;

pub static THERMO: LazyLock<Thermo> =
    LazyLock::new(|| parse(include_str!("../data/thermo.inp")).expect("data/thermo.inp"));

fn num(s: &str) -> Result<f64> {
    let s = s.trim().replace('D', "E");
    if s.is_empty() {
        Ok(0.)
    } else {
        s.parse().with_context(|| format!("number {s:?}"))
    }
}

fn parse(text: &str) -> Result<Thermo> {
    let lines: Vec<&str> = text.lines().collect();
    let mut gas = [Species::default(); NS];
    let mut found = [false; NS];
    let mut condensed = [Species::default(); NC];
    let mut found_c = [false; NC];
    let (mut nh3_liquid_h, mut ch4_liquid_h) = (None, None);
    let mut i = 0;
    while i < lines.len() {
        let name = lines[i].split_whitespace().next().context("species name")?;
        let hdr = lines[i + 1];
        let n_int: usize = hdr[..2].trim().parse()?;
        let mut atoms = [0.; NE];
        for k in 0..5 {
            let sym = hdr[10 + 8 * k..12 + 8 * k].trim().to_uppercase();
            let count = num(&hdr[12 + 8 * k..18 + 8 * k])?;
            if count == 0. {
                continue;
            }
            let e =
                ELEMENTS.iter().position(|e| *e == sym).with_context(|| sym.clone())?;
            atoms[e] = count;
        }
        let phase = num(&hdr[50..52])?;
        let mw = num(&hdr[52..65])? * 1e-3;
        let hf = num(&hdr[65..80])?;
        if n_int == 0 {
            match name {
                "NH3(L)" => nh3_liquid_h = Some(hf / mw),
                "CH4(L)" => ch4_liquid_h = Some(hf / mw),
                _ => (),
            }
            i += 3;
            continue;
        }
        let mut sp = Species { mw, atoms, n_intervals: n_int, ..Default::default() };
        for k in 0..n_int {
            let l0 = lines[i + 2 + 3 * k];
            let (l1, l2) = (lines[i + 3 + 3 * k], lines[i + 4 + 3 * k]);
            let iv = &mut sp.intervals[k];
            iv.t_lo = num(&l0[..11])?;
            iv.t_hi = num(&l0[11..22])?;
            for c in 0..5 {
                iv.a[c] = num(&l1[16 * c..16 * c + 16])?;
            }
            iv.a[5] = num(&l2[..16])?;
            iv.a[6] = num(&l2[16..32])?;
            iv.b[0] = num(&l2[48..64])?;
            iv.b[1] = num(&l2[64..80])?;
        }
        if phase != 0. {
            if let Some(j) = CONDENSED.iter().position(|g| *g == name) {
                condensed[j] = sp;
                found_c[j] = true;
            }
        } else if let Some(j) = GASES.iter().position(|g| *g == name) {
            gas[j] = sp;
            found[j] = true;
        }
        i += 2 + 3 * n_int;
    }
    if let Some(j) = found.iter().position(|f| !f) {
        bail!("species {} missing from thermo data", GASES[j]);
    }
    if let Some(j) = found_c.iter().position(|f| !f) {
        bail!("species {} missing from thermo data", CONDENSED[j]);
    }
    Ok(Thermo {
        gas,
        condensed,
        nh3_liquid_h: nh3_liquid_h.context("NH3(L) missing")?,
        ch4_liquid_h: ch4_liquid_h.context("CH4(L) missing")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn janaf_spot_values() {
        let th = &*THERMO;
        let h2 = &th.gas[sp::H2];
        // JANAF: H(2000)-H(298) = 52.95 kJ/mol, S(2000) = 188.42 J/mol K
        assert!((h2.h_rt(2000.) * R * 2000. - 52950.).abs() < 100.);
        assert!((h2.s_r(2000.) * R - 188.42).abs() < 0.2);
        // Hf(H, 298) = 217.999 kJ/mol
        assert!((th.gas[sp::H].h_rt(298.15) * R * 298.15 - 217999.).abs() < 50.);
        // Hf(H2O, g) = -241.826 kJ/mol
        assert!((th.gas[sp::H2O].h_rt(298.15) * R * 298.15 + 241826.).abs() < 50.);
        assert!((th.nh3_liquid_h * NH3_MW + 71555.).abs() < 1.);
        assert!((th.ch4_liquid_h * CH4_MW + 89233.).abs() < 1.);
        // Graphite is the reference state of carbon: zero at 298 K.
        assert!(th.condensed[0].h_rt(298.15).abs() < 1e-3);
    }
}
