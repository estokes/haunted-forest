//! Entry from a circular orbit, in the plane of an eastward equatorial
//! track over a rotating planet. A deorbit burn drops periapsis; from the
//! entry interface the fighter flies belly-first at a high angle of attack
//! and banks to hold an equilibrium glide, as the Shuttle did, lowering
//! its angle of attack as it slows. Heating is computed along the way at
//! the nose, the wing's leading edge, stations along the windward
//! centreline, and the lee side; each surface radiates at equilibrium.

use crate::{
    aero::Aero,
    heat::{
        flat_plate, leading_edge, radiative_equilibrium, stagnation, Insulation, Soak,
        SIGMA,
    },
    planet::{air, Planet, State},
};

pub struct EntrySpec {
    pub mass: f64,
    pub orbit: f64,
    /// Periapsis the deorbit burn aims for, m.
    pub periapsis: f64,
    pub interface: f64,
    /// Angle of attack above Mach 10, and at and below Mach 4, rad.
    pub alpha_high: f64,
    pub alpha_low: f64,
    pub nose_radius: f64,
    pub edge_radius: f64,
    pub sweep: f64,
    /// Windward centreline stations, m from the nose.
    pub stations: [f64; 4],
    pub emissivity: f64,
    /// Free-stream Reynolds number on x at which the windward flow trips.
    pub transition: f64,
    /// Lee-side heating as a fraction of the windward centreline's.
    pub lee: f64,
    /// Damping of the flight-path angle in the glide, 1/s.
    pub glide_gain: f64,
    pub dt: f64,
}

impl Default for EntrySpec {
    fn default() -> Self {
        Self {
            mass: 16000.,
            orbit: 200e3,
            periapsis: 40e3,
            interface: 120e3,
            alpha_high: 40f64.to_radians(),
            alpha_low: 20f64.to_radians(),
            nose_radius: 0.1,
            edge_radius: 0.03,
            sweep: 58f64.to_radians(),
            stations: [1., 3., 6., 10.],
            emissivity: 0.85,
            transition: 2e6,
            lee: 0.03,
            glide_gain: 0.05,
            dt: 0.2,
        }
    }
}

/// The peak of one surface's heating through entry, and its total.
#[derive(Clone, Copy, Debug, Default)]
pub struct Peak {
    /// W/m2 at radiative equilibrium.
    pub flux: f64,
    pub temperature: f64,
    pub time: f64,
    pub altitude: f64,
    pub speed: f64,
    /// J/m2 over the entry.
    pub load: f64,
}

impl Peak {
    fn record(
        &mut self,
        flux: f64,
        temperature: f64,
        time: f64,
        altitude: f64,
        speed: f64,
        dt: f64,
    ) {
        self.load += flux * dt;
        if temperature > self.temperature {
            *self = Peak { flux, temperature, time, altitude, speed, load: self.load };
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct EntryReport {
    pub deorbit_dv: f64,
    pub interface_speed: f64,
    pub interface_gamma: f64,
    pub duration: f64,
    pub downrange: f64,
    pub peak_g: f64,
    pub peak_q: f64,
    pub nose: Peak,
    pub edge: Peak,
    pub stations: [Peak; 4],
    pub turbulent_from: f64,
    pub lee: Peak,
    pub end_altitude: f64,
    pub end_mach: f64,
    /// (time, windward skin temperature at the 6 m station).
    pub skin: Vec<(f64, f64)>,
}

impl EntrySpec {
    fn alpha(&self, mach: f64) -> f64 {
        let f = ((mach - 4.) / 6.).clamp(0., 1.);
        self.alpha_low + f * (self.alpha_high - self.alpha_low)
    }

    pub fn fly(&self, planet: &Planet, aero: &Aero) -> EntryReport {
        let (ra, rp) = (planet.radius + self.orbit, planet.radius + self.periapsis);
        let a = (ra + rp) / 2.;
        let v_apo = (planet.mu * (2. / ra - 1. / a)).sqrt();
        let mut rep = EntryReport {
            deorbit_dv: planet.circular_speed(self.orbit) - v_apo,
            ..Default::default()
        };
        let ri = planet.radius + self.interface;
        let v = (planet.mu * (2. / ri - 1. / a)).sqrt();
        let vt = ra * v_apo / ri;
        let mut s = State { r: ri, theta: 0., vr: -(v * v - vt * vt).sqrt(), vt };
        rep.interface_speed = s.airspeed(planet);
        rep.interface_gamma = s.gamma(planet);
        rep.turbulent_from = f64::INFINITY;
        let m = self.mass;
        let mut t = 0.;
        loop {
            let h = s.altitude(planet);
            let at = air(h);
            let v = s.airspeed(planet);
            let mach = v / at.sound;
            if mach < 2. || h < 15e3 || t > 4000. {
                rep.end_altitude = h;
                rep.end_mach = mach;
                break;
            }
            let alpha = self.alpha(mach);
            let (cl, cd) = aero.newtonian(mach, alpha);
            let q = 0.5 * at.rho * v * v;
            let (lift, drag) = (q * aero.wing * cl, q * aero.wing * cd);
            let gamma = s.gamma(planet);
            let g_eff = planet.mu / (s.r * s.r) - s.vt * s.vt / s.r;
            let wanted = m * (g_eff - self.glide_gain * v * gamma);
            let cos_bank = if lift > 0. { (wanted / lift).clamp(-1., 1.) } else { 1. };
            rep.peak_g = rep.peak_g.max(lift.hypot(drag) / m / 9.80665);
            rep.peak_q = rep.peak_q.max(q);

            // Heating.
            let dt = self.dt;
            let eq =
                |q: f64, k: f64| radiative_equilibrium(q, v, at.t, self.emissivity, k);
            let (tn, qn) = eq(stagnation(at.rho, v, self.nose_radius), 1.);
            rep.nose.record(qn, tn, t, h, v, dt);
            let (te, qe) =
                eq(leading_edge(at.rho, v, self.edge_radius, self.sweep, alpha), 1.);
            rep.edge.record(qe, te, t, h, v, dt);
            let mut skin6 = at.t;
            for (i, &x) in self.stations.iter().enumerate() {
                let turbulent = at.rho * v * x / at.viscosity > self.transition;
                if turbulent {
                    rep.turbulent_from = rep.turbulent_from.min(t);
                }
                let k = if turbulent { 1.11 } else { 1. };
                // Hot-wall temperature only matters below 3,962 m/s.
                let (tw, qw) = eq(flat_plate(at.rho, v, x, alpha, turbulent, 1500.), k);
                rep.stations[i].record(qw, tw, t, h, v, dt);
                if x == 6. {
                    skin6 = tw;
                }
                if i == 2 {
                    let (tl, ql) = eq(
                        self.lee * flat_plate(at.rho, v, x, alpha, turbulent, 1500.),
                        k,
                    );
                    rep.lee.record(ql, tl, t, h, v, dt);
                }
            }
            rep.skin.push((t, skin6));

            s = s.step(planet, dt, |st| {
                let (vr, vt) = st.relative(planet);
                let vv = vr.hypot(vt);
                let at = air(st.altitude(planet));
                let q = 0.5 * at.rho * vv * vv;
                let mach = vv / at.sound;
                let (cl, cd) = aero.newtonian(mach, self.alpha(mach));
                let (l, d) = (q * aero.wing * cl / m * cos_bank, q * aero.wing * cd / m);
                // Drag along -v; lift perpendicular to v, upward in the plane.
                ((-d * vr + l * vt) / vv, (-d * vt - l * vr) / vv)
            });
            t += dt;
        }
        rep.duration = t;
        rep.downrange = s.theta * planet.radius;
        rep
    }
}

/// Heat into a belly tank behind `insulation`, from the 6 m station's skin
/// temperature through entry and then while the skin cools in subsonic
/// flight and on the ground: a hot structure of `skin_capacity` J/m2 K
/// radiating and convecting to 288 K air.
pub fn tank_soak(
    rep: &EntryReport,
    insulation: &Insulation,
    t_tank: f64,
    skin_capacity: f64,
    eps: f64,
) -> Soak {
    let end = rep.duration;
    let t_end = rep.skin.last().map_or(300., |p| p.1);
    // Precompute the cooling skin after entry on a 1 s grid.
    let mut cooling = Vec::new();
    let mut ts = t_end;
    for k in 0..7200 {
        cooling.push(ts);
        let h_conv = if k < 1200 { 100. } else { 10. };
        ts -= (eps * SIGMA * (ts.powi(4) - 288f64.powi(4)) + h_conv * (ts - 288.))
            / skin_capacity;
    }
    let skin = |t: f64| -> f64 {
        if t <= end {
            let i = ((t / end) * (rep.skin.len() - 1) as f64) as usize;
            rep.skin[i.min(rep.skin.len() - 1)].1
        } else {
            cooling[((t - end) as usize).min(cooling.len() - 1)]
        }
    };
    insulation.soak(skin, t_tank, end + 7200.)
}
