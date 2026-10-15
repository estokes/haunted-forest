//! Short takeoff and landing on a dirt strip. The takeoff run shares the
//! turbopump's power between the engine, in reheat, and the lift fan; the
//! nozzle is swivelled down as far as pitch balance allows, the rear
//! nozzle's lift being at most `balance` of the fan's about the centre of
//! gravity. The fan pays the ram drag of the air it swallows. The aircraft
//! lifts off when wing lift and thrust lift carry its weight. Landing
//! touches down at the slowest speed that thrust lift and wing lift hold,
//! then brakes on dirt with the wing spoiled and the thrust lift cut.

use anyhow::Result;
use hf_engine::{
    cycle::{EngineSpec, Point, PointReport},
    hover::{calibrated_efficiency, RHO_SL},
};
use std::f64::consts::PI;

pub struct StripSpec {
    pub rolling: f64,
    pub braking: f64,
    /// Wing lift coefficient on the takeoff run at its rotation attitude,
    /// and on approach.
    pub cl_ground: f64,
    pub cl_approach: f64,
    pub cd_ground: f64,
    /// After touchdown, spoiled.
    pub cl_rollout: f64,
    pub wing: f64,
    pub fan_diameter: f64,
    /// Electric power the lift fan's motor takes at most, W.
    pub fan_max: f64,
    pub turbopump: f64,
    pub balance: f64,
    pub eta_motor: f64,
}

impl Default for StripSpec {
    fn default() -> Self {
        Self {
            rolling: 0.05,
            braking: 0.3,
            cl_ground: 0.6,
            cl_approach: 0.8,
            cd_ground: 0.12,
            cl_rollout: 0.1,
            wing: 41.,
            fan_diameter: 1.5,
            fan_max: 20e6,
            turbopump: 35e6,
            balance: 0.5,
            eta_motor: 0.98,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Takeoff {
    pub roll: f64,
    pub speed: f64,
    pub engine_power: f64,
    pub fan_power: f64,
    pub nozzle_angle: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Landing {
    pub touchdown: f64,
    pub roll: f64,
}

impl StripSpec {
    fn fan_area(&self) -> f64 {
        PI * self.fan_diameter * self.fan_diameter / 4.
    }

    /// Lift-fan thrust, N, on `electric` W.
    pub fn fan_thrust(&self, electric: f64) -> f64 {
        let shaft = electric * self.eta_motor;
        (shaft * calibrated_efficiency() * (4. * RHO_SL * self.fan_area()).sqrt())
            .powf(2. / 3.)
    }

    /// The engine at sea level in reheat, from idle to full, as (electric
    /// W, static report).
    pub fn engine_curve(engine: &EngineSpec) -> Vec<(f64, PointReport)> {
        (2..=20)
            .filter_map(|k| {
                let pt = Point {
                    throttle: k as f64 / 20.,
                    t4: Some(2400.),
                    t7: Some(2400.),
                    ..Default::default()
                };
                engine.evaluate(&pt).ok().map(|r| (r.electric, r))
            })
            .collect()
    }

    fn roll(&self, mass: f64, engine: &PointReport, fan_power: f64) -> Takeoff {
        let w = mass * 9.80665;
        let tf = self.fan_thrust(fan_power);
        let fan_jet = (tf / (RHO_SL * self.fan_area())).sqrt();
        let fan_flow = tf / fan_jet.max(1.);
        let (mut v, mut x, dt) = (0., 0., 0.02);
        let mut angle = 0.;
        while x < 5000. {
            let tn = (engine.thrust - engine.airflow * v).max(0.);
            let vertical_n = (self.balance * tf).min(tn);
            angle = (vertical_n / tn.max(1.)).asin();
            let q = 0.5 * RHO_SL * v * v;
            let lift = q * self.wing * self.cl_ground + tf + vertical_n;
            if lift >= w {
                break;
            }
            let push = tn * angle.cos()
                - q * self.wing * self.cd_ground
                - fan_flow * v
                - self.rolling * (w - lift);
            if push <= 0. {
                x = f64::INFINITY;
                break;
            }
            v += push / mass * dt;
            x += v * dt;
        }
        Takeoff {
            roll: x,
            speed: v,
            engine_power: engine.electric,
            fan_power,
            nozzle_angle: angle,
        }
    }

    /// The shortest takeoff run at `mass` over the split of power.
    pub fn takeoff(&self, mass: f64, curve: &[(f64, PointReport)]) -> Takeoff {
        curve
            .iter()
            .filter(|(p, _)| *p < self.turbopump)
            .map(|(p, r)| self.roll(mass, r, (self.turbopump - p).min(self.fan_max)))
            .min_by(|a, b| a.roll.total_cmp(&b.roll))
            .unwrap_or_default()
    }

    /// The heaviest takeoff within `length` m.
    pub fn heaviest(&self, length: f64, curve: &[(f64, PointReport)]) -> Result<f64> {
        hf_engine::gas::brent(
            |m| Ok(self.takeoff(m, curve).roll - length),
            8000.,
            40000.,
            1e-6,
        )
    }

    /// Land at `mass`: the slowest touchdown thrust lift and wing lift
    /// hold, then the braked roll.
    pub fn land(&self, mass: f64, curve: &[(f64, PointReport)]) -> Landing {
        let w = mass * 9.80665;
        let thrust_lift = curve
            .iter()
            .filter(|(p, _)| *p < self.turbopump)
            .map(|(p, r)| {
                let tf = self.fan_thrust((self.turbopump - p).min(self.fan_max));
                tf + (self.balance * tf).min(r.thrust)
            })
            .fold(0., f64::max);
        let need = (w - thrust_lift).max(0.);
        let touchdown = (2. * need / (RHO_SL * self.wing * self.cl_approach)).sqrt();
        let (mut v, mut x, dt) = (touchdown, 0., 0.02);
        while v > 0. {
            let q = 0.5 * RHO_SL * v * v;
            let normal = (w - q * self.wing * self.cl_rollout).max(0.);
            let decel = (self.braking * normal + q * self.wing * self.cd_ground) / mass;
            v -= decel * dt;
            x += v.max(0.) * dt;
        }
        Landing { touchdown, roll: x }
    }
}
