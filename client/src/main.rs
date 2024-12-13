use avian3d::{prelude::PhysicsDebugPlugin, PhysicsPlugins};
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

mod client_net;
mod player;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_remote_inspector::RemoteInspectorPlugins;
use client_net::NetworkPlugin;
use player::PlayerPlugin;
use shared::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::DEBUG,
                    filter: "wgpu=error,naga=warn,bevy=info,offset_allocator=info,winit=info"
                        .to_owned(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Centered(MonitorSelection::Current),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(RemoteInspectorPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(NetworkPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldPlugin)
        .run();
}
