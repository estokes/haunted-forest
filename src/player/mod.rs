mod controller;

use bevy::prelude::*;
pub use controller::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSettings>()
            .add_systems(Startup, (spawn_player, grab_cursor))
            .add_systems(Update, (player_movement, player_look, toggle_cursor));
    }
}
