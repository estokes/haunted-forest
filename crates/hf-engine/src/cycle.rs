//! The electric air turbo-rocket at a flight condition, in turbine or
//! ramjet mode, and the vacuum performance of its rocket mode.
//!
//! Turbine mode: inlet, electric fan, low bypass split, electric
//! compressor, a main combustor fed by the turbopump's exhaust (and slurry
//! if the exhaust alone does not reach the set temperature), then either a
//! constant-pressure mixer with the bypass and reheat, or separate core
//! and bypass nozzles. Nozzles are fully expanded. Airflow scales with the
//! fan face's corrected flow. Ramjet mode: inlet, the bypass duct, the
//! main combustor, the nozzle; the turbopump makes only the electricity
//! asked of it.

use crate::{
    atmos::{isa, Air},
    gas::{brent, eq_hp, Diluent, Feed, Gas},
    parts::{burn, burn_to, compress, expand, mix, Stream, Turbopump, TurbopumpSpec, G0},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct EngineSpec {
    /// Air at sea level, static, full fan speed, kg/s.
    pub airflow: f64,
    pub bypass: f64,
    pub fan_pr: f64,
    pub compressor_pr: f64,
    pub eta_fan: f64,
    pub eta_compressor: f64,
    pub eta_motor: f64,
    /// Total-pressure ratio across each burner.
    pub burner_loss: f64,
    /// Total-pressure ratio of the ramjet's path from inlet to combustor.
    pub duct_loss: f64,
    /// Kinetic-energy efficiency of the nozzles.
    pub nozzle_eta: f64,
    /// Bypass Mach number at the mixer, which sets its pressure.
    pub mixer_mach: f64,
    /// Inlet capture area, m2: it caps the airflow in either mode.
    pub capture: f64,
    /// Avionics and hotel load, W.
    pub systems: f64,
    pub turbopump: TurbopumpSpec,
}

impl Default for EngineSpec {
    fn default() -> Self {
        Self {
            airflow: 90.,
            bypass: 0.4,
            fan_pr: 3.5,
            compressor_pr: 3.5,
            eta_fan: 0.88,
            eta_compressor: 0.88,
            eta_motor: 0.98,
            burner_loss: 0.95,
            duct_loss: 0.95,
            nozzle_eta: 0.96,
            mixer_mach: 0.5,
            capture: 0.6,
            systems: 0.5e6,
            turbopump: TurbopumpSpec::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Turbine,
    Ramjet,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Point {
    pub altitude: f64,
    pub mach: f64,
    pub air: Air,
    pub mode: Mode,
    /// Fraction of full corrected airflow, in turbine mode.
    pub throttle: f64,
    pub fan_pr: f64,
    pub compressor_pr: f64,
    /// Main combustor exit temperature, K: slurry is added if the
    /// turbopump's exhaust alone falls short. None adds none.
    pub t4: Option<f64>,
    /// Reheat exit temperature, K. None is no reheat.
    pub t7: Option<f64>,
    /// Mix the bypass into the core for reheat, or nozzle it separately.
    pub mixed: bool,
    /// Electricity for things other than the engine, W.
    pub extra_power: f64,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            altitude: 0.,
            mach: 0.,
            air: Air::Earth,
            mode: Mode::Turbine,
            throttle: 1.,
            fan_pr: 3.5,
            compressor_pr: 3.5,
            t4: None,
            t7: None,
            mixed: true,
            extra_power: 0.,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct PointReport {
    pub airflow: f64,
    /// N
    pub thrust: f64,
    /// W, from the turbopump.
    pub electric: f64,
    /// kg/s
    pub diluent: f64,
    pub dust: f64,
    /// s, on all propellant and on dust alone.
    pub isp: f64,
    pub isp_dust: f64,
    /// Compressor delivery, main combustor exit and nozzle entry, K.
    pub t3: f64,
    pub t4: f64,
    pub t7: f64,
    pub p3: f64,
    pub v_jet: f64,
    pub turbopump_exit_t: f64,
    /// Fraction of the propellant's flow that goes through the turbopump.
    pub turbopump_share: f64,
    /// Mass fraction of the exhaust that is soot, at the nozzle exit.
    pub soot: f64,
}

/// The air path's stagnation states in turbine mode. The nozzle's entry
/// is the reheat's exit: the mixed stream, or the core alone when the
/// bypass is nozzled separately.
#[derive(Clone, Copy, Debug)]
pub struct Stations {
    pub face: Gas,
    pub fan_exit: Stream,
    pub bypass: Stream,
    pub compressor_exit: Stream,
    pub combustor_exit: Stream,
    pub nozzle_entry: Stream,
    /// Static pressure outside, Pa.
    pub ambient: f64,
}

/// Inlet total-pressure recovery (MIL-E-5008B) with a 3% subsonic loss.
pub fn recovery(m: f64) -> f64 {
    0.97 * if m <= 1. {
        1.
    } else if m <= 5. {
        1. - 0.075 * (m - 1.).powf(1.35)
    } else {
        800. / (m.powi(4) + 935.)
    }
}

struct Freestream {
    p: f64,
    v: f64,
    /// Air the inlet captures, kg/s; at rest, unlimited.
    captured: f64,
    /// At the engine face, after the inlet.
    face: Gas,
}

fn freestream(pt: &Point, capture: f64) -> Freestream {
    let (t, p) = isa(pt.altitude);
    let v = pt.mach * (1.4 * 287.053 * t).sqrt();
    let k = 1. + 0.2 * pt.mach * pt.mach;
    let face = pt.air.gas(t * k, p * k.powf(3.5) * recovery(pt.mach));
    let captured = if v > 0. { p / (287.053 * t) * v * capture } else { f64::INFINITY };
    Freestream { p, v, captured, face }
}

impl EngineSpec {
    pub fn evaluate(&self, pt: &Point) -> Result<PointReport> {
        match pt.mode {
            Mode::Turbine => self.turbine(pt),
            Mode::Ramjet => self.ramjet(pt),
        }
    }

    fn turbine(&self, pt: &Point) -> Result<PointReport> {
        Ok(self.turbine_stations(pt)?.1)
    }

    /// The turbine-mode engine's stations at `pt`.
    pub fn stations(&self, pt: &Point) -> Result<Stations> {
        Ok(self.turbine_stations(pt)?.0)
    }

    fn turbine_stations(&self, pt: &Point) -> Result<(Stations, PointReport)> {
        let fs = freestream(pt, self.capture);
        let face = fs.face;
        let m = pt.throttle
            * (self.airflow * (face.p / 101325.) / (face.t / 288.15).sqrt())
                .min(fs.captured);
        let (m_core, m_bypass) =
            (m / (1. + self.bypass), m * self.bypass / (1. + self.bypass));
        let (g13, w_fan) = compress(face, pt.fan_pr, self.eta_fan)?;
        let (g3, w_comp) = compress(g13, pt.compressor_pr, self.eta_compressor)?;
        let electric = (m * w_fan + m_core * w_comp) / self.eta_motor
            + self.systems
            + pt.extra_power;
        let tp = Turbopump::run(&self.turbopump, electric, g3.p)?;
        let slurry = self.turbopump.slurry();
        let carrier = slurry.carrier;
        let p4 = g3.p * self.burner_loss;
        let core_parts = [(m_core, g3.feed()), tp.exhaust.part()];
        let (s4, d4) = match pt.t4 {
            Some(t) => burn_to(&core_parts, p4, t, slurry)?,
            None => (burn(&core_parts, p4)?, 0.),
        };
        let bypass = Stream { m: m_bypass, gas: g13 };
        let (thrust_flow, s7, v_jet, d7, soot) = if pt.mixed {
            let s6 = mix(&s4, &bypass, self.mixer_mach)?;
            let (s7, d7) = match pt.t7 {
                Some(t) => burn_to(&[s6.part()], s6.gas.p * self.burner_loss, t, slurry)?,
                None => (s6, 0.),
            };
            let (v, ge) = expand(&s7.gas, fs.p, self.nozzle_eta)?;
            (s7.m * v, s7, v, d7, ge.soot())
        } else {
            let (s7, d7) = match pt.t7 {
                Some(t) => burn_to(&[s4.part()], s4.gas.p * self.burner_loss, t, slurry)?,
                None => (s4, 0.),
            };
            let (vc, ge) = expand(&s7.gas, fs.p, self.nozzle_eta)?;
            let vb = expand(&g13, fs.p, self.nozzle_eta)?.0;
            (s7.m * vc + m_bypass * vb, s7, vc, d7, ge.soot() * s7.m / (s7.m + m_bypass))
        };
        let thrust = thrust_flow - m * fs.v;
        let diluent = tp.diluent + carrier * (d4 + d7);
        let dust = tp.dust + d4 + d7;
        let report = PointReport {
            airflow: m,
            thrust,
            electric,
            diluent,
            dust,
            isp: thrust / ((diluent + dust) * G0),
            isp_dust: thrust / (dust * G0),
            t3: g3.t,
            t4: s4.gas.t,
            t7: s7.gas.t,
            p3: g3.p,
            v_jet,
            turbopump_exit_t: tp.exhaust.gas.t,
            turbopump_share: (tp.diluent + tp.dust) / (diluent + dust),
            soot,
        };
        let stations = Stations {
            face,
            fan_exit: Stream { m, gas: g13 },
            bypass,
            compressor_exit: Stream { m: m_core, gas: g3 },
            combustor_exit: s4,
            nozzle_entry: s7,
            ambient: fs.p,
        };
        Ok((stations, report))
    }

    fn ramjet(&self, pt: &Point) -> Result<PointReport> {
        let fs = freestream(pt, self.capture);
        let m = fs.captured * pt.throttle;
        let p_duct = fs.face.p * self.duct_loss;
        let electric = self.systems + pt.extra_power;
        let tp = Turbopump::run(&self.turbopump, electric, p_duct)?;
        let slurry = self.turbopump.slurry();
        let carrier = slurry.carrier;
        let parts = [(m, fs.face.feed()), tp.exhaust.part()];
        let p4 = p_duct * self.burner_loss;
        let (s4, d4) = match pt.t4 {
            Some(t) => burn_to(&parts, p4, t, slurry)?,
            None => (burn(&parts, p4)?, 0.),
        };
        let (v, ge) = expand(&s4.gas, fs.p, self.nozzle_eta)?;
        let soot = ge.soot();
        let thrust = s4.m * v - m * fs.v;
        let diluent = tp.diluent + carrier * d4;
        let dust = tp.dust + d4;
        Ok(PointReport {
            airflow: m,
            thrust,
            electric,
            diluent,
            dust,
            isp: thrust / ((diluent + dust) * G0),
            isp_dust: thrust / (dust * G0),
            t3: fs.face.t,
            t4: s4.gas.t,
            t7: s4.gas.t,
            p3: fs.face.p,
            v_jet: v,
            turbopump_exit_t: tp.exhaust.gas.t,
            turbopump_share: (tp.diluent + tp.dust) / (diluent + dust),
            soot,
        })
    }

    /// The throttle at which this point gives `thrust`.
    pub fn throttle_for(&self, pt: &Point, thrust: f64) -> Result<f64> {
        brent(
            |k| Ok(self.evaluate(&Point { throttle: k, ..pt.clone() })?.thrust - thrust),
            0.02,
            1.,
            1e-5,
        )
    }

    /// The point throttled back, if need be, to draw no more than
    /// `electric` W from the turbopump.
    pub fn within_power(&self, pt: &Point, electric: f64) -> Result<Point> {
        if self.evaluate(pt)?.electric <= electric {
            return Ok(pt.clone());
        }
        let k = brent(
            |k| {
                Ok(self.evaluate(&Point { throttle: k, ..pt.clone() })?.electric
                    - electric)
            },
            0.02,
            pt.throttle,
            1e-5,
        )?;
        Ok(Point { throttle: k, ..pt.clone() })
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Rocket {
    pub chamber_t: f64,
    /// Vacuum specific impulse, s, shifting equilibrium, ideal nozzle.
    pub isp_vacuum: f64,
    pub exit_p: f64,
    /// m/s
    pub exit_v: f64,
    /// kg/s per m2 of throat.
    pub throat_flux: f64,
    /// Mass fraction of soot at the exit.
    pub soot: f64,
}

/// A dust rocket burning `mix` kg of diluent per kg of dust at chamber
/// pressure `pc`, through a nozzle of area ratio `area_ratio`. Soot, if
/// any, keeps pace with the gas: two-phase losses are not counted.
pub fn rocket(diluent: Diluent, mix: f64, pc: f64, area_ratio: f64) -> Result<Rocket> {
    let feed = Feed::blend(&[(1., Feed::dust()), (mix, diluent.feed(0.))]);
    let gc = eq_hp(&feed, pc, None)?;
    let (h0, s0) = (gc.h(), gc.s());
    let flux = |p: f64| -> Result<(f64, Gas, f64)> {
        let g = gc.eq_s(s0, p)?;
        let v = (2. * (h0 - g.h())).max(0.).sqrt();
        Ok((g.rho() * v, g, v))
    };
    let (mut a, mut b) = ((0.3 * pc).ln(), (0.9 * pc).ln());
    let phi = 0.618033988749895;
    for _ in 0..40 {
        let (c, d) = (b - phi * (b - a), a + phi * (b - a));
        if flux(c.exp())?.0 > flux(d.exp())?.0 {
            b = d
        } else {
            a = c
        }
    }
    let g_throat = flux(((a + b) / 2.).exp())?.0;
    let pt = ((a + b) / 2.).exp();
    let lnpe = brent(
        |lp| Ok(g_throat / flux(lp.exp())?.0 - area_ratio),
        (pc * 1e-9).ln(),
        pt.ln() - 1e-6,
        1e-10,
    )?;
    let (_, ge, ve) = flux(lnpe.exp())?;
    Ok(Rocket {
        chamber_t: gc.t,
        isp_vacuum: (ve + ge.p / (ge.rho() * ve)) / G0,
        exit_p: ge.p,
        exit_v: ve,
        throat_flux: g_throat,
        soot: ge.soot(),
    })
}
