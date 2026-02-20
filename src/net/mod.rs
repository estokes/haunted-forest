mod bridge;

use bevy::prelude::*;
pub use bridge::*;

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        let bridge = NetBridge::start().expect("failed to start netidx bridge");
        app.insert_resource(bridge);
    }
}
