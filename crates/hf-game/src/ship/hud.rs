use bevy::prelude::*;
use netidx::{path::Path, publisher::FromValue};
use ahash::AHashMap;
use crate::net::NetBridge;
use super::reactor::{ReactorMode, ReactorStatus};

/// Stores the latest values received from netidx subscriptions.
#[derive(Resource, Default)]
pub struct ReactorReadouts {
    pub values: AHashMap<String, f64>,
    pub status: Option<ReactorStatus>,
    pub mode: Option<ReactorMode>,
}

#[derive(Component)]
pub struct ReactorHud;

/// Subscribe to all reactor paths on startup.
pub fn subscribe_reactor(bridge: Res<NetBridge>) {
    let base = "/ship/reactor";
    for path in [
        "plasma/temperature",
        "plasma/pressure",
        "plasma/density",
        "plasma/field_strength",
        "plasma/containment",
        "output/power",
        "output/efficiency",
        "control/fuel_rate",
        "control/cooling_rate",
        "control/target_power",
        "control/mode",
        "status",
    ] {
        bridge.subscribe(Path::from(format!("{base}/{path}")));
    }
}

/// Process netidx subscription updates into the ReactorReadouts resource.
pub fn update_readouts(bridge: Res<NetBridge>, mut readouts: ResMut<ReactorReadouts>) {
    for update in bridge.drain_updates() {
        let key = update.path.as_ref();
        if key.ends_with("/status") {
            readouts.status = ReactorStatus::get(update.value);
        } else if key.ends_with("/control/mode") {
            readouts.mode = ReactorMode::get(update.value);
        } else if let Some(v) = f64::get(update.value) {
            readouts.values.insert(key.to_string(), v);
        }
    }
}

pub fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        ReactorHud,
        Text::new("Reactor: ---"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.0, 0.8, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(12.0),
            right: px(12.0),
            ..default()
        },
    ));
}

pub fn update_hud(readouts: Res<ReactorReadouts>, mut hud: Query<&mut Text, With<ReactorHud>>) {
    let Ok(mut text) = hud.single_mut() else {
        return;
    };
    let v = |key: &str| -> f64 {
        readouts
            .values
            .get(&format!("/ship/reactor/{key}"))
            .copied()
            .unwrap_or(0.0)
    };
    let status = readouts.status.map_or("---", |s| s.as_str());
    let mode = readouts.mode.map_or("---", |m| m.as_str());
    **text = format!(
        "REACTOR [{status}] {mode}\n\
         Temp:   {:.1} MK\n\
         Press:  {:.3}\n\
         Field:  {:.1} T\n\
         Fuel:   {:.1}%\n\
         Dens:   {:.1}%\n\
         Cont:   {:.0}%\n\
         Power:  {:.1} GW\n\
         Eff:    {:.0}%\n\
         Target: {:.0} GW",
        v("plasma/temperature"),
        v("plasma/pressure"),
        v("plasma/field_strength"),
        v("control/fuel_rate") * 100.0,
        v("plasma/density") * 100.0,
        v("plasma/containment") * 100.0,
        v("output/power"),
        v("output/efficiency") * 100.0,
        v("control/target_power"),
    );
}
