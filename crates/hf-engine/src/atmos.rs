//! Atmospheres: the ISA temperature and pressure profile, with Earth air
//! or an oxygen-free nitrogen air of the same profile.

use crate::{
    gas::Gas,
    thermo::{sp, NS},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Air {
    #[default]
    Earth,
    /// Nitrogen with Earth's argon and no oxygen.
    Nitrogen,
}

impl Air {
    /// The gas at `t`, `p`.
    pub fn gas(self, t: f64, p: f64) -> Gas {
        let mut x = [0.; NS];
        match self {
            Air::Earth => {
                x[sp::N2] = 0.78084;
                x[sp::O2] = 0.20946;
                x[sp::AR] = 0.00934;
            }
            Air::Nitrogen => {
                x[sp::N2] = 0.99066;
                x[sp::AR] = 0.00934;
            }
        }
        let th = &*crate::thermo::THERMO;
        let mw: f64 = (0..NS).map(|j| x[j] * th.gas[j].mw).sum();
        Gas { n: std::array::from_fn(|j| x[j] / mw), t, p, ..Default::default() }
    }
}

/// ISA temperature (K) and pressure (Pa) at geometric-ish altitude `z` m.
pub fn isa(z: f64) -> (f64, f64) {
    const G: f64 = 9.80665;
    const RA: f64 = 287.053;
    // (base altitude, base temperature, lapse K/m)
    const LAYERS: [(f64, f64, f64); 4] = [
        (0., 288.15, -0.0065),
        (11000., 216.65, 0.),
        (20000., 216.65, 0.001),
        (32000., 228.65, 0.0028),
    ];
    let mut p = 101325.;
    for (k, &(z0, t0, lapse)) in LAYERS.iter().enumerate() {
        let top = LAYERS.get(k + 1).map_or(f64::INFINITY, |l| l.0);
        let dz = z.min(top) - z0;
        let t = t0 + lapse * dz;
        p *= if lapse == 0. {
            (-G * dz / (RA * t0)).exp()
        } else {
            (t / t0).powf(-G / (RA * lapse))
        };
        if z <= top {
            return (t, p);
        }
    }
    unreachable!()
}

/// The altitude at which flight at Mach `m` has dynamic pressure `q` Pa.
pub fn altitude_for_q(m: f64, q: f64) -> f64 {
    let p_target = q / (0.7 * m * m);
    crate::gas::brent(|z| Ok(isa(z).1 - p_target), 0., 47000., 1e-9).unwrap_or(47000.)
}

#[cfg(test)]
mod tests {
    #[test]
    fn isa_points() {
        let (t, p) = super::isa(11000.);
        assert!((t - 216.65).abs() < 0.01 && (p - 22632.).abs() < 5.);
        let (t, p) = super::isa(20000.);
        assert!((t - 216.65).abs() < 0.01 && (p - 5474.9).abs() < 3.);
    }
}
