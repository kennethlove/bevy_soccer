use bevy::prelude::*;
use leafwing_input_manager::{input_map, prelude::*};
use crate::constants::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(Startup, setup_sprite)
            .add_systems(Update, use_actions);
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    Walk,
    Run
}

impl Action {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // default game pad input bindings
        input_map.insert(Self::Walk, DualAxis::left_stick());

        // default keyboard input bindings
        input_map.insert(Self::Walk, VirtualDPad::wasd());

        input_map
    }
}

#[derive(Component)]
struct Player;

fn setup_sprite(mut commands: Commands) {
    let translation = Vec3::new(0., 0., 1.);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(translation + GROUND_OFFSET),
            sprite: Sprite {
                color: Color::RED,
                custom_size: Vec2::new(50., 50.).into(),
                ..default()
            },
            ..default()
        },
        InputManagerBundle::with_map(Action::default_input_map()),
        Player,
    ));
}

fn use_actions(
    mut query: Query<(&ActionState<Action>, &mut Transform), With<Player>>,
) {
    let (action_state, mut transform) = query.single_mut();

    if action_state.pressed(&Action::Walk) {
        let direction = action_state
            .clamped_axis_pair(&Action::Walk)
            .unwrap()
            .xy();
        transform.translation += direction.extend(1.);
    }
}
