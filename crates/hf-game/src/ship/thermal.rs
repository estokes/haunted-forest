//! Thermal management: the liquid-tin loop and zoned aft radiators that
//! reject the reactor's waste heat to space. The reactor's other half.
//!
//! A three-node lumped network — blanket wall → tin coolant → radiator zones —
//! ending in εσAT⁴. Zones can be valved off so the ship stops painting a
//! bearing with waste heat; remaining area runs hotter or the plant throttles.
//! Overdraw into too little live area is the cascade.

use bevy::prelude::*;
use netidx::{
    path::Path,
    publisher::{FromValue, Value},
};

use crate::net::{NetBridge, WriteInbox};
use crate::ship::constants::{
    radiator_capacity_w, N_RADIATOR_ZONES, RADIATOR_AREA_M2, RADIATOR_EMISSIVITY, SIGMA_SB,
    TIN_BOILING_K, TIN_LIQUIDUS_K,
};
use crate::ship::health::{worst, Stressor};
use crate::ship::instruments::NoiseClock;
use crate::ship::interconnect::Interconnect;
use crate::ship::value_enum::str_enum;

const K_ZONE: f64 =
    RADIATOR_EMISSIVITY * SIGMA_SB * RADIATOR_AREA_M2 / N_RADIATOR_ZONES as f64; // W/K^4
const UA_B: f64 = 4.0e8; // blanket -> tin [W/K]
const UA_R_TOTAL: f64 = 6.25e8; // tin -> all live zones [W/K]
const C_WALL: f64 = 1.6e8; // blanket heat capacity [J/K]
const C_TIN: f64 = 1.5e8; // tin loop heat capacity [J/K]
const C_RAD_LIVE: f64 = 4.0e8; // live zone heat capacity [J/K]
const C_RAD_DEAD: f64 = 3.8e8; // bypassed zone [J/K]
const T_WALL_WARN: f64 = 2200.0;
const T_WALL_HARD: f64 = 2600.0;
const T_TIN_WARN: f64 = 2200.0;
const T_TIN_FREEZE_WARN: f64 = 560.0;
const CAPACITY_REF_K: f64 = 1850.0; // rated rejection with all zones lit

str_enum! {
    pub enum ThermalStatus {
        Nominal => "nominal",
        Warning => "warning",
        Critical => "critical",
        Saturated => "saturated",
    }
}

#[derive(Clone, Copy)]
struct Zone {
    t_rad: f64,
    live: bool,
}

#[derive(Resource)]
pub struct Thermal {
    t_wall: f64,
    t_tin: f64,
    zones: [Zone; N_RADIATOR_ZONES],
    flow_fraction: f64,
    heat_load: f64,
    rejected: f64,
    capacity: f64,
    status: ThermalStatus,
    noise: NoiseClock,
}

impl Default for Thermal {
    fn default() -> Self {
        Self {
            t_wall: 2000.0,
            t_tin: 1900.0,
            zones: [Zone { t_rad: 1850.0, live: true }; N_RADIATOR_ZONES],
            flow_fraction: 1.0,
            heat_load: 0.0,
            rejected: 0.0,
            capacity: radiator_capacity_w(CAPACITY_REF_K, N_RADIATOR_ZONES as u8),
            status: ThermalStatus::Nominal,
            noise: NoiseClock::default(),
        }
    }
}

impl Thermal {
    fn n_live(&self) -> usize {
        self.zones.iter().filter(|z| z.live).count().max(1)
    }

    pub fn zone_mask(&self) -> u8 {
        self.zones.iter().enumerate().fold(0u8, |acc, (i, z)| {
            if z.live {
                acc | (1 << i)
            } else {
                acc
            }
        })
    }

    pub fn set_zone_mask(&mut self, mask: u8) {
        for (i, z) in self.zones.iter_mut().enumerate() {
            z.live = (mask & (1 << i)) != 0;
        }
    }

    /// Advance the thermal network one tick. Reads reactor waste heat from the
    /// interconnect; writes back the headroom and coolant temperature the reactor
    /// reads next frame.
    pub fn step(&mut self, ic: &mut Interconnect, dt: f64) {
        self.noise.advance(dt);
        let p_waste = ic.reactor_waste_heat;
        self.heat_load = p_waste;

        let flow_g = self.flow_fraction.clamp(0.0, 1.0);
        let g_b = UA_B * flow_g;
        let n_live = self.n_live() as f64;
        let g_r = UA_R_TOTAL * flow_g;

        let q_b = g_b * (self.t_wall - self.t_tin);
        let mut q_r = [0.0_f64; N_RADIATOR_ZONES];
        let mut sum_q_r = 0.0;
        for (i, z) in self.zones.iter().enumerate() {
            if z.live {
                q_r[i] = (g_r / n_live) * (self.t_tin - z.t_rad);
                sum_q_r += q_r[i];
            }
        }

        self.t_wall = (self.t_wall + (p_waste - q_b) / C_WALL * dt).max(3.0);
        self.t_tin = (self.t_tin + (q_b - sum_q_r) / C_TIN * dt).max(3.0);

        let mut rejected = 0.0;
        for (i, z) in self.zones.iter_mut().enumerate() {
            let p_rad = K_ZONE * z.t_rad.powi(4);
            if z.live {
                z.t_rad = (z.t_rad + (q_r[i] - p_rad) / C_RAD_LIVE * dt).max(3.0);
                rejected += p_rad;
            } else {
                z.t_rad = (z.t_rad - p_rad / C_RAD_DEAD * dt).max(3.0);
            }
        }
        self.rejected = rejected;
        self.capacity = radiator_capacity_w(CAPACITY_REF_K, n_live as u8);

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

pub fn thermal_physics(
    mut thermal: ResMut<Thermal>,
    mut interconnect: ResMut<Interconnect>,
    time: Res<Time>,
) {
    thermal.step(&mut interconnect, time.delta_secs_f64());
}

pub fn thermal_auto_control() {}

pub fn thermal_publish(thermal: Res<Thermal>, bridge: Res<NetBridge>) {
    let base = "/ship/thermal";
    let n = thermal.noise;
    let pub_noisy = |path: &str, value: f64, seed: f64| {
        bridge.publish(
            Path::from(format!("{base}/{path}")),
            Value::from(n.read(seed, value, 0.0)),
        );
    };
    pub_noisy("loop/wall_temp_k", thermal.t_wall, 1.0);
    pub_noisy("loop/tin_temp_k", thermal.t_tin, 2.0);
    for (i, z) in thermal.zones.iter().enumerate() {
        pub_noisy(&format!("radiator/{i}/temp_k"), z.t_rad, 3.0 + i as f64);
        bridge.publish(Path::from(format!("{base}/radiator/{i}/live")), Value::from(z.live));
    }
    pub_noisy("load/p_waste_gw", thermal.heat_load / 1e9, 12.0);
    pub_noisy("load/p_rejected_gw", thermal.rejected / 1e9, 13.0);
    pub_noisy("capacity_gw", thermal.capacity / 1e9, 14.0);

    bridge.publish(Path::from(format!("{base}/status")), Value::from(thermal.status));
    bridge.publish_writable(
        Path::from(format!("{base}/control/pump_fraction")),
        Value::from(thermal.flow_fraction),
    );
    bridge.publish_writable(
        Path::from(format!("{base}/control/zone_mask")),
        Value::from(thermal.zone_mask() as f64),
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
            "zone_mask" => {
                if let Some(v) = f64::get(w.value) {
                    let mask = (v.round() as i64).clamp(0, 255) as u8;
                    info!("thermal zone mask -> {mask:#010b}");
                    thermal.set_zone_mask(mask);
                }
            }
            _ => {}
        }
    }
}
