use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::*;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_obstacles)
            .add_systems(Update, spin_obstacles);
    }
}

#[derive(Component)]
enum ObstacleVariant {
    Static,
    Spinning,
}

#[derive(Component)]
struct Obstacle {
    translation: Vec3,
    variant: ObstacleVariant,
}

impl Default for Obstacle {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            variant: ObstacleVariant::Static,
        }
    }
}

const OBSTACLE_SIZE: f32 = 40.;

const OBSTACLES: [Obstacle; 4] = [
    Obstacle {
        translation: Vec3::new(
            -WINDOW_WIDTH / 3.,
            (-GROUND_HEIGHT / 4.) + GROUND_OFFSET.y,
            5.,
        ), // bottom left
        variant: ObstacleVariant::Static,
    },
    Obstacle {
        translation: Vec3::new(
            WINDOW_WIDTH / 3.,
            (-GROUND_HEIGHT / 4.) + GROUND_OFFSET.y,
            5.,
        ), // bottom left
        variant: ObstacleVariant::Static,
    },
    Obstacle {
        translation: Vec3::new(
            -WINDOW_WIDTH / 3.,
            (GROUND_HEIGHT / 4.) + GROUND_OFFSET.y,
            5.,
        ), // top right
        variant: ObstacleVariant::Static,
    },
    Obstacle {
        translation: Vec3::new(
            WINDOW_WIDTH / 3.,
            (GROUND_HEIGHT / 4.) + GROUND_OFFSET.y,
            4.,
        ), // top right
        variant: ObstacleVariant::Spinning,
    },
];

fn spawn_obstacles(mut commands: Commands) {
    for obstacle in OBSTACLES {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(obstacle.translation),
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(OBSTACLE_SIZE, OBSTACLE_SIZE)),
                    ..default()
                },
                ..default()
            },
            Collider::cuboid(OBSTACLE_SIZE / 2., OBSTACLE_SIZE / 2.),
            RigidBody::Fixed,
            obstacle.variant,
            Restitution {
                coefficient: 1.,
                combine_rule: CoefficientCombineRule::Max,
            },
        ));
    }
}

fn spin_obstacles(mut query: Query<(&ObstacleVariant, &mut Transform)>, time: Res<Time>) {
    for (variant, mut position) in query.iter_mut() {
        if let ObstacleVariant::Spinning = variant {
            position.rotation *=
                Quat::from_rotation_z(time.delta_seconds() * 2. * std::f32::consts::PI);
        }
    }
}
