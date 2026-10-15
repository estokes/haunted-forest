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

/// US Standard Atmosphere 1976 temperature (K) and pressure (Pa) at
/// geopotential altitude `z` m: its layers to 84.852 km (86 km geometric),
/// then its tabulated density and temperature, by geometric altitude, to
/// 120 km, extended above with a 7 km scale height.
pub fn isa(z: f64) -> (f64, f64) {
    const G: f64 = 9.80665;
    const RA: f64 = 287.053;
    // (base altitude, base temperature, lapse K/m)
    const LAYERS: [(f64, f64, f64); 8] = [
        (0., 288.15, -0.0065),
        (11000., 216.65, 0.),
        (20000., 216.65, 0.001),
        (32000., 228.65, 0.0028),
        (47000., 270.65, 0.),
        (51000., 270.65, -0.0028),
        (71000., 214.65, -0.002),
        (84852., 186.946, 0.),
    ];
    const R0: f64 = 6356766.;
    // (geometric altitude, temperature, density) above 86 km.
    const THERMOSPHERE: [(f64, f64, f64); 8] = [
        (86000., 186.87, 6.958e-6),
        (90000., 186.87, 3.416e-6),
        (95000., 188.42, 1.393e-6),
        (100000., 195.08, 5.604e-7),
        (105000., 208.84, 2.325e-7),
        (110000., 240.0, 9.708e-8),
        (115000., 300.0, 4.289e-8),
        (120000., 360.0, 2.222e-8),
    ];
    if z > LAYERS[LAYERS.len() - 1].0 {
        let z = R0 * z / (R0 - z);
        let last = THERMOSPHERE[THERMOSPHERE.len() - 1];
        let (t, rho) = match THERMOSPHERE.windows(2).find(|w| z <= w[1].0) {
            Some(w) => {
                let f = (z - w[0].0) / (w[1].0 - w[0].0);
                (
                    w[0].1 + f * (w[1].1 - w[0].1),
                    (w[0].2.ln() + f * (w[1].2 / w[0].2).ln()).exp(),
                )
            }
            None => (last.1, last.2 * (-(z - last.0) / 7000.).exp()),
        };
        return (t, rho * RA * t);
    }
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
        // US76 at 50 km geopotential: 75.944 Pa.
        let (_, p) = super::isa(50000.);
        assert!((p / 75.944 - 1.).abs() < 0.001, "{p}");
        // Continuous where the layers hand over to the table.
        let (t0, p0) = super::isa(84851.);
        let (t1, p1) = super::isa(84853.);
        assert!((p1 / p0 - 1.).abs() < 0.01 && (t1 - t0).abs() < 0.2, "{p0} {p1}");
        let (_, p) = super::isa(98500.);
        assert!((p - 0.032).abs() < 0.004, "{p}");
    }
}
