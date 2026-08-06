mod components;
mod resources;
mod input;

use bevy::camera::ScalingMode;
use bevy::color::palettes::basic::SILVER;
use bevy::prelude::*;
use crate::components::Player;
use crate::input::camera_input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(resources::CameraController::default())
        .insert_resource(resources::Keybinds::default())
        .add_systems(Startup, (startup))
        .add_systems(Update, (camera_input))
        .run();
}

fn startup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        Camera3d::default(),
        Player,
        Projection::from(OrthographicProjection {
            // 6 world units per pixel of window height.
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 6.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(30.0, 40.0, 30.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));
    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));
    commands.spawn((
        Transform::from_xyz(0.0, 2.0, 0.0),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial{
            base_color: Color::srgb(255.0,0.0,0.0),
            ..default()
        }))
        ));
}