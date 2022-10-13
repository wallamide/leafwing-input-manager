//! Demonstrates how to store (and use) per-action cooldowns
//!
//! This example shows off a tiny jump-move-and-shoot platformer!
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<PlatformerActions>::default())
        .add_startup_system(spawn_platforms)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_blobs)
        .add_system(player_move)
        .run();
}

#[derive(Default, Actionlike, Clone, Copy)]
enum PlatformerActions {
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

pub fn spawn_platforms(mut commands: Commands) {
    commands.spawn().insert_bundle(PlatformBundle {
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(25.0, 100.0)),
                ..default()
            },
            ..default()
        },
        ..default()
    });
}

pub fn spawn_player() {}

pub fn spawn_blobs() {}

pub fn player_move() {}
