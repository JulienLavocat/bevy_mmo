use std::{f32::consts::FRAC_PI_2, time::Duration};

use avian3d::prelude::Collider;
use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use tiny_bail::{or_return, or_return_quiet};

pub struct PlayerPlugin;

const PLAYER_MODEL_PATH: &str = "player/player.glb";
const IDLE_ANIMATION: usize = 0;
const CAMERA_SPEED: f32 = 10.0;
const CAMERA_SENSITIVITY: Vec2 = Vec2::new(0.003, 0.002);
const CAMERA_PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Resource)]
pub struct PlayerAnimations {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, play_animations)
            .add_systems(Update, (rotate_player, move_player).chain());
    }
}

fn play_animations(
    mut commands: Commands,
    animations: Res<PlayerAnimations>,
    mut q: Query<(&mut AnimationPlayer, Entity), Added<AnimationPlayer>>,
) {
    for (mut player, entity) in &mut q {
        let mut transitions = AnimationTransitions::new();

        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(transitions);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    info!("spawning player");

    let idle_animation =
        asset_server.load(GltfAssetLabel::Animation(IDLE_ANIMATION).from_asset(PLAYER_MODEL_PATH));
    let (graph, animations_indices) = AnimationGraph::from_clips([idle_animation]);

    let graph_handle = graphs.add(graph);
    commands.insert_resource(PlayerAnimations {
        animations: animations_indices,
        graph: graph_handle,
    });

    commands.spawn((
        Name::new("PlayerCamera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    commands
        .spawn((
            Name::new("Player"),
            Player,
            Transform::from_xyz(0.0, 0.0, 0.0),
            Collider::capsule_endpoints(0.3, Vec3::new(0.0, 0.3, 0.0), Vec3::new(0.0, 1.5, 0.0)),
            InheritedVisibility::default(), // Remove a warning at runtime
        ))
        .with_child((
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(PLAYER_MODEL_PATH))),
            Transform::default().with_scale(Vec3::new(0.01, 0.01, 0.01)),
        ));
}

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut q_controllers: Query<&mut Transform, With<Camera3d>>,
) {
    let mut transform = or_return!(q_controllers.get_single_mut());

    let mut horizontal = 0.0;
    let mut vertical = 0.0;

    if input.pressed(KeyCode::KeyW) {
        vertical -= CAMERA_SPEED;
    }
    if input.pressed(KeyCode::KeyS) {
        vertical += CAMERA_SPEED;
    }

    if input.pressed(KeyCode::KeyA) {
        horizontal -= CAMERA_SPEED;
    }
    if input.pressed(KeyCode::KeyD) {
        horizontal += CAMERA_SPEED;
    }

    let towards = transform.rotation;
    let mut translation =
        towards * Vec3::new(horizontal, 0.0, vertical).normalize_or_zero() * time.delta_secs();

    if input.pressed(KeyCode::ShiftLeft) {
        translation *= 4.0
    }

    transform.translation += translation
}

fn rotate_player(
    mut q_transform: Query<&mut Transform, With<Camera3d>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
) {
    // For now, camera free move
    let mut transform = or_return_quiet!(q_transform.get_single_mut());
    let mut primary_window = or_return!(q_windows.get_single_mut());

    if mouse_input.just_pressed(MouseButton::Right) {
        primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor_options.visible = false;
    }

    if mouse_input.just_released(MouseButton::Right) {
        primary_window.cursor_options.grab_mode = CursorGrabMode::None;
        primary_window.cursor_options.visible = true;
    }

    // Camera rotation, when right clicking
    if mouse_input.pressed(MouseButton::Right) && accumulated_mouse_motion.delta != Vec2::ZERO {
        let delta = -accumulated_mouse_motion.delta * CAMERA_SENSITIVITY;
        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta.x;
        let pitch = (pitch + delta.y).clamp(-CAMERA_PITCH_LIMIT, CAMERA_PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}
