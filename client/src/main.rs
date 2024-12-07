use avian3d::{
    prelude::{PhysicsDebugPlugin, PhysicsSchedule},
    PhysicsPlugins,
};
use bevy::prelude::*;

mod player;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_remote_inspector::RemoteInspectorPlugins;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use player::PlayerPlugin;
use shared::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(RemoteInspectorPlugins)
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
        .add_plugins((
            TnuaControllerPlugin::new(PhysicsSchedule),
            TnuaAvian3dPlugin::new(PhysicsSchedule),
        ))
        .add_plugins((PlayerPlugin, WorldPlugin))
        .run();
}
