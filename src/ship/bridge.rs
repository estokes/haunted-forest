use bevy::prelude::*;

#[derive(Component)]
pub struct Console;

#[derive(Component)]
pub struct BridgeViewport;

pub fn spawn_bridge(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let width = 12.0_f32;
    let depth = 8.0_f32;
    let height = 3.5_f32;
    let wall = 0.2_f32;

    let floor_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.15, 0.18),
        metallic: 0.8,
        perceptual_roughness: 0.3,
        ..default()
    });
    let wall_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.1, 0.12),
        metallic: 0.5,
        perceptual_roughness: 0.6,
        ..default()
    });
    let console_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.05, 0.05, 0.08),
        emissive: LinearRgba::new(0.0, 0.8, 1.2, 1.0),
        ..default()
    });
    let viewport_mat = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        emissive: LinearRgba::new(0.005, 0.005, 0.02, 1.0),
        unlit: true,
        ..default()
    });

    // floor
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(width, wall, depth))),
        MeshMaterial3d(floor_mat),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // ceiling
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(width, wall, depth))),
        MeshMaterial3d(wall_mat.clone()),
        Transform::from_xyz(0.0, height, 0.0),
    ));

    // back wall (behind the player)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(width, height, wall))),
        MeshMaterial3d(wall_mat.clone()),
        Transform::from_xyz(0.0, height / 2.0, depth / 2.0),
    ));

    // front wall (viewport — player faces this)
    commands.spawn((
        BridgeViewport,
        Mesh3d(meshes.add(Cuboid::new(width, height, wall))),
        MeshMaterial3d(viewport_mat),
        Transform::from_xyz(0.0, height / 2.0, -depth / 2.0),
    ));

    // left wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(wall, height, depth))),
        MeshMaterial3d(wall_mat.clone()),
        Transform::from_xyz(-width / 2.0, height / 2.0, 0.0),
    ));

    // right wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(wall, height, depth))),
        MeshMaterial3d(wall_mat),
        Transform::from_xyz(width / 2.0, height / 2.0, 0.0),
    ));

    // helm console (front center, the main interactable)
    commands.spawn((
        Console,
        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 0.8))),
        MeshMaterial3d(console_mat.clone()),
        Transform::from_xyz(0.0, 0.5, -2.0),
    ));

    // navigation console (left)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.5, 1.0, 0.8))),
        MeshMaterial3d(console_mat.clone()),
        Transform {
            translation: Vec3::new(-3.5, 0.5, -1.0),
            rotation: Quat::from_rotation_y(0.3),
            ..default()
        },
    ));

    // engineering console (right)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.5, 1.0, 0.8))),
        MeshMaterial3d(console_mat),
        Transform {
            translation: Vec3::new(3.5, 0.5, -1.0),
            rotation: Quat::from_rotation_y(-0.3),
            ..default()
        },
    ));

    // main overhead light — cool white
    commands.spawn((
        PointLight {
            intensity: 100_000.0,
            shadows_enabled: true,
            color: Color::srgb(0.8, 0.85, 1.0),
            ..default()
        },
        Transform::from_xyz(0.0, 3.2, 0.0),
    ));

    // accent light over helm — blue tint
    commands.spawn((
        PointLight {
            intensity: 30_000.0,
            color: Color::srgb(0.2, 0.5, 0.9),
            ..default()
        },
        Transform::from_xyz(0.0, 2.5, -2.5),
    ));

    // fill light near back wall
    commands.spawn((
        PointLight {
            intensity: 20_000.0,
            color: Color::srgb(0.6, 0.6, 0.7),
            ..default()
        },
        Transform::from_xyz(0.0, 2.5, 3.0),
    ));
}
