use crate::{
    animation::{AnimationIndices, AnimationTimer},
    constants::*,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

const WALK_SPEED: f32 = 100.;
const RUN_SPEED: f32 = 250.;

const IDLE_FRAMES: AnimationIndices = AnimationIndices { first: 0, last: 3 };
const WALK_FRAMES: AnimationIndices = AnimationIndices { first: 4, last: 10 };
const RUN_FRAMES: AnimationIndices = AnimationIndices {
    first: 18,
    last: 23,
};

#[derive(Clone, Component, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum PlayerState {
    #[default]
    Idle,
    Walking,
    Running,
}

#[derive(Component)]
enum Direction {
    Left,
    Right,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>()
            .add_event::<PlayerMoves>()
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Startup, spawn_player)
            .add_systems(
                FixedUpdate,
                (player_idles, player_moves, update_sprite_direction),
            )
            .add_systems(
                Update,
                (
                    movement,
                    update_direction,
                    idle_animation.run_if(in_state(PlayerState::Idle)),
                    walk_animation.run_if(in_state(PlayerState::Walking)),
                    run_animation.run_if(in_state(PlayerState::Running)),
                ),
            );
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
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
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    sprite_bundle: SpriteSheetBundle,
    player: Player,
    input_manager: InputManagerBundle<PlayerAction>,
    direction: Direction,
}

const PLAYER_STARTING_POS: Vec3 = Vec3::new(-WINDOW_WIDTH / 4., GROUND_MIDDLE, 5.);

impl PlayerBundle {
    fn default() -> Self {
        let translation = PLAYER_STARTING_POS;
        Self {
            sprite_bundle: SpriteSheetBundle {
                transform: Transform::from_translation(translation),
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Vec2::new(50., 50.).into(),
                    ..default()
                },
                ..default()
            },
            player: Player,
            input_manager: InputManagerBundle::with_map(PlayerBundle::default_input_map()),
            direction: Direction::Right,
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

    commands.spawn((
        player,
        IDLE_FRAMES,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        KinematicCharacterController {
            apply_impulse_to_dynamic_bodies: true,
            custom_mass: Some(100.),
            ..default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(12., 17.),
    ));
}

#[derive(Debug, Default, Event)]
struct PlayerMoves {
    direction: Option<Direction2d>,
    running: bool,
}

fn player_idles(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut next_state: ResMut<NextState<PlayerState>>,
) {
    let action_state = query.single();

    for input_direction in PlayerAction::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            return;
        }
    }

    next_state.set(PlayerState::Idle);
}

fn player_moves(
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
            direction: Some(direction),
            running: action_state.pressed(&PlayerAction::Run),
            ..default()
        });
    }
}

fn movement(
    mut query: Query<&mut KinematicCharacterController, With<Player>>,
    mut player_moves: EventReader<PlayerMoves>,
    mut next_state: ResMut<NextState<PlayerState>>,
    time: Res<Time>,
) {
    if query.is_empty() {
        return;
    }

    let mut player = query.single_mut();

    for event in player_moves.read() {
        let PlayerMoves { direction, running } = event;
        {
            if *running {
                next_state.set(PlayerState::Running);
            } else {
                next_state.set(PlayerState::Walking);
            }

            if let Some(direction) = direction {
                player.translation = {
                    Some(
                        Vec2::new(direction.x, direction.y)
                            * time.delta_seconds()
                            * if *running { RUN_SPEED } else { WALK_SPEED },
                    )
                }
            }
        }
    }
}

fn idle_animation(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TextureAtlas), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut atlas) = query.single_mut();
    let mut entity = commands.entity(entity);
    entity.insert(IDLE_FRAMES);
    if atlas.index > IDLE_FRAMES.last {
        atlas.index = IDLE_FRAMES.first;
    }
}

fn walk_animation(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TextureAtlas), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut atlas) = query.single_mut();
    let mut entity = commands.entity(entity);
    entity.insert(WALK_FRAMES);
    if atlas.index < WALK_FRAMES.first || atlas.index > WALK_FRAMES.last {
        atlas.index = WALK_FRAMES.first;
    }
}

fn run_animation(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TextureAtlas), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut atlas) = query.single_mut();
    let mut entity = commands.entity(entity);
    entity.insert(RUN_FRAMES);
    if atlas.index < RUN_FRAMES.first || atlas.index > RUN_FRAMES.last {
        atlas.index = RUN_FRAMES.first;
    }
}

fn update_direction(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput)>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, controller) = query.single();

    if controller.desired_translation.x > 0. {
        commands.entity(entity).insert(Direction::Right);
    } else if controller.desired_translation.x < 0. {
        commands.entity(entity).insert(Direction::Left);
    }
}

fn update_sprite_direction(mut query: Query<(&mut Sprite, &Direction), With<Player>>) {
    if query.is_empty() {
        return;
    }

    let (mut sprite, direction) = query.single_mut();
    match direction {
        Direction::Left => sprite.flip_x = true,
        Direction::Right => sprite.flip_x = false,
    }
}
