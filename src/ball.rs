use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

use crate::{arena::Wall, constants::GROUND_OFFSET, player::Player};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(FixedUpdate, (get_kicked, hit_walls));
    }
}

#[derive(Component)]
pub struct Ball;

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Mesh2dHandle(meshes.add(Circle { radius: 10. }));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh,
            material: materials.add(Color::ORANGE),
            transform: Transform::from_translation(Vec3::new(0., -GROUND_OFFSET.y, 1.)),
            ..default()
        },
        Ball,
        RigidBody::Dynamic,
        AdditionalMassProperties::Mass(1.0),
        Collider::ball(10.),
        Restitution {
            coefficient: 0.7,
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

fn get_kicked(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    player: Query<Entity, With<Player>>,
) {
    let player = player.single();
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, _entity2, _flags) = collision_event {
            if player == *_entity2 {
                let mut ball = commands.entity(*entity1);
                ball.insert(ExternalImpulse {
                    impulse: Vec2::new(10., 20.),
                    torque_impulse: 1.,
                });
            }
        }
    }
}
