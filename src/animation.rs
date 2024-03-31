use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (animate_sprites, flash_sprites));
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct FlashingTimer(pub Timer);

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

pub fn flash_sprites(
    mut commands: Commands,
    mut flashing_query: Query<(&mut FlashingTimer, Entity)>,
    time: Res<Time>,
) {
    for (mut timer, entity) in &mut flashing_query {
        let mut entity = commands.entity(entity);
        entity.insert(Visibility::Hidden);

        timer.0.tick(time.delta());

        if timer.0.finished() {
            entity.insert(Visibility::Visible);
            entity.remove::<FlashingTimer>();
        }
    }
}
