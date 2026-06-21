//! The uniform strain/condition channel every ship system exposes.
//!
//! This is the substrate for the "machines as living things" / sub-threshold-AI
//! arc: each system reports how hard it is being pushed *right now* (`strain`),
//! how much it *remembers* being run hard (`cumulative_stress`), and how worn it
//! has become (`condition`). The crew AI reads these off the same ground truth
//! the player does, so it can never hallucinate the ship's state.

use bevy::prelude::*;
use netidx::{path::Path, publisher::Value};

use crate::net::NetBridge;
use crate::ship::reactor::Reactor;
use crate::ship::thermal::Thermal;

/// A single monitored quantity against a soft (warn) and hard (limit) threshold.
/// `hard > soft` is an upper limit; `hard < soft` is a lower limit (e.g. tin
/// freezing). Strain is 0 at/below `soft`, 1 at/beyond `hard`, linear between.
#[derive(Clone, Copy)]
pub struct Stressor {
    pub value: f64,
    pub soft: f64,
    pub hard: f64,
}

impl Stressor {
    pub fn strain(self) -> f64 {
        let Self { value, soft, hard } = self;
        if hard == soft {
            return if value >= hard { 1.0 } else { 0.0 };
        }
        ((value - soft) / (hard - soft)).clamp(0.0, 1.0)
    }
}

/// Worst-case strain across a set of stressors — a system is as strained as its
/// most-pressed limit.
pub fn worst(stressors: &[Stressor]) -> f64 {
    stressors.iter().copied().map(Stressor::strain).fold(0.0, f64::max)
}

const STRESS_TAU_UP: f64 = 8.0;
const STRESS_TAU_DOWN: f64 = 120.0;
const STRAIN_DAMAGE: f64 = 0.85;
const COND_WEAR_RATE: f64 = 2.0e-4;
const FRAILTY_GAIN: f64 = 1.0;
const COND_MIN: f64 = 0.2;

/// How one system "feels". `cumulative_stress` is an asymmetric saturating EMA
/// of strain (NOT a leaky integrator): it rises fast under sustained strain and
/// relaxes slowly, settling at the sustained level — the temperament signal.
/// `condition` only ever degrades, and degrades faster the worse it already is.
#[derive(Clone, Copy)]
pub struct SystemHealth {
    pub strain: f64,
    pub cumulative_stress: f64,
    pub condition: f64,
}

impl Default for SystemHealth {
    fn default() -> Self {
        Self { strain: 0.0, cumulative_stress: 0.0, condition: 1.0 }
    }
}

impl SystemHealth {
    pub fn update(&mut self, strain: f64, dt: f64) {
        let Self { strain: s, cumulative_stress, condition } = self;
        *s = strain.clamp(0.0, 1.0);
        let tau = if *s > *cumulative_stress { STRESS_TAU_UP } else { STRESS_TAU_DOWN };
        *cumulative_stress =
            (*cumulative_stress + (*s - *cumulative_stress) * (dt / tau)).clamp(0.0, 1.0);
        let over = (*s - STRAIN_DAMAGE).max(0.0);
        if over > 0.0 {
            let frailty = 1.0 + (1.0 - *condition) * FRAILTY_GAIN;
            *condition = (*condition - over * COND_WEAR_RATE * frailty * dt).max(COND_MIN);
        }
    }

    /// Performance multiplier: 1.0 fresh, down to `COND_MIN` when worn.
    pub fn performance(&self) -> f64 {
        self.condition
    }

    /// Added instability/failure proneness as the system wears (0 fresh → 0.8 worn).
    pub fn failure_bias(&self) -> f64 {
        1.0 - self.condition
    }
}

/// Per-system health for the whole ship. The `update` destructure forces every
/// new system to be wired here or the build breaks — the bookkeeping the
/// compiler does so we don't have to.
#[derive(Resource, Default)]
pub struct ShipHealth {
    pub reactor: SystemHealth,
    pub thermal: SystemHealth,
}

/// This frame's strain reading from each system.
pub struct StrainInputs {
    pub reactor: f64,
    pub thermal: f64,
}

impl ShipHealth {
    pub fn update(&mut self, inputs: StrainInputs, dt: f64) {
        let Self { reactor, thermal } = self;
        let StrainInputs { reactor: rs, thermal: ts } = inputs;
        reactor.update(rs, dt);
        thermal.update(ts, dt);
    }
}

// ---- Bevy systems ----

/// Gather this frame's strain from each system and age the health channel.
pub fn ship_health_update(
    mut health: ResMut<ShipHealth>,
    reactor: Res<Reactor>,
    thermal: Res<Thermal>,
    time: Res<Time>,
) {
    let inputs = StrainInputs { reactor: reactor.strain(), thermal: thermal.strain() };
    health.update(inputs, time.delta_secs_f64());
}

/// Publish the uniform `/ship/<system>/health/{strain,stress,condition}` channel.
pub fn publish_health(health: Res<ShipHealth>, bridge: Res<NetBridge>) {
    let publish = |sys: &str, h: &SystemHealth| {
        let base = format!("/ship/{sys}/health");
        bridge.publish(Path::from(format!("{base}/strain")), Value::from(h.strain));
        bridge.publish(Path::from(format!("{base}/stress")), Value::from(h.cumulative_stress));
        bridge.publish(Path::from(format!("{base}/condition")), Value::from(h.condition));
    };
    publish("reactor", &health.reactor);
    publish("thermal", &health.thermal);
}
