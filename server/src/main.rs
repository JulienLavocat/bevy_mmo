use std::time::Duration;

use bevy::{
    app::{PanicHandlerPlugin, ScheduleRunnerPlugin},
    diagnostic::DiagnosticsPlugin,
    log::{Level, LogPlugin},
    prelude::*,
    render::mesh::MeshPlugin,
};
use bevy_remote_inspector::RemoteInspectorPlugins;
use server_net::NetworkPlugin;

mod server_net;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
            // DefaultPlugins without feature-based plugins (minus the ones from MinimalPlugins)
            PanicHandlerPlugin,
            LogPlugin {
                level: Level::DEBUG,
                filter: "wgpu=error,naga=warn,bevy=info,offset_allocator=info,winit=info"
                    .to_owned(),
                ..default()
            },
            TransformPlugin,
            HierarchyPlugin,
            DiagnosticsPlugin,
        ))
        // Plugins and assets to ensure compatibility with shared crate
        .add_plugins((AssetPlugin::default(), MeshPlugin))
        .init_asset::<StandardMaterial>()
        // Project's plugins
        .add_plugins(RemoteInspectorPlugins)
        // Server code
        .add_plugins(NetworkPlugin)
        .run();
}
