use crate::constants::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_ground, setup_goals, setup_walls))
            .add_systems(Update, hit_goal);
    }
}

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Goal;

const HOLE_SIZE: f32 = 80.;
const HOLE_POSITIONS: [Vec3; 4] = [
    Vec3::new(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2., 1.), // top right
    Vec3::new(-WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2., 1.), // top left
    Vec3::new(
        WINDOW_WIDTH / 2.,
        (-WINDOW_HEIGHT / 2.) + GROUND_OFFSET.y * 2.,
        1.,
    ), // bottom right
    Vec3::new(
        -WINDOW_WIDTH / 2.,
        (-WINDOW_HEIGHT / 2.) + GROUND_OFFSET.y * 2.,
        1.,
    ), // bottom left
];

fn setup_goals(mut commands: Commands) {
    for position in HOLE_POSITIONS.iter() {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(*position),
                sprite: Sprite {
                    color: Color::PINK,
                    custom_size: Some(Vec2::new(HOLE_SIZE, HOLE_SIZE)),
                    ..default()
                },
                ..default()
            },
            Goal,
            RigidBody::Fixed,
            Collider::ball(HOLE_SIZE / 2.),
            Sensor,
        ));
    }
}

fn hit_goal(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    goals: Query<Entity, With<Goal>>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            // let ball = commands.entity(*entity1);
            for goal in &goals {
                if goal == *entity2 {
                    info!("Ball went in the hole!");
                }
            }
        }
    }
}

fn setup_ground(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
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

fn setup_walls(mut commands: Commands) {
    // Vertical walls
    for (x, y) in [(WINDOW_WIDTH / 2., 0.), (-WINDOW_WIDTH / 2., 0.)] {
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
            Collider::cuboid(1., WINDOW_HEIGHT / 2.),
        ));
    }

    // Horizontal walls
    for (x, y) in [
        (0., WINDOW_HEIGHT / 2.),
        (0., (-WINDOW_HEIGHT / 2.) + GROUND_OFFSET.y * 2.),
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
            Collider::cuboid(WINDOW_WIDTH / 2., 1.),
        ));
    }
}
