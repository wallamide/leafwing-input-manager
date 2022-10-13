//! Demonstrates how to store (and use) per-action cooldowns
//!
//! This example shows off a tiny jump-move-and-shoot platformer!
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_platforms)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_blobs)
        .add_system(player_act)
        .add_system(player_gravity)
        .add_system(despawn_blob)
        .add_system(despawn_bullets)
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
        },
        ..default()
    });
}

#[derive(Default, Actionlike, Clone, Copy)]
enum PlayerAction {
    #[default]
    DoNothing,
    MoveLeft,
    MoveRight,
    Jump,
    Shoot,
}

#[derive(Component, Default)]
struct Platform;

#[derive(Bundle, Default)]
struct PlatformBundle {
    platform: Platform,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

const PLATFORM_SIZE: Vec2 = Vec2::new(300.0, 30.0);
const PLATFORM_GAP: Vec2 = Vec2::new(PLATFORM_SIZE.x + 50.0, 0.0);

pub fn spawn_platforms(mut commands: Commands) {
    commands.spawn_bundle(PlatformBundle {
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(PLATFORM_SIZE),
                ..default()
            },
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(PlatformBundle {
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(PLATFORM_SIZE),
                ..default()
            },
            transform: Transform::from_translation(PLATFORM_GAP.extend(0.0)),
            ..default()
        },
        ..default()
    });
}

#[derive(Component, Default)]
struct Player;

// Keeping things simple at the start, we only track the player's y-velocity.
// Later we can make this a Vec2.
#[derive(Component, Default)]
struct Velocity(f32);

#[derive(Bundle, Default)]
struct PlayerBundle {
    player: Player,
    #[bundle]
    sprite_bundle: SpriteBundle,
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
    velocity: Velocity,
}

const PLAYER_SIZE: Vec2 = Vec2::new(25.0, 25.0);

pub fn spawn_player(mut commands: Commands) {
    commands.spawn_bundle(PlayerBundle {
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GREEN,
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            transform: Transform::from_translation(
                Vec2::new(0.0, PLATFORM_SIZE.y / 2.0 + PLAYER_SIZE.y / 2.0).extend(0.0),
            ),
            ..default()
        },
        input_manager: InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (KeyCode::W, PlayerAction::Jump),
                (KeyCode::A, PlayerAction::MoveLeft),
                (KeyCode::D, PlayerAction::MoveRight),
                (KeyCode::S, PlayerAction::Shoot),
            ]),
        },
        ..default()
    });
}

fn player_act(
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    action_query: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    let action_state = action_query.single();

    if action_state.just_pressed(PlayerAction::Jump) {}

    if action_state.just_pressed(PlayerAction::Shoot) {
        // bullet spawning happens here
    }

    if action_state.pressed(PlayerAction::MoveLeft) {
        let mut player_transform = player_transform_query.single_mut();
        player_transform.translation.x -= 1.0;
    }

    if action_state.pressed(PlayerAction::MoveRight) {
        let mut player_transform = player_transform_query.single_mut();
        player_transform.translation.x += 1.0;
    }
}

fn player_gravity(mut player_transform_query: Query<&mut Transform, With<Player>>) {
    let mut player_transform = player_transform_query.single_mut();
}

#[derive(Component, Default)]
struct Bullet;

#[derive(Bundle, Default)]
struct BulletBundle {
    blob: Blob,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

const BULLET_SIZE: Vec2 = Vec2::new(3.0, 2.0);

fn despawn_bullets() {
    // bullets should be despawned after N seconds
}

#[derive(Component, Default)]
struct Blob;

#[derive(Bundle, Default)]
struct BlobBundle {
    blob: Blob,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

const BLOB_SIZE: Vec2 = Vec2::new(10.0, 35.0);

fn spawn_blobs(mut commands: Commands) {
    let blob_start_positions: [Vec2; 4] = [
        Vec2::new(40.0, PLATFORM_SIZE.y / 2.0 + BLOB_SIZE.y / 2.0),
        Vec2::new(80.0, PLATFORM_SIZE.y / 2.0 + BLOB_SIZE.y / 2.0),
        PLATFORM_GAP + Vec2::new(0.0, PLATFORM_SIZE.y / 2.0 + BLOB_SIZE.y / 2.0),
        PLATFORM_GAP + Vec2::new(40.0, PLATFORM_SIZE.y / 2.0 + BLOB_SIZE.y / 2.0),
    ];

    for blob_position in blob_start_positions {
        commands.spawn_bundle(BlobBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::CRIMSON,
                    custom_size: Some(BLOB_SIZE),
                    ..default()
                },
                transform: Transform::from_translation(blob_position.extend(0.0)),
                ..default()
            },
            ..default()
        });
    }
}

fn despawn_blob() {
    // blobs should be despawned once touched by a bullet
}
