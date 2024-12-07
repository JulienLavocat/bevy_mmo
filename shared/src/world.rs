use avian3d::prelude::Collider;
use bevy::{
    color::palettes::css::{BLUE, DARK_GREEN},
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
    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::from(DARK_GREEN))),
        Collider::cuboid(50.0, 0.01, 50.0),
    ));

    commands.spawn((
        Name::new("Blue cube"),
        Transform::from_xyz(-2.0, 1.5, -2.0),
        Mesh3d(meshes.add(Cuboid::new(3.0, 3.0, 3.0))),
        MeshMaterial3d(materials.add(Color::from(BLUE))),
        Collider::cuboid(3.0, 3.0, 3.0),
    ));
}
