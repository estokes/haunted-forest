//! The climb to orbit. The air-breathing part is flown by energy: along a
//! schedule of Mach and altitude, each segment's rise in specific energy
//! is bought at the cost in propellant of the best engine setting there —
//! turbine mode with reheat, its compressor unloaded as delivery nears its
//! limit, or the ramjet — until the rocket buys energy more cheaply. The
//! rocket part is integrated over the rotating planet with linear-tangent
//! steering, optimised for the least propellant, to an apoapsis at the
//! orbit's height, and a circularising burn there.

use crate::{
    aero::Aero,
    heat::{leading_edge, radiative_equilibrium, stagnation},
    optim::nelder_mead,
    planet::{air, Planet, State},
};
use anyhow::Result;
use hf_engine::{
    atmos::altitude_for_q,
    cycle::{rocket, EngineSpec, Mode, Point, PointReport},
    parts::G0,
};
use rayon::prelude::*;

pub type Table = Vec<((f64, f64), Vec<(&'static str, PointReport)>)>;

pub struct AscentSpec {
    pub engine: EngineSpec,
    /// The turbopump's rating, W.
    pub electric: f64,
    /// Dynamic pressure of the supersonic climb, Pa.
    pub q_climb: f64,
    /// Compressor delivery limit, K.
    pub t3_max: f64,
    /// Vacuum thrust in rocket mode, N.
    pub rocket_thrust: f64,
    /// Vacuum specific impulse, s, with nozzle losses.
    pub rocket_isp: f64,
    /// Nozzle exit area, m2, and the area ratio it gives.
    pub exit_area: f64,
    pub area_ratio: f64,
    /// kg of diluent per kg of dust in rocket mode, and the chamber's
    /// temperature on it.
    pub mix: f64,
    pub chamber_t: f64,
    pub orbit: f64,
    pub dt: f64,
    /// For the heating on the way up: nose and leading-edge radii, sweep.
    pub nose_radius: f64,
    pub edge_radius: f64,
    pub sweep: f64,
}

impl AscentSpec {
    /// Rocket mode burning `mix` kg of the engine's diluent per kg of dust
    /// at chamber pressure `pc` through a nozzle whose exit is `exit_area`
    /// m2, at 92% of ideal: the area ratio is what that exit gives the
    /// throat the thrust needs.
    pub fn new(
        engine: EngineSpec,
        rocket_thrust: f64,
        pc: f64,
        exit_area: f64,
        mix: f64,
    ) -> Result<Self> {
        let diluent = engine.turbopump.diluent;
        let mut ratio = 10.;
        let mut r = rocket(diluent, mix, pc, ratio)?;
        for _ in 0..6 {
            let throat = rocket_thrust / (r.isp_vacuum * G0) / r.throat_flux;
            ratio = exit_area / throat;
            r = rocket(diluent, mix, pc, ratio)?;
        }
        Ok(Self {
            engine,
            electric: 50e6,
            q_climb: 40e3,
            t3_max: 860.,
            rocket_thrust,
            rocket_isp: 0.92 * r.isp_vacuum,
            exit_area,
            area_ratio: ratio,
            mix,
            chamber_t: r.chamber_t,
            orbit: 200e3,
            dt: 0.1,
            nose_radius: 0.1,
            edge_radius: 0.03,
            sweep: 58f64.to_radians(),
        })
    }

    /// The air-breathing schedule: (Mach, altitude).
    pub fn schedule(&self) -> Vec<(f64, f64)> {
        let mut pts =
            vec![(0.3, 0.), (0.5, 1000.), (0.7, 3000.), (0.9, 7000.), (0.95, 10000.)];
        let mut m = 1.2;
        while m <= 5.01 {
            let z = altitude_for_q(m, self.q_climb).max(pts.last().unwrap().1);
            pts.push((m, z));
            m += 0.25;
        }
        pts
    }

    /// Every engine setting worth trying at Mach `m`, altitude `z`, at full
    /// power within the turbopump's rating.
    fn settings(&self, m: f64, z: f64) -> Vec<(&'static str, PointReport)> {
        let e = &self.engine;
        let mut out = Vec::new();
        for (fan, comp) in
            [(3.5, 3.5), (3.5, 2.5), (3.5, 1.5), (3.5, 1.), (2.5, 1.), (1.6, 1.)]
        {
            let pt = Point {
                altitude: z,
                mach: m,
                fan_pr: fan,
                compressor_pr: comp,
                t4: Some(2400.),
                t7: Some(2400.),
                ..Default::default()
            };
            let Ok(r) = e.evaluate(&pt) else { continue };
            if r.t3 > self.t3_max {
                continue;
            }
            if let Ok(pt) = e.within_power(&pt, self.electric)
                && let Ok(r) = e.evaluate(&pt)
            {
                out.push(("turbine", r));
            }
            break;
        }
        if m >= 2.5 {
            for (t4, label) in [
                (2400., "ramjet 2,400 K"),
                (3000., "ramjet 3,000 K"),
                (3500., "ramjet 3,500 K"),
            ] {
                let pt = Point {
                    altitude: z,
                    mach: m,
                    mode: Mode::Ramjet,
                    t4: Some(t4),
                    ..Default::default()
                };
                if let Ok(r) = e.evaluate(&pt) {
                    out.push((label, r));
                }
            }
        }
        out
    }

    /// The engine settings at each segment's midpoint, which do not depend
    /// on the aircraft's mass.
    pub fn engine_table(&self) -> Table {
        let s = self.schedule();
        s.windows(2)
            .collect::<Vec<_>>()
            .par_iter()
            .map(|w| {
                let mid = ((w[0].0 + w[1].0) / 2., (w[0].1 + w[1].1) / 2.);
                (mid, self.settings(mid.0, mid.1))
            })
            .collect()
    }

    fn rocket_thrust_at(&self, p: f64) -> f64 {
        self.rocket_thrust - p * self.exit_area
    }

    /// Fly from a standing start at mass `m0` to orbit.
    pub fn fly(&self, planet: &Planet, aero: &Aero, table: &Table, m0: f64) -> Ascent {
        let sched = self.schedule();
        let g = planet.surface_gravity();
        let mut out = Ascent::default();
        let mut m = m0;
        let mut gamma = 0.;
        let mut handover = sched[sched.len() - 1];
        for (i, (mid, settings)) in table.iter().enumerate() {
            let (a, b) = (sched[i], sched[i + 1]);
            let at = air(mid.1);
            let v = mid.0 * at.sound;
            let q = 0.5 * at.rho * v * v;
            let lift = m * (g - v * v / (planet.radius + mid.1));
            let drag = aero.drag(mid.0, q, lift.max(0.));
            let best = settings
                .iter()
                .filter(|(_, r)| r.thrust > drag)
                .map(|(label, r)| {
                    ((r.thrust - drag) * v / (r.diluent + r.dust), label, r)
                })
                .max_by(|x, y| x.0.total_cmp(&y.0));
            let rocket_e = (self.rocket_thrust_at(at.p) - drag) * v
                / (self.rocket_thrust / (self.rocket_isp * G0));
            let Some((e, label, r)) = best.filter(|(e, ..)| *e > rocket_e) else {
                handover = a;
                break;
            };
            let (va, vb) = (a.0 * air(a.1).sound, b.0 * air(b.1).sound);
            let de = g * (b.1 - a.1) + (vb * vb - va * va) / 2.;
            if de > 0. {
                let prop = m * de / e;
                let dt = m * de / ((r.thrust - drag) * v);
                out.air_time += dt;
                out.air_diluent += prop * r.diluent / (r.diluent + r.dust);
                out.air_dust += prop * r.dust / (r.diluent + r.dust);
                m -= prop;
                gamma = ((b.1 - a.1) / (v * dt)).clamp(-1., 1.).asin();
            }
            out.segments.push((b.0, b.1, *label, r.thrust, drag));
            handover = b;
        }
        out.handover_mach = handover.0;
        out.handover_altitude = handover.1;
        out.handover_mass = m;
        let at = air(handover.1);
        let v = handover.0 * at.sound;
        let start = State {
            r: planet.radius + handover.1,
            theta: 0.,
            vr: v * gamma.sin(),
            vt: v * gamma.cos() + planet.omega * (planet.radius + handover.1),
        };
        let cost = |p: [f64; 2]| -> f64 {
            let b = self.burn(planet, aero, start, m, p[0], p[1]);
            b.propellant
                + if b.reached { 0. } else { 1e6 }
                + if b.min_altitude < 15e3 { 1e5 } else { 0. }
        };
        let (best, _) =
            nelder_mead(cost, [30f64.to_radians(), 1e-3], [10f64.to_radians(), 5e-4], 80);
        out.rocket = self.burn(planet, aero, start, m, best[0], best[1]);
        out.steering = best;
        out.final_mass = m - out.rocket.propellant;
        out
    }

    /// The rocket burn from `s` at mass `m` with steering tan(pitch) =
    /// tan(p0) - c t above the local horizontal, to apoapsis at the orbit,
    /// then circularising there.
    pub fn burn(
        &self,
        planet: &Planet,
        aero: &Aero,
        mut s: State,
        m0: f64,
        p0: f64,
        c: f64,
    ) -> Burn {
        let m_dot = self.rocket_thrust / (self.rocket_isp * G0);
        let target = planet.radius + self.orbit;
        let mut b = Burn { min_altitude: f64::INFINITY, ..Default::default() };
        let mut m = m0;
        let mut t = 0.;
        while t < 2000. && m > 0.2 * m0 {
            let h = s.altitude(planet);
            b.min_altitude = b.min_altitude.min(h);
            if h < 0. {
                break;
            }
            let (ra, _) = s.apsides(planet);
            if ra >= target {
                b.reached = true;
                break;
            }
            let pitch = (p0.tan() - c * t).atan();
            let at = air(h);
            let thrust = self.rocket_thrust_at(at.p).max(0.);
            let v = s.airspeed(planet);
            let q = 0.5 * at.rho * v * v;
            b.max_q = b.max_q.max(q);
            if v > 300. {
                let alpha = (pitch - s.gamma(planet)).abs();
                let (tn, _) = radiative_equilibrium(
                    stagnation(at.rho, v, self.nose_radius),
                    v,
                    at.t,
                    0.85,
                    1.,
                );
                let (te, _) = radiative_equilibrium(
                    leading_edge(at.rho, v, self.edge_radius, self.sweep, alpha),
                    v,
                    at.t,
                    0.85,
                    1.,
                );
                if tn > b.nose_t {
                    (b.nose_t, b.nose_at) = (tn, (h, v));
                }
                b.edge_t = b.edge_t.max(te);
            }
            b.max_g = b.max_g.max(thrust / m / G0);
            let drag = if v > 1. { aero.drag(v / at.sound, q.max(1e-9), 0.) } else { 0. };
            let mm = m;
            s = s.step(planet, self.dt, |st| {
                let (vr, vt) = st.relative(planet);
                let vv = vr.hypot(vt).max(1e-6);
                (
                    (thrust * pitch.sin() - drag * vr / vv) / mm,
                    (thrust * pitch.cos() - drag * vt / vv) / mm,
                )
            });
            m -= m_dot * self.dt;
            t += self.dt;
        }
        b.burn_time = t;
        b.burn_propellant = m0 - m;
        // Coast to apoapsis and circularise.
        let (ra, rp) = s.apsides(planet);
        let a = (ra + rp) / 2.;
        let v_apo = (planet.mu * (2. / ra - 1. / a)).sqrt();
        b.circularise_dv = (planet.mu / ra).sqrt() - v_apo;
        let circ = m * (1. - (-b.circularise_dv / (self.rocket_isp * G0)).exp());
        b.propellant = b.burn_propellant + circ;
        b.dv = self.rocket_isp * G0 * (m0 / (m0 - b.propellant)).ln();
        b
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Burn {
    pub reached: bool,
    pub burn_time: f64,
    pub burn_propellant: f64,
    pub circularise_dv: f64,
    pub propellant: f64,
    /// The burn's ideal delta-v, m/s.
    pub dv: f64,
    pub max_q: f64,
    pub max_g: f64,
    pub min_altitude: f64,
    /// Peak nose and leading-edge temperatures, K, and the nose's (altitude,
    /// speed) then.
    pub nose_t: f64,
    pub nose_at: (f64, f64),
    pub edge_t: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Ascent {
    pub air_time: f64,
    pub air_diluent: f64,
    pub air_dust: f64,
    /// (Mach, altitude, setting, thrust, drag) at the end of each segment.
    pub segments: Vec<(f64, f64, &'static str, f64, f64)>,
    pub handover_mach: f64,
    pub handover_altitude: f64,
    pub handover_mass: f64,
    pub steering: [f64; 2],
    pub rocket: Burn,
    pub final_mass: f64,
}
