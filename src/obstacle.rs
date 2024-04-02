use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::*;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_obstacle);
    }
}

#[derive(Component)]
struct Obstacle;

const OBSTACLE_SIZE: f32 = 40.;

const OBSTACLES: [Vec3; 2] = [
    Vec3::new(-WINDOW_WIDTH / 4., GROUND_MIDDLE, 5.), // middle left
    Vec3::new(WINDOW_WIDTH / 4., GROUND_MIDDLE, 5.),  // middle right
];

fn spawn_obstacle(mut commands: Commands) {
    for position in OBSTACLES {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(position),
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(OBSTACLE_SIZE, OBSTACLE_SIZE)),
                    ..default()
                },
                ..default()
            },
            Collider::cuboid(OBSTACLE_SIZE / 2., OBSTACLE_SIZE / 2.),
            RigidBody::Fixed,
            Obstacle,
        ));
    }
}
