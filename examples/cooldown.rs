//! Demonstrates how to store (and use) per-action cooldowns
//!
//! This example shows off a tiny jump-move-and-shoot platformer!
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb;
use leafwing_input_manager::prelude::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .insert_resource(PlayerTouchingPlatform::new(true))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_platforms)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_blobs)
        .add_system(update_player_contact)
        .add_system(player_act.after(update_player_contact))
        .add_system(player_move.after(player_act))
        .add_system(despawn_blob)
        .add_system(bullet_move)
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

#[derive(Actionlike, Clone, Copy)]
enum PlayerAction {
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
const PLATFORM_GAP: Vec2 = Vec2::new(PLATFORM_SIZE.x + 50.0, 50.0);

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

struct PlayerTouchingPlatform(bool);

impl PlayerTouchingPlatform {
    pub fn new(is_touching: bool) -> PlayerTouchingPlatform {
        PlayerTouchingPlatform(is_touching)
    }

    pub fn is_touching(&self) -> bool {
        self.0
    }

    pub fn set_touching(&mut self, is_touching: bool) {
        self.0 = is_touching;
    }
}

#[derive(Component, Default)]
struct Player;

#[derive(Component, Default)]
struct Velocity(Vec2);

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
const MAX_PLAYER_VELOCITY: Vec2 = Vec2::new(25.0, 25.0);

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
                (KeyCode::Space, PlayerAction::Shoot),
            ]),
        },
        ..default()
    });
}

pub const PLAYER_RUN_MAGNITUDE: f32 = 10.0;

fn player_act(
    mut commands: Commands,
    mut velocity_query: Query<(&Transform, &mut Velocity), With<Player>>,
    action_query: Query<&ActionState<PlayerAction>, With<Player>>,
    player_touching_platform: Res<PlayerTouchingPlatform>,
) {
    let action_state = action_query.single();
    let (player_transform, mut velocity) = velocity_query.single_mut();

    if action_state.just_pressed(PlayerAction::Jump) {
        velocity.0.y += 15.0;
    }

    if action_state.pressed(PlayerAction::Shoot) {
        commands.spawn_bundle(BulletBundle::new(player_transform.translation.xy()));
    }

    if action_state.pressed(PlayerAction::MoveLeft) && player_touching_platform.is_touching() {
        velocity.0.x -= PLAYER_RUN_MAGNITUDE;
    }

    if action_state.pressed(PlayerAction::MoveRight) && player_touching_platform.is_touching() {
        velocity.0.x += PLAYER_RUN_MAGNITUDE;
    }
}

fn update_player_contact(
    mut player_query: Query<(&mut Transform, &Sprite), With<Player>>,
    platform_query: Query<(&Transform, &Sprite), (With<Platform>, Without<Player>)>,
    mut player_touching_platform: ResMut<PlayerTouchingPlatform>,
) {
    let (mut player_transform, player_sprite) = player_query.single_mut();

    let player_size = player_sprite.custom_size.unwrap();
    let player_pos = player_transform.translation;

    let mut is_touching = false;
    for (platform_transform, platform_sprite) in platform_query.iter() {
        let platform_size = platform_sprite.custom_size.unwrap();
        if collide_aabb::collide(
            player_pos,
            player_size,
            platform_transform.translation,
            platform_size,
        )
        .is_some()
        {
            is_touching = true;
            player_transform.translation.y =
                platform_transform.translation.y + (platform_size.y / 2.0) + (player_size.y / 2.0);
            break;
        }
    }
    player_touching_platform.set_touching(is_touching);
}

pub const PLATFORM_FRICTION: f32 = 0.8;
pub const GRAVITY: f32 = 1.0;

fn player_move(
    mut player_query: Query<(&mut Velocity, &mut Transform), With<Player>>,
    player_touching_platform: Res<PlayerTouchingPlatform>,
) {
    let (mut player_velocity, mut player_transform) = player_query.single_mut();

    if player_touching_platform.is_touching() {
        player_velocity.0.x += -1.0 * player_velocity.0.x * PLATFORM_FRICTION;
        if player_velocity.0.y < 0.0 {
            player_velocity.0.y = 0.0;
        }
    } else {
        player_velocity.0.y -= GRAVITY;
    }

    player_transform.translation += player_velocity.0.extend(0.0);
}

#[derive(Component, Default)]
struct Bullet;
#[derive(Component)]
struct BulletTimer(Timer);

#[derive(Bundle)]
struct BulletBundle {
    bullet: Bullet,
    #[bundle]
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    timer: BulletTimer,
}

const BULLET_VELOCITY: Vec2 = Vec2::new(15.0, 0.0);

impl BulletBundle {
    pub fn new(start_position: Vec2) -> BulletBundle {
        let timer = Timer::new(Duration::from_millis(500), false);
        BulletBundle {
            bullet: Bullet,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(BULLET_SIZE),
                    ..default()
                },
                transform: Transform::from_translation(start_position.extend(0.0)),
                ..default()
            },
            velocity: Velocity(BULLET_VELOCITY),
            timer: BulletTimer(timer),
        }
    }
}

const BULLET_SIZE: Vec2 = Vec2::new(3.0, 2.0);

fn bullet_move(mut bullet_query: Query<(&mut Velocity, &mut Transform), With<Bullet>>) {
    for (v, mut t) in bullet_query.iter_mut() {
        t.translation += v.0.extend(0.0);
    }
}

fn despawn_bullets(
    mut commands: Commands,
    bullet_query: Query<(Entity, &BulletTimer), With<Bullet>>,
) {
    for (e, t) in bullet_query.iter() {
        if t.0.finished() {
            commands.entity(e).despawn();
        }
    }
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
