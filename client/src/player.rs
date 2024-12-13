use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use avian3d::prelude::*;
use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use shared::server_packets::SpawnPlayer;
use tiny_bail::or_return_quiet;

use bevy_tnua::{
    builtins::{TnuaBuiltinCrouch, TnuaBuiltinCrouchState, TnuaBuiltinDash},
    control_helpers::{TnuaCrouchEnforcerPlugin, TnuaSimpleAirActionsCounter},
    math::{Float, Quaternion, Vector3},
    prelude::*,
    TnuaToggle,
};
use bevy_tnua_avian3d::*;

const CAMERA_SENSITIVITY: f32 = 0.01;
const CAMERA_HEIGHT: f32 = 1.0;

const PLAYER_MODEL_PATH: &str = "player/player.glb";
const PLAYER_SPEED: f32 = 10.0;
const PLAYER_FLOAT_HEIGHT: f32 = 1.0;
const PLAYER_JUMP_HEIGHT: f32 = 2.0;
const PLAYER_WALK_SPEED_FACTOR: f32 = 1.0;
const PLAYER_CROUCH_SPEED_FACTOR: f32 = 1.0;
const PLAYER_ACTIONS_IN_AIR: usize = 1;
const PLAYER_DASH_DISTANCE: f32 = 10.0;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Component)]
pub struct ForwardFromCamera {
    pub forward: Vector3,
    pub pitch_angle: Float,
}

impl Default for ForwardFromCamera {
    fn default() -> Self {
        Self {
            forward: Vector3::NEG_Z,
            pitch_angle: 0.0,
        }
    }
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TnuaAvian3dPlugin::default())
            .add_plugins(TnuaControllerPlugin::default())
            .add_plugins(TnuaCrouchEnforcerPlugin::default())
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, spawn_player)
            .add_systems(Update, grab_ungrab_mouse)
            .add_systems(
                Update,
                apply_movement_controls.in_set(TnuaUserControlsSystemSet),
            )
            .add_systems(
                PostUpdate,
                apply_camera_controls
                    .before(TransformSystem::TransformPropagate)
                    .after(PhysicsSet::Sync),
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("PlayerCamera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 16.0, 40.0).looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
    ));
}

fn spawn_player(
    mut events: EventReader<SpawnPlayer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        info!("spawning player at: {}", event.location);

        commands
            .spawn((
                Name::new("Player"),
                Player,
                InheritedVisibility::default(), // Remove a warning at runtime
                Transform::from_translation(event.location),
                Collider::capsule(0.3, 0.5),
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
                ForwardFromCamera::default(),
                TnuaController::default(),
                TnuaToggle::Enabled,
                TnuaAvian3dSensorShape(Collider::cylinder(0.2, 0.0)),
                TnuaSimpleAirActionsCounter::default(),
            ))
            .with_child((
                SceneRoot(
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset(PLAYER_MODEL_PATH)),
                ),
                Transform::default()
                    .with_scale(Vec3::new(0.01, 0.01, 0.01))
                    .with_translation(Vec3::new(0.0, -1.0, 0.0)),
            ));
    }
}

fn apply_movement_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut q: Query<
        (
            &mut TnuaController,
            &mut TnuaSimpleAirActionsCounter,
            &ForwardFromCamera,
        ),
        With<Player>,
    >,
) {
    let (mut controller, mut air_actions_counter, forward_from_camera) =
        or_return_quiet!(q.get_single_mut());

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction -= Vec3::Z;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction += Vec3::Z;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction -= Vec3::X;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += Vec3::X;
    }

    direction = Transform::default()
        .looking_to(forward_from_camera.forward, Vec3::Y)
        .transform_point(direction);

    let jump = keyboard.pressed(KeyCode::Space);
    let dash = keyboard.pressed(KeyCode::ShiftLeft);

    let speed_factor = if let Some((_, state)) = controller.concrete_action::<TnuaBuiltinCrouch>() {
        if matches!(state, TnuaBuiltinCrouchState::Rising) {
            PLAYER_WALK_SPEED_FACTOR
        } else {
            PLAYER_CROUCH_SPEED_FACTOR
        }
    } else {
        PLAYER_WALK_SPEED_FACTOR
    };

    air_actions_counter.update(controller.as_mut());

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: direction.normalize_or_zero() * speed_factor * PLAYER_SPEED,
        desired_forward: Dir3::new(forward_from_camera.forward).ok(),
        float_height: PLAYER_FLOAT_HEIGHT,
        max_slope: FRAC_PI_4,
        ..default()
    });

    if jump {
        controller.action(TnuaBuiltinJump {
            height: PLAYER_JUMP_HEIGHT,
            allow_in_air: air_actions_counter.air_count_for(TnuaBuiltinJump::NAME)
                <= PLAYER_ACTIONS_IN_AIR,
            ..Default::default()
        });
    }

    if dash {
        controller.action(TnuaBuiltinDash {
            displacement: direction.normalize_or_zero() * PLAYER_DASH_DISTANCE,
            desired_forward: None,
            allow_in_air: air_actions_counter.air_count_for(TnuaBuiltinDash::NAME)
                <= PLAYER_ACTIONS_IN_AIR,
            ..default()
        });
    }
}

fn grab_ungrab_mouse(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = or_return_quiet!(primary_window_query.get_single_mut());

    if window.cursor_options.visible {
        if mouse_buttons.just_pressed(MouseButton::Left) {
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
        }
    } else if keyboard.just_released(KeyCode::Escape)
        || mouse_buttons.just_pressed(MouseButton::Left)
    {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
    }
}

fn apply_camera_controls(
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_character_query: Query<(&GlobalTransform, &mut ForwardFromCamera)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let mouse_controls_camera = primary_window_query
        .get_single()
        .map_or(false, |w| !w.cursor_options.visible);

    if !mouse_controls_camera {
        return;
    }

    let (player_transform, mut forward_from_camera) =
        or_return_quiet!(player_character_query.get_single_mut());

    let yaw = Quaternion::from_rotation_y(-CAMERA_SENSITIVITY * accumulated_mouse_motion.delta.x);
    forward_from_camera.forward = yaw.mul_vec3(forward_from_camera.forward);

    let pitch = 0.005 * accumulated_mouse_motion.delta.y;
    forward_from_camera.pitch_angle =
        (forward_from_camera.pitch_angle + pitch).clamp(-FRAC_PI_2, FRAC_PI_2);

    for mut camera in camera_query.iter_mut() {
        camera.translation = player_transform.translation()
            + -10.0 * forward_from_camera.forward
            + CAMERA_HEIGHT * Vec3::Y;
        camera.look_to(forward_from_camera.forward, Vec3::Y);
        let pitch_axis = camera.left();
        camera.rotate_around(
            player_transform.translation(),
            Quat::from_axis_angle(*pitch_axis, forward_from_camera.pitch_angle),
        );
    }
}
