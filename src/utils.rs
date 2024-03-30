#![allow(dead_code)]
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn log_transition<T: States>(mut transitions: EventReader<StateTransitionEvent<T>>) {
    for transition in transitions.read() {
        info!(
            "transition: {:?} => {:?}",
            transition.before, transition.after
        );
    }
}

pub fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                info!("Collision started: {:?} and {:?}", entity1, entity2);
            }
            CollisionEvent::Stopped(entity1, entity2, _flags) => {
                info!("Collision stopped: {:?} and {:?}", entity1, entity2);
            }
        }
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
