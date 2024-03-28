use crate::{
    animation::{AnimationIndices, AnimationTimer},
    constants::*,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

const WALK_SPEED: f32 = 75.;
const RUN_SPEED: f32 = 150.;

const IDLE_FRAMES: AnimationIndices = AnimationIndices { first: 0, last: 3 };
const WALK_FRAMES: AnimationIndices = AnimationIndices { first: 4, last: 10 };
const KICK_FRAMES: AnimationIndices = AnimationIndices { first: 11, last: 12 };
const RUN_FRAMES: AnimationIndices = AnimationIndices { first: 18, last: 23 };

#[derive(Clone, Component, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum PlayerState {
    #[default]
    Idle,
    Walking,
    Running,
    Kicking,
}

fn log_transitions(mut transitions: EventReader<StateTransitionEvent<PlayerState>>) {
    for transition in transitions.read() {
        info!(
            "transition: {:?} => {:?}",
            transition.before, transition.after
        );
    }
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>()
            .add_event::<PlayerMoves>()
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, (player_idles, player_walks, player_runs, player_kicks))
            .add_systems(
                Update,
                (
                    log_transitions,
                    movement,
                    idle_animation.run_if(in_state(PlayerState::Idle)),
                    run_animation.run_if(in_state(PlayerState::Running)),
                    walk_animation.run_if(in_state(PlayerState::Walking)),
                    kick_animation.run_if(in_state(PlayerState::Kicking)),
                ),
            );
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
    Kick,
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

        input_map.insert(Kick, KeyCode::Space);
        input_map.insert(Kick, GamepadButtonType::South);

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
    player.insert(AnimationTimer(Timer::from_seconds(
        0.1,
        TimerMode::Repeating,
    )));
}

#[derive(Event)]
struct PlayerMoves {
    direction: Option<Direction2d>,
    running: bool,
    kicking: bool,
}

impl Default for PlayerMoves {
    fn default() -> Self {
        Self {
            direction: None,
            running: false,
            kicking: false,
        }
    }
}

fn player_idles(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut next_state: ResMut<NextState<PlayerState>>,
) {
    let action_state = query.single();

    for input_direction in PlayerAction::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            return
        }
    }

    next_state.set(PlayerState::Idle);
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
            direction: Some(direction),
            ..default()
        });
    }
}

fn player_runs(
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
                direction: Some(direction),
                running: true,
                ..default()
            });
        }
    }
}

fn player_kicks(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut event_writer: EventWriter<PlayerMoves>,
) {
    let action_state = query.single();

    if action_state.pressed(&PlayerAction::Kick) {
            event_writer.send(PlayerMoves {
                direction: None,
                kicking: true,
                ..default()
            });
    }
}

fn movement(
    mut query: Query<(&mut Transform, &mut Sprite), With<Player>>,
    mut player_moves: EventReader<PlayerMoves>,
    mut next_state: ResMut<NextState<PlayerState>>,
    time: Res<Time>,
) {
    if query.is_empty() {
        return;
    }

    let (mut transform, mut sprite) = query.single_mut();

    for event in player_moves.read() {
        let PlayerMoves { direction, running, kicking } = event;
        {
            if *running {
                next_state.set(PlayerState::Running);
            } else if *kicking {
                next_state.set(PlayerState::Kicking);
            } else {
                next_state.set(PlayerState::Walking);
            }
            if let Some(direction) = direction {
                sprite.flip_x = direction.x < 0.;
                transform.translation += Vec3::new(direction.x, direction.y, 0.0)
                    * time.delta_seconds()
                    * if *running { RUN_SPEED } else { WALK_SPEED };
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

fn kick_animation(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TextureAtlas), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut atlas) = query.single_mut();
    let mut entity = commands.entity(entity);
    entity.insert(KICK_FRAMES);
    if atlas.index < KICK_FRAMES.first || atlas.index > KICK_FRAMES.last {
        atlas.index = KICK_FRAMES.first;
    }
}
