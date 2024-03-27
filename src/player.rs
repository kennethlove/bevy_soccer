use crate::{animation::{AnimationIndices, AnimationTimer}, constants::*};
use bevy::{prelude::*, transform::commands};
use leafwing_input_manager::prelude::*;

const WALK_SPEED: f32 = 75.;
const RUN_SPEED: f32 = 150.;

const IDLE_FRAMES: AnimationIndices = AnimationIndices { first: 0, last: 3 };
const WALK_FRAMES: AnimationIndices = AnimationIndices { first: 4, last: 10 };
const RUN_FRAMES: AnimationIndices = AnimationIndices { first: 18, last: 22 };

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
    Idle,
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
    sprite_bundle: SpriteSheetBundle,
    player: Player,
    input_manager: InputManagerBundle<PlayerAction>,
}

impl PlayerBundle {
    fn default() -> Self {
        let translation = Vec3::new(0., 0., 1.);
        Self {
            sprite_bundle: SpriteSheetBundle {
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

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image> = asset_server.load("sprites/blue.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(24., 24.), 24, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let mut player = PlayerBundle::default();

    player.sprite_bundle.texture = texture;
    player.sprite_bundle.sprite.color = Color::WHITE;
    player.sprite_bundle.atlas = TextureAtlas {
        layout: texture_atlas_layout,
        index: IDLE_FRAMES.first,
    };
    let mut player = commands.spawn(player);
    player.insert(IDLE_FRAMES);
    player.insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));

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
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Player>>,
    mut player_moves: EventReader<PlayerMoves>,
    time: Res<Time>
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut transform) = query.single_mut();
    let mut entity = commands.entity(entity);
    entity.insert(IDLE_FRAMES);

    for event in player_moves.read() {
        match event {
            PlayerMoves {
                direction,
                dashing
            } => {
                if *dashing {
                    entity.insert(RUN_FRAMES);
                } else {
                    entity.insert(WALK_FRAMES);
                }
                transform.translation += Vec3::new(direction.x, direction.y, 0.0) *
                                            time.delta_seconds() *
                                            if *dashing { RUN_SPEED } else { WALK_SPEED };
            }
        }
    }
}
