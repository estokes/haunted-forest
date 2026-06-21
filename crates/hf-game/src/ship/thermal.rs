//! Thermal management: the liquid-tin coolant loop and endcap radiators that
//! reject the reactor's waste heat to space. The reactor's other half.
//!
//! A three-node lumped network — blanket wall → tin coolant → endcap radiator —
//! ending in the only heat sink available in vacuum: εσAT⁴ radiation. When the
//! reactor is overdrawn, waste heat outruns the T⁴ ceiling, the tin climbs toward
//! its 2875 K boiloff, and the wall can no longer reject — so thermal reaches
//! back through the interconnect and throttles (or SCRAMs) the reactor. That
//! cascade is the gameplay.

use bevy::prelude::*;
use netidx::{path::Path, publisher::{FromValue, Value}};

use crate::net::{NetBridge, WriteInbox};
use crate::ship::constants::{
    radiator_capacity_w, RADIATOR_AREA_M2, RADIATOR_EMISSIVITY, SIGMA_SB, TIN_BOILING_K,
    TIN_DESIGN_K, TIN_LIQUIDUS_K,
};
use crate::ship::health::{worst, Stressor};
use crate::ship::instruments::NoiseClock;
use crate::ship::interconnect::Interconnect;
use crate::ship::value_enum::str_enum;

const K_ENDCAP: f64 = RADIATOR_EMISSIVITY * SIGMA_SB * RADIATOR_AREA_M2; // W/K^4 per endcap
const UA_B: f64 = 2.06e9; // blanket -> tin conductance [W/K]
const UA_R: f64 = 3.08e9; // tin -> radiator conductance [W/K]
const C_WALL: f64 = 8.0e8; // blanket heat capacity [J/K]
const C_TIN: f64 = 7.5e8; // tin loop heat capacity [J/K]
const C_RAD_WET: f64 = 1.81e9; // active (tin-filled) endcap heat capacity [J/K]
const C_RAD_DRY: f64 = 1.75e9; // drained endcap heat capacity [J/K]
const T_WALL_WARN: f64 = 2200.0;
const T_WALL_HARD: f64 = 2500.0; // SiC blanket failure
const T_TIN_WARN: f64 = 2700.0;
const T_TIN_FREEZE_WARN: f64 = 560.0;
const CAPACITY_REF_K: f64 = 2000.0; // reference temperature for the safe-rejection rating

str_enum! {
    pub enum ThermalStatus {
        Nominal => "nominal",
        Warning => "warning",
        Critical => "critical",
        Saturated => "saturated",
    }
}

str_enum! {
    pub enum RadiatorConfig {
        Single => "single",
        Dual => "dual",
    }
}

str_enum! {
    pub enum EndcapState {
        Active => "active",
        Iced => "iced",
        Draining => "draining",
        Warming => "warming",
    }
}

#[derive(Clone, Copy)]
struct Endcap {
    t_rad: f64,
    state: EndcapState,
}

#[derive(Resource)]
pub struct Thermal {
    t_wall: f64,
    t_tin: f64,
    endcaps: [Endcap; 2],
    flow_fraction: f64, // pump control, 0..1 of design flow
    heat_load: f64,     // W, from the interconnect
    rejected: f64,      // W radiated this tick
    capacity: f64,      // W, safe rejection rating
    config: RadiatorConfig,
    status: ThermalStatus,
    noise: NoiseClock,
}

impl Default for Thermal {
    fn default() -> Self {
        // Cruise config: aft endcap radiating, fore endcap iced for ablation
        // shielding. Temperatures near the full-power one-endcap equilibrium; the
        // demand-driven reactor settles them downward to the cruise load.
        Self {
            t_wall: 2042.0,
            t_tin: 1952.0,
            endcaps: [
                Endcap { t_rad: 1892.0, state: EndcapState::Active },
                Endcap { t_rad: 14.0, state: EndcapState::Iced },
            ],
            flow_fraction: 1.0,
            heat_load: 0.0,
            rejected: 0.0,
            capacity: radiator_capacity_w(CAPACITY_REF_K, 1),
            config: RadiatorConfig::Single,
            status: ThermalStatus::Nominal,
            noise: NoiseClock::default(),
        }
    }
}

impl Thermal {
    fn active_count(&self) -> f64 {
        self.endcaps.iter().filter(|e| e.state == EndcapState::Active).count().max(1) as f64
    }

    /// Advance the thermal network one tick. Reads reactor waste heat from the
    /// interconnect; writes back the headroom and coolant temperature the reactor
    /// reads next frame.
    pub fn step(&mut self, ic: &mut Interconnect, dt: f64) {
        self.noise.advance(dt);
        let p_waste = ic.reactor_waste_heat;
        self.heat_load = p_waste;

        // The flow clamp to [0,1] also bounds the node conductances, which is what
        // keeps the explicit-Euler integration stable (an unclamped pump_fraction
        // would stiffen the tin node toward oscillation).
        let flow_g = self.flow_fraction.clamp(0.0, 1.0);
        let g_b = UA_B * flow_g;
        let g_r = UA_R * flow_g;
        let n_active = self.active_count();

        // derivatives from current state (explicit Euler)
        let q_b = g_b * (self.t_wall - self.t_tin);
        let mut q_r = [0.0_f64; 2];
        let mut sum_q_r = 0.0;
        for (i, e) in self.endcaps.iter().enumerate() {
            if e.state == EndcapState::Active {
                q_r[i] = (g_r / n_active) * (self.t_tin - e.t_rad);
                sum_q_r += q_r[i];
            }
        }

        self.t_wall = (self.t_wall + (p_waste - q_b) / C_WALL * dt).max(3.0);
        self.t_tin = (self.t_tin + (q_b - sum_q_r) / C_TIN * dt).max(3.0);

        let mut rejected = 0.0;
        for (i, e) in self.endcaps.iter_mut().enumerate() {
            let p_rad = K_ENDCAP * e.t_rad.powi(4);
            match e.state {
                EndcapState::Active => {
                    e.t_rad = (e.t_rad + (q_r[i] - p_rad) / C_RAD_WET * dt).max(3.0);
                    rejected += p_rad;
                }
                EndcapState::Draining => {
                    e.t_rad = (e.t_rad - p_rad / C_RAD_DRY * dt).max(3.0);
                }
                EndcapState::Iced => e.t_rad = 14.0,
                EndcapState::Warming => {}
            }
        }
        self.rejected = rejected;
        self.capacity = radiator_capacity_w(CAPACITY_REF_K, n_active as u8);

        // Feedback to the reactor: a single headroom scalar from how far the wall
        // and tin are into their warn->hard bands. 1 = clear, 0 = throttle hard.
        let stress = ((self.t_wall - T_WALL_WARN) / (T_WALL_HARD - T_WALL_WARN))
            .max((self.t_tin - T_TIN_WARN) / (TIN_BOILING_K - T_TIN_WARN));
        ic.thermal_headroom = (1.0 - stress).clamp(0.0, 1.0);
        ic.coolant_return_temp = self.t_tin;

        self.status = if self.t_tin >= TIN_BOILING_K || self.t_wall >= T_WALL_HARD {
            ThermalStatus::Critical
        } else if self.t_wall >= T_WALL_WARN || self.t_tin >= T_TIN_WARN {
            ThermalStatus::Warning
        } else if self.heat_load > self.capacity {
            ThermalStatus::Saturated
        } else {
            ThermalStatus::Nominal
        };
    }

    /// Deploy or stow the second endcap. In-system both radiate (~2× capacity);
    /// at cruise one carries ablation ice. The multi-day flip thermal sequence is
    /// a fast-follow; v1 switches the state directly.
    fn set_config(&mut self, config: RadiatorConfig) {
        self.config = config;
        self.endcaps[1].state = match config {
            RadiatorConfig::Single => EndcapState::Iced,
            RadiatorConfig::Dual => EndcapState::Active,
        };
        if matches!(config, RadiatorConfig::Dual) && self.endcaps[1].t_rad < 100.0 {
            // bring the freshly-deployed panel up to a sane starting temperature
            self.endcaps[1].t_rad = self.t_tin.min(T_TIN_DESIGN_FALLBACK);
        }
    }

    pub fn tin_temp(&self) -> f64 {
        self.t_tin
    }
    pub fn wall_temp(&self) -> f64 {
        self.t_wall
    }
    pub fn status(&self) -> ThermalStatus {
        self.status
    }

    pub fn strain(&self) -> f64 {
        worst(&[
            Stressor { value: self.t_wall, soft: T_WALL_WARN, hard: T_WALL_HARD },
            Stressor { value: self.t_tin, soft: T_TIN_WARN, hard: TIN_BOILING_K },
            Stressor { value: self.t_tin, soft: T_TIN_FREEZE_WARN, hard: TIN_LIQUIDUS_K },
            Stressor {
                value: self.heat_load / self.capacity.max(1.0),
                soft: 0.85,
                hard: 1.0,
            },
        ])
    }
}

const T_TIN_DESIGN_FALLBACK: f64 = TIN_DESIGN_K;

// ---- Bevy system wrappers ----

pub fn thermal_physics(
    mut thermal: ResMut<Thermal>,
    mut interconnect: ResMut<Interconnect>,
    time: Res<Time>,
) {
    thermal.step(&mut interconnect, time.delta_secs_f64());
}

/// Thermal currently has no closed-loop control of its own (pump and radiator
/// config are player/AI commands); this keeps the system-set shape uniform.
pub fn thermal_auto_control() {}

pub fn thermal_publish(thermal: Res<Thermal>, bridge: Res<NetBridge>) {
    let base = "/ship/thermal";
    let instab = 0.0;
    let n = thermal.noise;
    let pub_noisy = |path: &str, value: f64, seed: f64| {
        bridge.publish(
            Path::from(format!("{base}/{path}")),
            Value::from(n.read(seed, value, instab)),
        );
    };
    pub_noisy("loop/wall_temp_k", thermal.t_wall, 1.0);
    pub_noisy("loop/tin_temp_k", thermal.t_tin, 2.0);
    pub_noisy("radiator/0/temp_k", thermal.endcaps[0].t_rad, 3.0);
    pub_noisy("radiator/1/temp_k", thermal.endcaps[1].t_rad, 4.0);
    pub_noisy("load/p_waste_gw", thermal.heat_load / 1e9, 5.0);
    pub_noisy("load/p_rejected_gw", thermal.rejected / 1e9, 6.0);
    pub_noisy("capacity_gw", thermal.capacity / 1e9, 7.0);

    bridge.publish(
        Path::from(format!("{base}/radiator/0/state")),
        Value::from(thermal.endcaps[0].state),
    );
    bridge.publish(
        Path::from(format!("{base}/radiator/1/state")),
        Value::from(thermal.endcaps[1].state),
    );
    bridge.publish(Path::from(format!("{base}/status")), Value::from(thermal.status));
    bridge.publish_writable(
        Path::from(format!("{base}/control/pump_fraction")),
        Value::from(thermal.flow_fraction),
    );
    bridge.publish_writable(
        Path::from(format!("{base}/control/config")),
        Value::from(thermal.config),
    );
}

pub fn thermal_apply_controls(mut thermal: ResMut<Thermal>, inbox: Res<WriteInbox<Thermal>>) {
    const PREFIX: &str = "/ship/thermal/control/";
    for w in inbox.drain() {
        let Some(name) = w.path.as_ref().strip_prefix(PREFIX) else {
            continue;
        };
        match name {
            "pump_fraction" => {
                if let Some(v) = f64::get(w.value) {
                    thermal.flow_fraction = v.clamp(0.0, 1.0);
                }
            }
            "config" => {
                if let Some(c) = RadiatorConfig::get(w.value) {
                    info!("thermal radiator config -> {c}");
                    thermal.set_config(c);
                }
            }
            _ => {}
        }
    }
}
