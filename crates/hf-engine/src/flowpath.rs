//! The air path's preliminary design: a meanline through the fan and the
//! compressor at the cycle's design point, sea level static at full
//! airflow, and the areas of the ducts behind them, dry and in reheat.
//!
//! Each rotor does an equal share of its component's work. The flow is a
//! free vortex (r V_theta constant) at constant axial velocity through
//! each component; the fan keeps its casing radius, the compressor its
//! mean radius. A stator after each rotor turns the flow back to axial.
//! Counter-rotating fan rotors, each on its own motor, pass the flow from
//! one to the next with no stator: at equal tip speed and equal work the
//! second takes out exactly the swirl the first put in. Blade counts are
//! set so that no radius exceeds the design diffusion factor, with chords
//! from a fixed span-to-chord ratio.
//!
//! Losses, for an efficiency to set beside the cycle's assumed one:
//! profile, from the diffusion factor through a fit to Lieblein's wake
//! momentum thickness; secondary and end-wall, Howell's; and the passage
//! shock of a supersonic row, a normal shock at the mean of its relative
//! inlet Mach number and 1. First-order, for CFD to settle.
//!
//! Sections are circular arcs at zero incidence, their exit metal angle
//! from Carter's rule. The fan's pitch range is what the hover asks of
//! its first rotor: at the hover's airflow and pressure ratio, the speed
//! at which its blades, turning the flow as much as at design, do the
//! hover's work, and the pitch that meets the flow there.

use crate::{
    cycle::{EngineSpec, Point, Stations},
    gas::{brent, Gas},
    parts::compress,
    thermo::R,
};
use anyhow::{bail, Result};
use serde::Serialize;
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct FlowpathSpec {
    pub fan_rotors: usize,
    pub counter_rotating: bool,
    /// The fan's tip speed, m/s.
    pub fan_tip_speed: f64,
    /// Hub over tip radius at the fan face.
    pub fan_hub_tip: f64,
    /// Axial Mach number at the fan face and at the compressor's entry.
    pub fan_mach: f64,
    pub compressor_mach: f64,
    pub compressor_stages: usize,
    /// The compressor's mean radius, m, below the fan's hub: a duct turns
    /// the core in to it. Its blade speed there, m/s.
    pub compressor_radius: f64,
    pub compressor_speed: f64,
    /// Diffusion factor no radius of any row exceeds.
    pub diffusion: f64,
    /// Span over chord: fan, compressor.
    pub aspect: (f64, f64),
    /// Thickness over chord at hub and tip.
    pub thickness: (f64, f64),
    /// The hover: fan pressure ratio and fraction of design airflow.
    pub hover: (f64, f64),
}

impl Default for FlowpathSpec {
    fn default() -> Self {
        Self {
            fan_rotors: 2,
            counter_rotating: false,
            fan_tip_speed: 500.,
            fan_hub_tip: 0.35,
            fan_mach: 0.6,
            compressor_mach: 0.5,
            compressor_stages: 3,
            compressor_radius: 0.22,
            compressor_speed: 420.,
            diffusion: 0.45,
            aspect: (1.6, 1.2),
            thickness: (0.08, 0.03),
            hover: (1.3, 0.9),
        }
    }
}

/// Combustor reference Mach number, and liner length over height; the
/// pre-diffuser ahead of its dump, m.
const COMBUSTOR: (f64, f64) = (0.1, 3.);
const DIFFUSER: f64 = 0.1;
/// Bypass duct Mach number; reheat duct Mach number and length over
/// diameter.
const BYPASS_MACH: f64 = 0.4;
const REHEAT: (f64, f64) = (0.25, 1.5);
/// Axial gap between rows, in chords; wall between streams, m.
const ROW_GAP: f64 = 0.25;
const WALL: f64 = 0.03;
/// Superconducting machines: W/kg, and kg/m3 with their cryostats. A
/// rotor's motor is a ring in the hub from its leading edge aft, its bore
/// MOTOR_BORE of its outside, MOTOR_GAP inside the hub's wall; the next
/// rotor stands clear of it.
pub const MOTOR_SPECIFIC: f64 = 100e3;
pub const MACHINE_DENSITY: f64 = 6000.;
const MOTOR_BORE: f64 = 0.6;
const MOTOR_GAP: f64 = 0.015;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Component {
    Fan,
    Compressor,
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct Section {
    pub r: f64,
    /// Flow angles in and out, from axial, degrees, in the row's frame.
    pub flow_in: f64,
    pub flow_out: f64,
    pub metal_in: f64,
    pub metal_out: f64,
    pub stagger: f64,
    pub tc: f64,
    /// Inlet Mach number in the row's frame.
    pub mach: f64,
    pub diffusion: f64,
    pub solidity: f64,
    pub loss: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Row {
    pub component: Component,
    /// Sense of rotation, +1 or -1; 0 for a stator.
    pub spin: i8,
    pub rpm: f64,
    pub blades: usize,
    pub chord: f64,
    pub x_le: f64,
    pub x_te: f64,
    /// (leading edge, trailing edge), m.
    pub hub: (f64, f64),
    pub tip: (f64, f64),
    /// Hub, mean, tip.
    pub sections: [Section; 3],
    /// J/kg, rotors.
    pub work: f64,
    /// Entropy the row makes, J/kg K.
    pub ds: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Duct {
    pub x0: f64,
    pub x1: f64,
    pub r_in: f64,
    pub r_out: f64,
}

/// The mixer's plane, inside the reheat duct's wall: the core in an
/// annulus round the tail cone, the bypass outside the lobes.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Mixer {
    pub x: f64,
    pub r_cone: f64,
    pub r_lobes: f64,
    pub r_out: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Nozzle {
    pub throat: f64,
    pub exit: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Motor {
    pub kg: f64,
    pub x0: f64,
    pub x1: f64,
    pub r_in: f64,
    pub r_out: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Pitch {
    /// The first rotor's speed over design, and its pitch change at the
    /// mean radius and at the tip, degrees (negative is toward axial).
    pub speed: f64,
    pub mean: f64,
    pub tip: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Flowpath {
    pub rows: Vec<Row>,
    /// Where the bypass splits from the core: x, radius.
    pub splitter: (f64, f64),
    pub combustor: Duct,
    pub bypass: Duct,
    pub mixer: Mixer,
    pub reheat: Duct,
    /// Radii, m, dry and in reheat.
    pub nozzle: (Nozzle, Nozzle),
    /// Isentropic efficiency the meanline's losses give, fan and compressor.
    pub eta: (f64, f64),
    pub pitch: Pitch,
    /// One per rotor, rated for the larger of its powers at the design
    /// point and at full airflow at Mach 2 and 11 km, where ram makes the
    /// fan and compressor want the most.
    pub motors: Vec<Motor>,
}

/// Static state of a stagnation state `g` moving at `v`, frozen.
fn static_at(g: &Gas, v: f64) -> Gas {
    let (cp, k) = (g.cp(), g.gamma());
    let t = g.t - v * v / (2. * cp);
    g.at(t, g.p * (t / g.t).powf(k / (k - 1.)))
}

fn sound(g: &Gas) -> f64 {
    (g.gamma() * R / g.mw() * g.t).sqrt()
}

/// Total-pressure ratio across a normal shock at Mach `m`.
fn shock(m: f64, k: f64) -> f64 {
    let m2 = m * m;
    ((k + 1.) * m2 / ((k - 1.) * m2 + 2.)).powf(k / (k - 1.))
        * ((k + 1.) / (2. * k * m2 - (k - 1.))).powf(1. / (k - 1.))
}

/// A row seen at one radius: speeds in and out, tangential change, flow
/// angles in and out (radians, in the row's frame).
struct Triangle {
    w1: f64,
    w2: f64,
    turn: f64,
    b1: f64,
    b2: f64,
}

impl Triangle {
    fn new(vx: f64, wt1: f64, wt2: f64) -> Self {
        Self {
            w1: vx.hypot(wt1),
            w2: vx.hypot(wt2),
            turn: (wt1 - wt2).abs(),
            b1: (wt1 / vx).atan(),
            b2: (wt2 / vx).atan(),
        }
    }

    /// Solidity at which the diffusion factor is `df`.
    fn solidity_for(&self, df: f64) -> Option<f64> {
        let room = df - 1. + self.w2 / self.w1;
        (room > 0.).then(|| self.turn / (2. * self.w1 * room))
    }

    fn diffusion(&self, sigma: f64) -> f64 {
        1. - self.w2 / self.w1 + self.turn / (2. * sigma * self.w1)
    }
}

/// One component's rows from inlet `g_in` to outlet `g_out`.
struct Builder<'a> {
    spec: &'a FlowpathSpec,
    component: Component,
    m: f64,
    g_in: Gas,
    g_out: Gas,
    rotors: usize,
    vx: f64,
    x: f64,
    /// r V_theta of the flow entering the next row, absolute, in the
    /// first rotor's sense.
    swirl: f64,
    /// Each rotor's motor, kg.
    motor_kg: f64,
    motors: Vec<Motor>,
}

impl Builder<'_> {
    /// Stagnation state after `k` of the rotors: equal work, the pressure
    /// geometric between the ends.
    fn total(&self, k: usize) -> Result<Gas> {
        let f = k as f64 / self.rotors as f64;
        let h = self.g_in.h() + f * (self.g_out.h() - self.g_in.h());
        self.g_in.frozen_h(h, self.g_in.p * (self.g_out.p / self.g_in.p).powf(f))
    }

    /// Annulus area for the flow at stagnation state `g` with swirl `rvt`
    /// at mean radius `rm`.
    fn area(&self, g: &Gas, rvt: f64, rm: f64) -> f64 {
        let v = self.vx.hypot(rvt / rm);
        self.m / (static_at(g, v).rho() * self.vx)
    }

    /// Hub and tip radii for an annulus of `area`: the fan keeps its
    /// casing at `fixed`, the compressor its mean radius.
    fn radii(&self, area: f64, fixed: f64) -> (f64, f64) {
        match self.component {
            Component::Fan => ((fixed * fixed - area / PI).sqrt(), fixed),
            Component::Compressor => {
                let h = area / (2. * PI * fixed);
                (fixed - h / 2., fixed + h / 2.)
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn row(
        &mut self,
        spin: i8,
        omega: f64,
        g1: &Gas,
        g2: &Gas,
        swirl_out: f64,
        r_in: (f64, f64),
        r_out: (f64, f64),
        aspect: f64,
    ) -> Result<Row> {
        let s = self.spec;
        let rs = |(h, t): (f64, f64)| [h, ((h * h + t * t) / 2.).sqrt(), t];
        let radii = rs(((r_in.0 + r_out.0) / 2., (r_in.1 + r_out.1) / 2.));
        let sense = if spin == 0 { 1. } else { spin as f64 };
        let tri = |r: f64| {
            let (vt1, vt2) = (sense * self.swirl / r, sense * swirl_out / r);
            let u = omega * r;
            if spin == 0 {
                Triangle::new(self.vx, vt1, vt2)
            } else {
                Triangle::new(self.vx, u - vt1, u - vt2)
            }
        };
        let span = ((r_in.1 - r_in.0) + (r_out.1 - r_out.0)) / 2.;
        let chord = span / aspect;
        let mut blades = 0.;
        for &r in &radii {
            let Some(sigma) = tri(r).solidity_for(s.diffusion) else {
                bail!(
                    "{:?} row at r {r:.3} m: W2/W1 {:.2} cannot reach diffusion factor {}",
                    self.component,
                    tri(r).w2 / tri(r).w1,
                    s.diffusion
                );
            };
            blades = f64::max(blades, sigma * 2. * PI * r / chord);
        }
        let blades = blades.ceil() as usize;
        let (k, mw) = (g1.gamma(), g1.mw());
        let mut sections = [Section::default(); 3];
        let (mut ds, mut weight) = (0., 0.);
        for (i, &r) in radii.iter().enumerate() {
            let t = tri(r);
            let sigma = blades as f64 * chord / (2. * PI * r);
            let df = t.diffusion(sigma);
            let theta = 0.0045 + 0.025 * df.powi(4) / (1. - df).max(0.05);
            let profile = 2. * sigma * theta / t.b2.cos();
            let tan_m = (t.b1.tan() + t.b2.tan()) / 2.;
            let cos_m = tan_m.atan().cos();
            let lift = 2. / sigma * cos_m * (t.b1.tan() - t.b2.tan());
            let pitch = 2. * PI * r / blades as f64;
            let howell = sigma
                * (0.018 * lift * lift + 0.02 * pitch / span)
                * t.b1.cos().powi(2)
                / cos_m.powi(3);
            let st = static_at(g1, self.vx.hypot(self.swirl / r));
            let mach = t.w1 / sound(&st);
            let p0_over_p = (1. + (k - 1.) / 2. * mach * mach).powf(k / (k - 1.));
            let passage = if mach > 1. {
                (1. - shock((mach + 1.) / 2., k)) * p0_over_p / (p0_over_p - 1.)
            } else {
                0.
            };
            let loss = profile + howell + passage;
            ds += -(R / mw) * (1. - loss * (1. - 1. / p0_over_p)).ln() * r;
            weight += r;
            let (k1, b2) = (t.b1.to_degrees(), t.b2.to_degrees());
            let carter = (0.23 + b2 / 500.) / sigma.sqrt();
            let k2 = (b2 - carter * k1) / (1. - carter);
            let span_frac = (r - radii[0]) / (radii[2] - radii[0]);
            sections[i] = Section {
                r,
                flow_in: k1,
                flow_out: b2,
                metal_in: k1,
                metal_out: k2,
                stagger: (k1 + k2) / 2.,
                tc: s.thickness.0 + (s.thickness.1 - s.thickness.0) * span_frac,
                mach,
                diffusion: df,
                solidity: sigma,
                loss,
            };
        }
        let axial = chord * sections[1].stagger.to_radians().cos();
        let row = Row {
            component: self.component,
            spin,
            rpm: omega * 60. / (2. * PI),
            blades,
            chord,
            x_le: self.x,
            x_te: self.x + axial,
            hub: (r_in.0, r_out.0),
            tip: (r_in.1, r_out.1),
            sections,
            work: g2.h() - g1.h(),
            ds: ds / weight,
        };
        self.x += axial + ROW_GAP * chord;
        self.swirl = swirl_out;
        Ok(row)
    }

    /// The component's rows. `fixed` is the fan's casing radius or the
    /// compressor's mean radius; `speed`, the blade speed there.
    fn build(&mut self, fixed: f64, speed: f64) -> Result<Vec<Row>> {
        let s = *self.spec;
        let (aspect, counter) = match self.component {
            Component::Fan => (s.aspect.0, s.counter_rotating),
            Component::Compressor => (s.aspect.1, false),
        };
        let omega = speed / fixed;
        let mut rows = Vec::new();
        let mut g1 = self.g_in;
        let mut r1 = self.radii(self.area(&g1, self.swirl, fixed), fixed);
        for k in 0..self.rotors {
            let spin: i8 = if counter && k % 2 == 1 { -1 } else { 1 };
            let g2 = self.total(k + 1)?;
            let rm = ((r1.0 * r1.0 + r1.1 * r1.1) / 2.).sqrt();
            let swirl_out = self.swirl + spin as f64 * (g2.h() - g1.h()) / omega;
            let r2 = self.radii(self.area(&g2, swirl_out, rm), fixed);
            let rotor = self.row(spin, omega, &g1, &g2, swirl_out, r1, r2, aspect)?;
            let r_out = rotor.hub.1 - MOTOR_GAP;
            let length = self.motor_kg / MACHINE_DENSITY / (PI * r_out * r_out * (1. - MOTOR_BORE * MOTOR_BORE));
            let motor = Motor { kg: self.motor_kg, x0: rotor.x_le, x1: rotor.x_le + length, r_in: MOTOR_BORE * r_out, r_out };
            rows.push(rotor);
            let stator = !counter || k + 1 == self.rotors;
            if stator && swirl_out.abs() > 1e-6 {
                let r3 = self.radii(self.area(&g2, 0., rm), fixed);
                rows.push(self.row(0, 0., &g2, &g2, 0., r2, r3, aspect)?);
                r1 = r3;
            } else {
                r1 = r2;
            }
            self.x = self.x.max(motor.x1 + MOTOR_GAP);
            self.motors.push(motor);
            g1 = g2;
        }
        Ok(rows)
    }
}

/// Throat and exit radii of a nozzle passing `stream` to `ambient`, ideal
/// expansion, frozen.
fn nozzle(g: &Gas, m: f64, ambient: f64) -> Nozzle {
    let k = g.gamma();
    let star = g.at(g.t * 2. / (k + 1.), g.p * (2. / (k + 1.)).powf(k / (k - 1.)));
    let te = g.t * (ambient / g.p).powf((k - 1.) / k);
    let exit = g.at(te, ambient);
    let exit_area = m / (exit.rho() * (2. * g.cp() * (g.t - te)).sqrt());
    let throat = if ambient < star.p { m / (star.rho() * sound(&star)) } else { exit_area };
    let r = |a: f64| (a / PI).sqrt();
    Nozzle { throat: r(throat), exit: r(exit_area) }
}

impl FlowpathSpec {
    pub fn design(&self, engine: &EngineSpec) -> Result<Flowpath> {
        let dry = engine.stations(&Point::default())?;
        let wet = engine
            .stations(&Point { t4: Some(2400.), t7: Some(2400.), ..Point::default() })?;
        let Stations { face, fan_exit, bypass, compressor_exit, combustor_exit, nozzle_entry, ambient } =
            dry;
        let m = fan_exit.m;
        let rating = engine.stations(&Point { altitude: 11000., mach: 2., ..Point::default() })?;
        let motor_kg = |c: Component| {
            let power = |st: &Stations| match c {
                Component::Fan => st.fan_exit.m * (st.fan_exit.gas.h() - st.face.h()) / self.fan_rotors as f64,
                Component::Compressor => {
                    st.compressor_exit.m * (st.compressor_exit.gas.h() - st.fan_exit.gas.h())
                        / self.compressor_stages as f64
                }
            };
            power(&dry).max(power(&rating)) / engine.eta_motor / MOTOR_SPECIFIC
        };

        let k = face.gamma();
        let ts = face.t / (1. + (k - 1.) / 2. * self.fan_mach.powi(2));
        let vx = self.fan_mach * (k * R / face.mw() * ts).sqrt();
        let a_face = m / (static_at(&face, vx).rho() * vx);
        let r_tip = (a_face / (PI * (1. - self.fan_hub_tip.powi(2)))).sqrt();
        let mut fan = Builder {
            spec: self,
            component: Component::Fan,
            m,
            g_in: face,
            g_out: fan_exit.gas,
            rotors: self.fan_rotors,
            vx,
            x: 0.,
            swirl: 0.,
            motor_kg: motor_kg(Component::Fan),
            motors: Vec::new(),
        };
        let mut rows = fan.build(r_tip, self.fan_tip_speed)?;
        let last = rows.last().unwrap();
        let (hub, tip) = (last.hub.1, last.tip.1);
        let r_split = (hub * hub + (tip * tip - hub * hub) * compressor_exit.m / m).sqrt();
        let split_x = fan.x;

        let gc = fan_exit.gas;
        let kc = gc.gamma();
        let tsc = gc.t / (1. + (kc - 1.) / 2. * self.compressor_mach.powi(2));
        let vxc = self.compressor_mach * (kc * R / gc.mw() * tsc).sqrt();
        let r_mean = self.compressor_radius;
        let r_core = ((hub * hub + r_split * r_split) / 2.).sqrt();
        let mut comp = Builder {
            spec: self,
            component: Component::Compressor,
            m: compressor_exit.m,
            g_in: gc,
            g_out: compressor_exit.gas,
            rotors: self.compressor_stages,
            vx: vxc,
            x: split_x + 0.05 + 2. * (r_core - r_mean).abs(),
            swirl: 0.,
            motor_kg: motor_kg(Component::Compressor),
            motors: Vec::new(),
        };
        rows.extend(comp.build(r_mean, self.compressor_speed)?);

        let eta = |c: Component, g_in: &Gas, g_out: &Gas| {
            let ds: f64 = rows.iter().filter(|r| r.component == c).map(|r| r.ds).sum();
            1. - g_out.t * ds / (g_out.h() - g_in.h())
        };
        let eta = (
            eta(Component::Fan, &face, &fan_exit.gas),
            eta(Component::Compressor, &gc, &compressor_exit.gas),
        );

        let g3 = compressor_exit.gas;
        let k3 = g3.gamma();
        let t3s = g3.t / (1. + (k3 - 1.) / 2. * COMBUSTOR.0.powi(2));
        let v3 = COMBUSTOR.0 * (k3 * R / g3.mw() * t3s).sqrt();
        let a_comb = compressor_exit.m / (static_at(&g3, v3).rho() * v3);
        let h_comb = a_comb / (2. * PI * r_mean);
        let x_comb = comp.x + DIFFUSER;
        let combustor = Duct {
            x0: x_comb,
            x1: x_comb + COMBUSTOR.1 * h_comb,
            r_in: r_mean - h_comb / 2.,
            r_out: r_mean + h_comb / 2.,
        };

        let gb = bypass.gas;
        let kb = gb.gamma();
        let area_at = |g: &Gas, mdot: f64, mach: f64| {
            let k = g.gamma();
            let t = g.t / (1. + (k - 1.) / 2. * mach * mach);
            let v = mach * (k * R / g.mw() * t).sqrt();
            mdot / (static_at(g, v).rho() * v)
        };
        let core_out = combustor.r_out + WALL;
        let a_bypass = area_at(&gb, bypass.m, BYPASS_MACH);
        let bypass_duct = Duct {
            x0: split_x,
            x1: combustor.x1,
            r_in: core_out,
            r_out: (core_out * core_out + a_bypass / PI).sqrt(),
        };
        let p_mix = gb.p / (1. + (kb - 1.) / 2. * engine.mixer_mach.powi(2)).powf(kb / (kb - 1.));
        let g4 = combustor_exit.gas;
        let k4 = g4.gamma();
        let t4s = g4.t * (p_mix / g4.p).powf((k4 - 1.) / k4);
        let v4 = (2. * g4.cp() * (g4.t - t4s)).sqrt();
        let a_core_mix = combustor_exit.m / (g4.at(t4s, p_mix).rho() * v4);
        let a_bypass_mix = area_at(&gb, bypass.m, engine.mixer_mach);
        let r_reheat = (area_at(&nozzle_entry.gas, nozzle_entry.m, REHEAT.0) / PI).sqrt();
        let r_lobes = (r_reheat * r_reheat - a_bypass_mix / PI).sqrt();
        let r_cone_sq = r_lobes * r_lobes - a_core_mix / PI;
        if r_cone_sq < 0. {
            bail!("the mixer's streams need more than the reheat duct's {r_reheat:.3} m radius");
        }
        let mixer = Mixer { x: combustor.x1 + 2. * (combustor.r_out - combustor.r_in), r_cone: r_cone_sq.sqrt(), r_lobes, r_out: r_reheat };
        let reheat = Duct {
            x0: mixer.x,
            x1: mixer.x + REHEAT.1 * 2. * r_reheat,
            r_in: 0.,
            r_out: r_reheat,
        };
        let nozzles = (
            nozzle(&nozzle_entry.gas, nozzle_entry.m, ambient),
            nozzle(&wet.nozzle_entry.gas, wet.nozzle_entry.m, wet.ambient),
        );

        let pitch = self.pitch(engine, &face, &rows[0], vx, a_face)?;
        let motors = fan.motors.into_iter().chain(comp.motors).collect();

        Ok(Flowpath {
            rows,
            splitter: (split_x, r_split),
            combustor,
            bypass: bypass_duct,
            mixer,
            reheat,
            nozzle: nozzles,
            eta,
            pitch,
            motors,
        })
    }

    /// The first rotor in the hover.
    fn pitch(&self, engine: &EngineSpec, face: &Gas, rotor: &Row, vx: f64, area: f64) -> Result<Pitch> {
        let m = engine.airflow * self.hover.1;
        let mut vxh = vx * self.hover.1;
        for _ in 0..20 {
            vxh = m / (static_at(face, vxh).rho() * area);
        }
        let (_, w) = compress(*face, self.hover.0, engine.eta_fan)?;
        let work = w / self.fan_rotors as f64;
        let omega = rotor.rpm * 2. * PI / 60.;
        let change = |s: &Section| -> Result<(f64, f64)> {
            let turning = (s.flow_in - s.flow_out).to_radians();
            let k = brent(
                |k: f64| {
                    let u = k * omega * s.r;
                    let b1 = (u / vxh).atan();
                    let vt2 = u - vxh * (b1 - turning).tan();
                    Ok(u * vt2 - work)
                },
                0.05,
                1.5,
                1e-9,
            )?;
            let b1 = (k * omega * s.r / vxh).atan().to_degrees();
            Ok((k, b1 - s.flow_in))
        };
        let (speed, mean) = change(&rotor.sections[1])?;
        let tip = {
            let s = &rotor.sections[2];
            let u = speed * omega * s.r;
            (u / vxh).atan().to_degrees() - s.flow_in
        };
        Ok(Pitch { speed, mean, tip })
    }
}
