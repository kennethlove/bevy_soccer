use crate::{animation::FlashingTimer, constants::*};
use bevy::{app::AppExit, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::*;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoalEvent>()
            .add_systems(Startup, (setup_ground, setup_goals, setup_walls))
            .add_systems(Update, (touch_goal, score_goal, update_high_score))
            .add_systems(Last, clear_score);
    }
}

#[derive(Event, Debug)]
pub struct GoalEvent {
    pub score_amount: i32,
    pub goal: Entity,
}

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Goal;

const GOAL_HEIGHT: f32 = 160.;
const GOAL_WIDTH: f32 = 20.;

const GOAL_POSITIONS: [Vec3; 2] = [
    Vec3::new(-WINDOW_WIDTH / 2., GROUND_MIDDLE, 1.), // middle left
    Vec3::new(WINDOW_WIDTH / 2., GROUND_MIDDLE, 1.),  // middle right
];

fn setup_goals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for position in GOAL_POSITIONS.iter() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(Rectangle {
                        half_size: Vec2::new(GOAL_WIDTH / 2., GOAL_HEIGHT / 2.), // (width, height)
                    })
                    .into(),
                transform: Transform::from_translation(*position),
                material: materials.add(Color::PINK),
                ..default()
            },
            Goal,
            RigidBody::Fixed,
            Collider::cuboid(GOAL_WIDTH / 2., GOAL_HEIGHT / 2.),
            Sensor,
        ));
    }
}

fn touch_goal(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    goals: Query<Entity, With<Goal>>,
    mut event_writer: EventWriter<GoalEvent>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            for goal in &goals {
                if goal == *entity1 {
                    let mut entity = commands.entity(*entity1);
                    entity.insert(FlashingTimer(Timer::from_seconds(
                        0.1,
                        TimerMode::Repeating,
                    )));
                    event_writer.send(GoalEvent {
                        score_amount: 1,
                        goal,
                    });
                }
            }
        }
    }
}

fn score_goal(mut goal_events: EventReader<GoalEvent>, mut pkv: ResMut<PkvStore>) {
    for goal_event in goal_events.read() {
        if let Ok(mut score) = pkv.get::<i32>("score") {
            score += goal_event.score_amount;
            pkv.set("score", &score).expect("Failed to set score");
        } else {
            let score = goal_event.score_amount;
            pkv.set("score", &score).expect("Failed to set score");
        }
    }
}

fn update_high_score(mut pkv: ResMut<PkvStore>) {
    let score = pkv.get::<i32>("score").unwrap_or(0);
    let high_score = pkv.get::<i32>("high_score").unwrap_or(0);

    if score > high_score {
        pkv.set("high_score", &score)
            .expect("Failed to set high score");
    }
}

fn setup_ground(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(GROUND_OFFSET),
            sprite: Sprite {
                color: Color::BEIGE,
                custom_size: Some(Vec2::new(GROUND_WIDTH, GROUND_HEIGHT)),
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
    for (x, y) in [
        (WINDOW_WIDTH / 2., GROUND_MIDDLE),
        (-WINDOW_WIDTH / 2., GROUND_MIDDLE),
    ] {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(2., GROUND_HEIGHT)),
                    ..default()
                },
                ..default()
            },
            Wall,
            RigidBody::Fixed,
            Collider::cuboid(1., GROUND_HEIGHT / 2.),
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

fn clear_score(mut pkv: ResMut<PkvStore>, mut events: EventReader<AppExit>) {
    for _ in events.read() {
        pkv.set("score", &0).expect("Failed to set score");
    }
}
