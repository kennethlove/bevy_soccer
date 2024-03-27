use bevy::prelude::*;
use crate::constants::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_sprite);
    }
}

fn setup_sprite(mut commands: Commands) {
    let translation = Vec3::new(0., 0., 1.);

    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(translation + GROUND_OFFSET),
        sprite: Sprite {
            color: Color::RED,
            custom_size: Vec2::new(50., 50.).into(),
            ..default()
        },
        ..default()
    });
}
