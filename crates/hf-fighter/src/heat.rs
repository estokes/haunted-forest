//! Entry heating, engineering methods: Sutton-Graves at a stagnation
//! point; Tauber, Menees and Adelman (J. Aircraft 24, 1987) on a flat
//! surface, laminar and turbulent; a swept cylinder's leading edge as the
//! unswept cylinder's (1/sqrt 2 of a sphere's) times cos^1.2 of the sweep
//! the flow sees; all for Earth air, fully catalytic, so conservative for
//! a non-catalytic skin. Surfaces cool by radiation alone, each at the
//! temperature where it radiates what it takes in.

use std::f64::consts::SQRT_2;

pub const SIGMA: f64 = 5.670374e-8;

/// Cold-wall stagnation heating, W/m2, on a nose of radius `rn` m.
pub fn stagnation(rho: f64, v: f64, rn: f64) -> f64 {
    1.7415e-4 * (rho / rn).sqrt() * v.powi(3)
}

/// Cold-wall heating, W/m2, on a swept leading edge of radius `r` at
/// sweep `sweep` and angle of attack `alpha`.
pub fn leading_edge(rho: f64, v: f64, r: f64, sweep: f64, alpha: f64) -> f64 {
    let effective = (sweep.sin() * alpha.cos()).asin();
    stagnation(rho, v, r) / SQRT_2 * effective.cos().powf(1.2)
}

/// Cold-wall heating, W/m2, a distance `x` m back on a surface inclined at
/// `phi` to the flow; turbulent if `turbulent`. `t_wall` matters only below
/// 3,962 m/s, turbulent.
pub fn flat_plate(
    rho: f64,
    v: f64,
    x: f64,
    phi: f64,
    turbulent: bool,
    t_wall: f64,
) -> f64 {
    let (c, s) = (phi.cos(), phi.sin().max(0.));
    if !turbulent {
        2.53e-5 * c.sqrt() * s * x.powf(-0.5) * rho.sqrt() * v.powf(3.2)
    } else if v > 3962. {
        2.2e-5 * c.powf(2.08) * s.powf(1.6) * x.powf(-0.2) * rho.powf(0.8) * v.powf(3.7)
    } else {
        3.89e-4
            * c.powf(1.78)
            * s.powf(1.6)
            * x.powf(-0.2)
            * (t_wall / 556.).powf(-0.25)
            * rho.powf(0.8)
            * v.powf(3.37)
    }
}

/// The wall temperature, K, at which a surface of emissivity `eps`
/// radiates the heating it takes, the cold-wall heating `q_cold` reduced by
/// the hot wall: (1 - k h_w/h0), k = 1 laminar, 1.11 turbulent.
pub fn radiative_equilibrium(
    q_cold: f64,
    v: f64,
    t_air: f64,
    eps: f64,
    k: f64,
) -> (f64, f64) {
    let h0 = v * v / 2. + 1005. * t_air;
    let net =
        |t: f64| q_cold * (1. - k * 1150. * t / h0).max(0.) - eps * SIGMA * t.powi(4);
    let (mut lo, mut hi) = (t_air.min(200.), 6000.);
    for _ in 0..60 {
        let mid = (lo + hi) / 2.;
        if net(mid) > 0. {
            lo = mid
        } else {
            hi = mid
        }
    }
    (lo, eps * SIGMA * lo.powi(4))
}

/// Insulation between a hot skin and a tank held at its liquid's
/// temperature: conductivity k0 + k1 T, for a high-temperature fibrous or
/// aerogel insulation whose radiative conduction grows with temperature.
#[derive(Clone, Copy, Debug)]
pub struct Insulation {
    pub thickness: f64,
    pub k0: f64,
    pub k1: f64,
    pub density: f64,
    pub heat_capacity: f64,
}

impl Default for Insulation {
    fn default() -> Self {
        Self { thickness: 0.05, k0: 0.02, k1: 4e-5, density: 150., heat_capacity: 1000. }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Soak {
    /// J/m2 into the tank.
    pub heat: f64,
    /// W/m2, peak into the tank.
    pub peak_flux: f64,
    /// s after entry interface.
    pub peak_time: f64,
}

impl Insulation {
    /// Heat through the insulation into a tank at `t_tank`, behind a skin
    /// whose temperature is `skin(t)`, from t = 0 to `t_end` s.
    pub fn soak(&self, skin: impl Fn(f64) -> f64, t_tank: f64, t_end: f64) -> Soak {
        const N: usize = 40;
        let dx = self.thickness / N as f64;
        let k = |t: f64| self.k0 + self.k1 * t;
        let rc = self.density * self.heat_capacity;
        let k_max = k(4000.);
        let dt = 0.4 * dx * dx * rc / k_max;
        let mut temp = [t_tank; N + 1];
        let mut out = Soak::default();
        let mut time = 0.;
        while time < t_end {
            temp[0] = skin(time);
            temp[N] = t_tank;
            let mut next = temp;
            for i in 1..N {
                let kp = k((temp[i] + temp[i + 1]) / 2.);
                let km = k((temp[i] + temp[i - 1]) / 2.);
                next[i] = temp[i]
                    + dt / (rc * dx * dx)
                        * (kp * (temp[i + 1] - temp[i]) - km * (temp[i] - temp[i - 1]));
            }
            temp = next;
            let flux = k((temp[N - 1] + temp[N]) / 2.) * (temp[N - 1] - temp[N]) / dx;
            out.heat += flux * dt;
            if flux > out.peak_flux {
                out.peak_flux = flux;
                out.peak_time = time;
            }
            time += dt;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shuttle_like_points() {
        // Shuttle-class nose, r ~1 m, near peak heating (~70 km, 6.7 km/s):
        // tens of W/cm2, ~1,600-1,700 K.
        let air = crate::planet::air(70000.);
        let q = stagnation(air.rho, 6700., 1.0);
        assert!(q > 2e5 && q < 8e5, "{q}");
        let (t, _) = radiative_equilibrium(q, 6700., air.t, 0.85, 1.);
        assert!(t > 1300. && t < 1900., "{t}");
    }
}
