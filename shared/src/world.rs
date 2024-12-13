use avian3d::prelude::{Collider, RigidBody};
use bevy::{
    color::palettes::css::{BLUE, DARK_GREEN, ORANGE, RED},
    prelude::*,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world)
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 2000.0,
            });
    }
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("spawning world");

    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::from(DARK_GREEN))),
        RigidBody::Static,
        Collider::cuboid(50.0, 0.1, 50.0),
    ));

    commands.spawn((
        Name::new("Blue cube"),
        Transform::from_xyz(-2.0, 1.5, -2.0),
        Mesh3d(meshes.add(Cuboid::new(3.0, 3.0, 3.0))),
        MeshMaterial3d(materials.add(Color::from(BLUE))),
        Collider::cuboid(3.0, 3.0, 3.0),
        RigidBody::Static,
    ));

    commands.spawn((
        Name::new("Red cube"),
        Transform::from_xyz(2.0, 0.5, 2.0),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(RED))),
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Static,
    ));

    commands.spawn((
        Name::new("Red cube"),
        Transform::from_xyz(2.0, 0.5, 2.0),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(RED))),
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Static,
    ));

    commands.spawn((
        Name::new("Orange cube"),
        Transform::from_xyz(-2.0, 0.9, 4.0),
        Mesh3d(meshes.add(Cuboid::new(1.8, 1.8, 1.8))),
        MeshMaterial3d(materials.add(Color::from(ORANGE))),
        Collider::cuboid(1.8, 1.8, 1.8),
        RigidBody::Static,
    ));
}
