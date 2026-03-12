mod controller;

use bevy::prelude::*;
pub use controller::*;

use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSettings>()
            .add_systems(Startup, (spawn_player, grab_cursor))
            .add_systems(
                Update,
                (
                    player_movement.run_if(in_state(GameState::Playing)),
                    player_look.run_if(in_state(GameState::Playing)),
                ),
            );
    }
}
