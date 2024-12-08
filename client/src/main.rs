use avian3d::{
    prelude::{PhysicsDebugPlugin, PhysicsSchedule},
    PhysicsPlugins,
};
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

mod client_net;
mod player;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_remote_inspector::RemoteInspectorPlugins;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use client_net::NetworkPlugin;
use player::PlayerPlugin;
use shared::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::DEBUG,
            filter: "wgpu=error,naga=warn,bevy=info,offset_allocator=info,winit=info".to_owned(),
            ..Default::default()
        }))
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(RemoteInspectorPlugins)
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
        .add_plugins((
            TnuaControllerPlugin::new(PhysicsSchedule),
            TnuaAvian3dPlugin::new(PhysicsSchedule),
        ))
        .add_plugins(NetworkPlugin)
        .add_plugins((PlayerPlugin, WorldPlugin))
        .run();
}
