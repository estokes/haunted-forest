mod bridge;
mod hud;
pub mod reactor;

use bevy::prelude::*;
pub use bridge::*;

use crate::GameState;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<reactor::Reactor>()
            .insert_resource(reactor::ReactorDebugTimer(0.0))
            .init_resource::<hud::ReactorReadouts>()
            .add_systems(Startup, (spawn_bridge, hud::spawn_hud, hud::subscribe_reactor))
            .add_systems(
                FixedUpdate,
                (reactor::reactor_physics, reactor::reactor_auto_control, reactor::reactor_debug_log).chain(),
            )
            .add_systems(
                Update,
                (
                    reactor::reactor_keyboard_controls.run_if(in_state(GameState::Playing)),
                    reactor::reactor_apply_controls,
                    reactor::reactor_publish,
                    hud::update_readouts,
                    hud::update_hud,
                ).chain(),
            );
    }
}
