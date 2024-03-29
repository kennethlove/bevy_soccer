use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::constants::*;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (setup_ground, setup_walls));
    }
}

#[derive(Component)]
pub struct Ground;

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
        Ground,
        RigidBody::Fixed,
    ));
}

#[derive(Component)]
pub struct Wall;

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
            Wall,
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
            Wall,
            RigidBody::Fixed,
            Collider::cuboid(WINDOW_WIDTH / 2., 1.)
        ));
    }
}
