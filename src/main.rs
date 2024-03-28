use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_kira_audio::prelude::*;
use bevy_pkv::PkvStore;
use bevy_rapier2d::{prelude::*, rapier::{dynamics::{RigidBodyBuilder, RigidBodySet}, geometry::{ColliderBuilder, ColliderSet}}};

use bevy_soccer::animation::animate_sprites;
use bevy_soccer::constants::*;
use bevy_soccer::player::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never) // Makes WASM happy
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(PkvStore::new("kennethlove", "soccer-game"))
        .add_plugins(EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault
        })
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Soccer Game".to_string(),
                    resolution:(WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()), // keeps pixel art crisp
        )
        .add_plugins(AudioPlugin) // Kira audio
        .add_plugins(TilemapPlugin) // ECS tilemap
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, (setup_camera, setup_ground, setup_walls))
        .add_systems(Update, animate_sprites)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
        ..Default::default()
    });
}

fn setup_ground(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(GROUND_OFFSET),
            sprite: Sprite {
                color: Color::BEIGE,
                custom_size: Some(Vec2::new(GROUND_SIZE_WIDTH, GROUND_SIZE_HEIGHT)),
                ..default()
            },
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(GROUND_SIZE_WIDTH / 2., GROUND_SIZE_HEIGHT / 2.)
    ));
}

fn setup_walls(
    mut commands: Commands,
) {
    // Vertical walls
    for (x, y) in [
        (WINDOW_WIDTH/2., 0.),
        (-WINDOW_WIDTH/2., 0.),
    ] {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(2., WINDOW_HEIGHT)),
                    ..default()
                },
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(1., WINDOW_HEIGHT / 2.)
        ));
    }

    // Horizontal walls
    for (x, y) in [
        (0., WINDOW_HEIGHT/2.),
        (0., (-WINDOW_HEIGHT/2.) + GROUND_OFFSET.y * 2.),
    ] {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(WINDOW_WIDTH, 2.)),
                    ..default()
                },
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(WINDOW_WIDTH / 2., 1.)
        ));
    }
}
