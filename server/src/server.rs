use bevy::prelude::*;
use shared::world::WorldPlugin;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldPlugin)
            .add_systems(Startup, hello_world)
            .add_systems(Update, on_update);
    }
}

fn hello_world(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(Mesh3d(meshes.add(Capsule3d::default())));
    info!("Hello world");
}

fn on_update() {}
