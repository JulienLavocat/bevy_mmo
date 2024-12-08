use std::time::Duration;

use bevy::{
    app::{PanicHandlerPlugin, ScheduleRunnerPlugin},
    diagnostic::DiagnosticsPlugin,
    log::{Level, LogPlugin},
    prelude::*,
    render::mesh::MeshPlugin,
};
use bevy_remote_inspector::RemoteInspectorPlugins;
use server::ServerPlugin;

mod server;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
            // DefaultPlugins without feature-based plugins (minus the ones from MinimalPlugins)
            PanicHandlerPlugin,
            LogPlugin {
                level: Level::TRACE,
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
        .add_plugins(ServerPlugin)
        .run();
}
