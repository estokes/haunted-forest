mod graphix;
mod llm;
mod net;
mod player;
mod ship;

use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    InteractingWithScreen,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Haunted Forest".into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins((
            net::NetPlugin,
            graphix::GraphixPlugin,
            player::PlayerPlugin,
            ship::ShipPlugin,
            llm::LlmPlugin,
        ))
        .run();
}
