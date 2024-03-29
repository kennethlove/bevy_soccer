use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_editor_pls::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::*;

use bevy_soccer::animation::animate_sprites;
use bevy_soccer::arena::ArenaPlugin;
use bevy_soccer::ball::BallPlugin;
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
        .add_plugins(EditorPlugin::default())
        .add_plugins((ArenaPlugin, PlayerPlugin, BallPlugin))
        .add_systems(Startup, setup_camera)
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
