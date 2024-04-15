use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::*;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_obstacles)
            .add_systems(Update, (spin_obstacles, expand_obstacles));
    }
}

#[derive(Component)]
enum ObstacleVariant {
    Static,
    Spinning {
        clockwise: bool,
    },
    Expanding {
        speed: f32,
        max_scale: f32,
        min_scale: f32,
        expanding: bool,
        horizontal: bool,
    },
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

const OBSTACLES: [Obstacle; 6] = [
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
        ), // bottom right
        variant: ObstacleVariant::Static,
    },
    Obstacle {
        translation: Vec3::new(
            -WINDOW_WIDTH / 3.,
            (GROUND_HEIGHT / 4.) + GROUND_OFFSET.y,
            5.,
        ), // top left
        variant: ObstacleVariant::Spinning { clockwise: true },
    },
    Obstacle {
        translation: Vec3::new(
            WINDOW_WIDTH / 3.,
            (GROUND_HEIGHT / 4.) + GROUND_OFFSET.y,
            4.,
        ), // top right
        variant: ObstacleVariant::Spinning { clockwise: false },
    },
    Obstacle {
        translation: Vec3::new(0., (GROUND_HEIGHT / 4.) + GROUND_OFFSET.y, 4.), // bottom center
        variant: ObstacleVariant::Expanding {
            speed: 0.5,
            max_scale: 2.,
            min_scale: 1.,
            expanding: true,
            horizontal: true,
        },
    },
    Obstacle {
        translation: Vec3::new(0., (-GROUND_HEIGHT / 4.) + GROUND_OFFSET.y, 4.), // top center
        variant: ObstacleVariant::Expanding {
            speed: 0.5,
            max_scale: 2.,
            min_scale: 1.,
            expanding: true,
            horizontal: false,
        },
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
        if let ObstacleVariant::Spinning { clockwise } = variant {
            let switch = if *clockwise { 1. } else { -1. };
            position.rotation *=
                Quat::from_rotation_z(switch * time.delta_seconds() * 2. * std::f32::consts::PI);
        }
    }
}

fn expand_obstacles(
    mut commands: Commands,
    mut query: Query<(Entity, &ObstacleVariant, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, variant, mut position) in query.iter_mut() {
        if let ObstacleVariant::Expanding {
            speed,
            max_scale,
            min_scale,
            expanding,
            horizontal,
        } = variant
        {
            let mut now_expanding = *expanding;
            if *horizontal {
                if *expanding && position.scale.x <= *max_scale {
                    position.scale.x += time.delta_seconds() * speed;
                    if position.scale.x >= *max_scale {
                        now_expanding = false;
                    }
                } else if !expanding && position.scale.x >= *min_scale {
                    position.scale.x -= time.delta_seconds() * speed;
                    if position.scale.x <= *min_scale {
                        now_expanding = true;
                    }
                }
            } else {
                if *expanding && position.scale.y <= *max_scale {
                    position.scale.y += time.delta_seconds() * speed;
                    if position.scale.y >= *max_scale {
                        now_expanding = false;
                    }
                } else if !expanding && position.scale.y >= *min_scale {
                    position.scale.y -= time.delta_seconds() * speed;
                    if position.scale.y <= *min_scale {
                        now_expanding = true;
                    }
                }
            }
            let new_variant = ObstacleVariant::Expanding {
                speed: *speed,
                max_scale: *max_scale,
                min_scale: *min_scale,
                expanding: now_expanding,
                horizontal: *horizontal,
            };
            commands.entity(entity).insert(new_variant);
        }
    }
}
