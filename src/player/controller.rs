use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use std::f32::consts::FRAC_PI_2;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Resource)]
pub struct PlayerSettings {
    pub sensitivity: f32,
    pub move_speed: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.003,
            move_speed: 5.0,
        }
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn((Player, Transform::from_xyz(0.0, 1.0, 4.0), Visibility::default()))
        .with_children(|parent| {
            parent.spawn((
                PlayerCamera,
                Camera3d::default(),
                Transform::from_xyz(0.0, 0.7, 0.0),
            ));
        });
}

pub fn grab_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

pub fn toggle_cursor(keys: Res<ButtonInput<KeyCode>>, mut cursor: Single<&mut CursorOptions>) {
    if keys.just_pressed(KeyCode::Escape) {
        let locked = cursor.grab_mode == CursorGrabMode::Locked;
        if locked {
            cursor.grab_mode = CursorGrabMode::None;
            cursor.visible = true;
        } else {
            cursor.grab_mode = CursorGrabMode::Locked;
            cursor.visible = false;
        }
    }
}

pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    settings: Res<PlayerSettings>,
) {
    let forward = *player.forward();
    let right = *player.right();
    let forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();
    let mut direction = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        direction += forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction -= forward;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += right;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction -= right;
    }
    direction = direction.normalize_or_zero();
    player.translation += direction * settings.move_speed * time.delta_secs();
}

pub fn player_look(
    mouse: Res<AccumulatedMouseMotion>,
    mut player: Query<&mut Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    settings: Res<PlayerSettings>,
    cursor: Query<&CursorOptions>,
) {
    let Ok(opts) = cursor.single() else {
        return;
    };
    if opts.grab_mode != CursorGrabMode::Locked {
        return;
    }
    let delta = mouse.delta;
    if delta == Vec2::ZERO {
        return;
    }
    let Ok(mut player_transform) = player.single_mut() else {
        return;
    };
    let Ok(mut camera_transform) = camera.single_mut() else {
        return;
    };
    // yaw on the player entity
    player_transform.rotate_y(-delta.x * settings.sensitivity);
    // pitch on the camera, clamped to avoid gimbal flip
    let pitch_limit = FRAC_PI_2 - 0.01;
    let (_, pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);
    let new_pitch = (pitch - delta.y * settings.sensitivity).clamp(-pitch_limit, pitch_limit);
    camera_transform.rotation = Quat::from_rotation_x(new_pitch);
}
