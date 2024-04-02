use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

use crate::{
    arena::{GoalEvent, Wall},
    constants::*,
};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_ball.run_if(run_if_no_ball))
            .add_systems(FixedUpdate, (hit_walls, despawn_after_goal));
    }
}

fn run_if_no_ball(balls: Query<Entity, With<Ball>>) -> bool {
    balls.is_empty()
}

#[derive(Component)]
pub struct Ball;

const BALL_RADIUS: f32 = 10.;

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Mesh2dHandle(meshes.add(Circle {
        radius: BALL_RADIUS,
    }));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh,
            material: materials.add(Color::ORANGE),
            transform: Transform::from_translation(Vec3::new(0., GROUND_MIDDLE, 1.)),
            ..default()
        },
        Ball,
        RigidBody::Dynamic,
        AdditionalMassProperties::Mass(1.0),
        Collider::ball(BALL_RADIUS),
        Friction {
            coefficient: 0.2,
            combine_rule: CoefficientCombineRule::Average,
        },
        Restitution {
            coefficient: 1.,
            combine_rule: CoefficientCombineRule::Average,
        },
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
    ));
}

fn hit_walls(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    walls: Query<Entity, With<Wall>>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            let mut ball = commands.entity(*entity1);
            for wall in &walls {
                if wall == *entity2 {
                    ball.insert(ExternalImpulse {
                        impulse: Vec2::new(0., 0.),
                        torque_impulse: 0.,
                    });
                }
            }
        }
    }
}

fn despawn_after_goal(
    mut commands: Commands,
    mut goal_events: EventReader<GoalEvent>,
    balls: Query<Entity, With<Ball>>,
) {
    for _ in goal_events.read() {
        for ball in &balls {
            commands.entity(ball).despawn()
        }
    }
}
