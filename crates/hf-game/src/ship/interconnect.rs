//! Ground-truth coupling bus between the reactor and thermal systems.
//!
//! The tight 64 Hz reactor<->thermal loop reads and writes these fields directly
//! — no netidx, no latency, no sensor noise — because routing a control loop
//! through the 16 ms batched, noise-injected bus would be sluggish and lie to the
//! controller. Every field here is *also* published to netidx for observation and
//! scripting; we simulate tightly but expose fully.

use bevy::prelude::*;

use crate::ship::constants::TIN_DESIGN_K;

#[derive(Resource)]
pub struct Interconnect {
    /// Reactor -> thermal, written THIS frame [W]: the heat the reactor dumps
    /// into the blanket/coolant.
    pub reactor_waste_heat: f64,
    /// Thermal -> reactor, from the PREVIOUS frame (0..1): 1 = radiators have
    /// margin, 0 = saturated, throttle the reactor.
    pub thermal_headroom: f64,
    /// Thermal -> reactor, from the PREVIOUS frame [K]: coolant temperature
    /// returning to the first wall — the reactor's heat-rejection boundary.
    pub coolant_return_temp: f64,
}

impl Default for Interconnect {
    fn default() -> Self {
        Self {
            reactor_waste_heat: 0.0,
            thermal_headroom: 1.0,
            coolant_return_temp: TIN_DESIGN_K,
        }
    }
}
