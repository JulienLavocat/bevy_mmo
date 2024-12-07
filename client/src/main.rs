use avian3d::{prelude::PhysicsDebugPlugin, PhysicsPlugins};
use bevy::prelude::*;

mod player;

use bevy_remote_inspector::RemoteInspectorPlugins;
use player::PlayerPlugin;
use shared::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RemoteInspectorPlugins)
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
        .add_plugins((PlayerPlugin, WorldPlugin))
        .run();
}
