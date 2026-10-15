//! First-order heat load on the turbopump's blades, and whether its
//! diluent can carry it.
//!
//! Transport properties are Chapman-Enskog with Lennard-Jones parameters
//! (Neufeld's collision integral), Eucken conductivities, Wilke mixing,
//! over the major species H, H2, N2, NH3, CH4, C2H2, H2O, OH and O. Heat flux follows Reynolds'
//! analogy on enthalpy, St = 1.5 * 0.037 Re_c^-0.2 Pr^-2/3: a turbulent
//! flat plate raised half again for leading edges and free-stream
//! turbulence. Driving it with equilibrium enthalpies counts the hydrogen
//! that recombines on a cooled, catalytic surface.
//!
//! Geometry: a constant mean radius set by the first stage's blades being
//! a tenth of it; blades as long as their chord; solidity 1.3; flow
//! coefficient 0.6 and loading 2. A row's blade surface is then 2 x
//! solidity x its annulus area whatever the chord; hub and casing add
//! 2 x 2 pi r x chord. Each stage is a stator and a rotor.

use crate::{
    gas::{eq_tp, Diluent, Gas},
    parts::{Stage, TurbopumpPerKg, MAX_STAGES},
    thermo::{sp, R, THERMO},
};
use anyhow::Result;
use std::f64::consts::PI;

/// (species, sigma Angstrom, epsilon/k K)
const LJ: [(usize, f64, f64); 9] = [
    (sp::H, 2.708, 37.0),
    (sp::H2, 2.827, 59.7),
    (sp::N2, 3.798, 71.4),
    (sp::NH3, 2.900, 558.3),
    (sp::CH4, 3.758, 148.6),
    (sp::C2H2, 4.033, 231.8),
    (sp::H2O, 2.641, 809.1),
    (sp::OH, 3.147, 79.8),
    (sp::O, 3.050, 106.7),
];

/// Viscosity (Pa s) and conductivity (W/m K) of the major species of `g`
/// at temperature `t`.
fn transport(g: &Gas, t: f64) -> (f64, f64) {
    let th = &*THERMO;
    let total: f64 = LJ.iter().map(|(j, ..)| g.n[*j]).sum();
    let mut mu = [0.; LJ.len()];
    let mut k = [0.; LJ.len()];
    let mut x = [0.; LJ.len()];
    let mut mw = [0.; LJ.len()];
    for (i, &(j, sigma, eps)) in LJ.iter().enumerate() {
        let ts = t / eps;
        let omega = 1.16145 / ts.powf(0.14874)
            + 0.52487 / (0.77320 * ts).exp()
            + 2.16178 / (2.43787 * ts).exp();
        mw[i] = th.gas[j].mw;
        mu[i] = 2.6693e-6 * (mw[i] * 1e3 * t).sqrt() / (sigma * sigma * omega);
        k[i] = mu[i] / mw[i] * (th.gas[j].cp_r(t) * R + 1.25 * R);
        x[i] = g.n[j] / total;
    }
    let (mut mu_mix, mut k_mix) = (0., 0.);
    for i in 0..LJ.len() {
        let phi: f64 = (0..LJ.len())
            .map(|j| {
                let a = 1. + (mu[i] / mu[j]).sqrt() * (mw[j] / mw[i]).powf(0.25);
                x[j] * a * a / (8. * (1. + mw[i] / mw[j])).sqrt()
            })
            .sum();
        mu_mix += x[i] * mu[i] / phi;
        k_mix += x[i] * k[i] / phi;
    }
    (mu_mix, k_mix)
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StageHeat {
    pub t: f64,
    pub p: f64,
    /// m
    pub blade: f64,
    /// W/m2
    pub flux: f64,
    /// W, stator and rotor together
    pub heat: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct BladeHeat {
    pub stages: [StageHeat; MAX_STAGES],
    pub n_stages: usize,
    pub mean_radius: f64,
    pub rpm: f64,
    /// W, the whole turbine.
    pub heat: f64,
    /// W the turbopump's diluent absorbs as coolant.
    pub capacity: f64,
    /// W of shaft work, to compare.
    pub work: f64,
}

/// The heat load on a turbopump moving `m_gas` kg/s out of its reverter,
/// with blade surfaces held at `t_wall`, its diluent absorbing `coolant_dh`
/// J/kg as coolant.
/// A `catalytic` surface recombines the hydrogen atoms that reach it and
/// takes their energy; a non-catalytic one sees the gas frozen.
pub fn blade_heat(
    k: &TurbopumpPerKg,
    m_gas: f64,
    t_wall: f64,
    coolant_dh: f64,
    catalytic: bool,
) -> Result<BladeHeat> {
    const LOADING: f64 = 2.;
    const FLOW: f64 = 0.6;
    const SOLIDITY: f64 = 1.3;
    let first = &k.stages[0];
    let u = (first.work / first.m / LOADING).sqrt();
    let vx = FLOW * u;
    let annulus = |s: &Stage| s.m * m_gas / (s.inlet.rho() * vx);
    let r = (annulus(first) / (2. * PI * 0.1)).sqrt();
    let mut out = [StageHeat::default(); MAX_STAGES];
    let mut total = 0.;
    for (i, s) in k.stages[..k.n_stages].iter().enumerate() {
        let g = s.inlet;
        let a = annulus(s);
        let blade = a / (2. * PI * r);
        let v = u * (1. + FLOW * FLOW).sqrt();
        let t_ref = (g.t + t_wall) / 2.;
        let (mu, cond) = transport(&g, t_ref);
        let cp = g.at(t_ref, g.p).cp();
        let pr = mu * cp / cond;
        let re = g.rho() * v * blade / mu;
        let st = 1.5 * 0.037 * re.powf(-0.2) * pr.powf(-2. / 3.);
        let h_aw = g.h() - (1. - pr.powf(1. / 3.)) * v * v / 2.;
        let h_w = if catalytic {
            eq_tp(&g.feed().b, t_wall, g.p, Some(&g))?.h()
        } else {
            g.at(t_wall, g.p).h()
        };
        let flux = (st * g.rho() * v * (h_aw - h_w)).max(0.);
        let area = 2. * SOLIDITY * a + 2. * 2. * PI * r * blade;
        let heat = 2. * flux * area;
        total += heat;
        out[i] = StageHeat { t: g.t, p: g.p, blade, flux, heat };
    }
    let capacity = k.diluent * m_gas * coolant_dh;
    let work: f64 = k.stages[..k.n_stages].iter().map(|s| s.work).sum::<f64>() * m_gas;
    Ok(BladeHeat {
        stages: out,
        n_stages: k.n_stages,
        mean_radius: r,
        rpm: u / r * 60. / (2. * PI),
        heat: total,
        capacity,
        work,
    })
}

/// J/kg a diluent takes as coolant from its tank to `t` at `p`, cracked
/// or dissociated to equilibrium on the way.
pub fn coolant_heat(d: Diluent, p: f64, t: f64) -> Result<f64> {
    let liquid = d.feed(0.);
    Ok(eq_tp(&liquid.b, t, p, None)?.h() - liquid.h)
}
