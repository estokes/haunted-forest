mod bridge;
mod hud;
pub mod constants;
pub mod health;
pub mod instruments;
pub mod interconnect;
pub mod reactor;
pub mod thermal;
mod value_enum;

use bevy::prelude::*;
use netidx::path::Path;
pub use bridge::*;

use crate::GameState;
use crate::net::NetBridge;

/// Phases of a ship-system tick: integrate physics, then run control, then age
/// the health/strain channel. Chaining makes the coupling order explicit.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShipSet {
    Physics,
    Control,
    Health,
}

/// Give each system its own typed control-write inbox at startup.
fn register_writes(mut commands: Commands, bridge: Res<NetBridge>) {
    commands.insert_resource(
        bridge.register_writes::<reactor::Reactor>(Path::from("/ship/reactor/control/")),
    );
    commands.insert_resource(
        bridge.register_writes::<thermal::Thermal>(Path::from("/ship/thermal/control/")),
    );
}

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<reactor::Reactor>()
            .init_resource::<thermal::Thermal>()
            .init_resource::<interconnect::Interconnect>()
            .init_resource::<health::ShipHealth>()
            .insert_resource(reactor::ReactorDebugTimer(0.0))
            .init_resource::<hud::ReactorReadouts>()
            .add_systems(
                Startup,
                (spawn_bridge, register_writes, hud::spawn_hud, hud::subscribe_reactor),
            )
            .configure_sets(
                FixedUpdate,
                (ShipSet::Physics, ShipSet::Control, ShipSet::Health).chain(),
            )
            .add_systems(
                FixedUpdate,
                (reactor::reactor_physics, thermal::thermal_physics)
                    .chain()
                    .in_set(ShipSet::Physics),
            )
            .add_systems(
                FixedUpdate,
                (reactor::reactor_auto_control, thermal::thermal_auto_control)
                    .in_set(ShipSet::Control),
            )
            .add_systems(FixedUpdate, health::ship_health_update.in_set(ShipSet::Health))
            .add_systems(FixedUpdate, reactor::reactor_debug_log.after(ShipSet::Health))
            .add_systems(
                Update,
                (
                    reactor::reactor_keyboard_controls.run_if(in_state(GameState::Playing)),
                    reactor::reactor_apply_controls,
                    thermal::thermal_apply_controls,
                    reactor::reactor_publish,
                    thermal::thermal_publish,
                    health::publish_health,
                    hud::update_readouts,
                    hud::update_hud,
                )
                    .chain(),
            );
    }
}
