use crate::constants::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

const WALK_SPEED: f32 = 75.;
const RUN_SPEED: f32 = 150.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMoves>()
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (player_walks, player_dashes))
            .add_systems(FixedUpdate, move_player);
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum PlayerAction {
    // Movement
    Up,
    Down,
    Left,
    Right,

    // Actions
    Walk,
    Run,
}

impl PlayerAction {
    const DIRECTIONS: [Self; 4] = [
        PlayerAction::Up,
        PlayerAction::Down,
        PlayerAction::Left,
        PlayerAction::Right,
    ];

    fn direction(self) -> Option<Direction2d> {
        match self {
            PlayerAction::Up => Some(Direction2d::Y),
            PlayerAction::Down => Some(Direction2d::NEG_Y),
            PlayerAction::Left => Some(Direction2d::NEG_X),
            PlayerAction::Right => Some(Direction2d::X),
            _ => None,
        }
    }
}

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    player: Player,
    input_manager: InputManagerBundle<PlayerAction>,
}

impl PlayerBundle {
    fn default() -> Self {
        let translation = Vec3::new(0., 0., 1.);
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(translation + GROUND_OFFSET),
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Vec2::new(50., 50.).into(),
                    ..default()
                },
                ..default()
            },
            player: Player,
            input_manager: InputManagerBundle::with_map(PlayerBundle::default_input_map()),
        }
    }

    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(Up, KeyCode::ArrowUp);
        input_map.insert(Up, KeyCode::KeyW);
        input_map.insert(Up, GamepadButtonType::DPadUp);

        input_map.insert(Down, KeyCode::ArrowDown);
        input_map.insert(Down, KeyCode::KeyS);
        input_map.insert(Down, GamepadButtonType::DPadDown);

        input_map.insert(Left, KeyCode::ArrowLeft);
        input_map.insert(Left, KeyCode::KeyA);
        input_map.insert(Left, GamepadButtonType::DPadLeft);

        input_map.insert(Right, KeyCode::ArrowRight);
        input_map.insert(Right, KeyCode::KeyD);
        input_map.insert(Right, GamepadButtonType::DPadRight);

        // Actions
        input_map.insert(Run, KeyCode::ShiftLeft);
        input_map.insert(Run, KeyCode::ShiftRight);
        input_map.insert(Run, GamepadButtonType::East);

        input_map
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
}

#[derive(Event)]
struct PlayerMoves {
    direction: Direction2d,
    dashing: bool,
}

fn player_walks(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut event_writer: EventWriter<PlayerMoves>,
) {
    let action_state = query.single();

    let mut direction_vector = Vec2::ZERO;

    for input_direction in PlayerAction::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            if let Some(direction) = input_direction.direction() {
                direction_vector += *direction;
            }
        }
    }

    let net_direction = Direction2d::new(direction_vector);

    if let Ok(direction) = net_direction {
        event_writer.send(PlayerMoves {
            direction,
            dashing: false,
        });
    }
}

fn player_dashes(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut event_writer: EventWriter<PlayerMoves>,
) {
    let action_state = query.single();

    if action_state.pressed(&PlayerAction::Run) {
        let mut direction_vector = Vec2::ZERO;

        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.pressed(&input_direction) {
                if let Some(direction) = input_direction.direction() {
                    direction_vector += *direction;
                }
            }
        }

        let net_direction = Direction2d::new(direction_vector);
        if let Ok(direction) = net_direction {
            event_writer.send(PlayerMoves {
                direction,
                dashing: true,
            });
        }
    }
}

fn move_player(
    mut query: Query<(&mut Transform, &mut Sprite), With<Player>>,
    mut player_moves: EventReader<PlayerMoves>,
    time: Res<Time>
) {
    if query.is_empty() {
        return;
    }

    let (mut transform, mut sprite) = query.single_mut();
    let mut movement = 0.0;

    for event in player_moves.read() {
        match event {
            PlayerMoves {
                direction,
                dashing
            } => {
                sprite.color = Color::RED;
                if *dashing {
                    sprite.color = Color::BLUE;
                }
                transform.translation += Vec3::new(direction.x, direction.y, 0.0) * time.delta_seconds() * if *dashing { RUN_SPEED } else { WALK_SPEED };
            }
        }
    }
}
