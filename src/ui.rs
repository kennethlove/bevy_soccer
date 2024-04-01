use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::{camera::UI_LAYER, constants::*};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, pkv_store: Res<PkvStore>) {
    let font = asset_server.load("fonts/More 15 Basic.ttf");

    let high_score = pkv_store.get::<i32>("high_score").unwrap_or(0);
    let score = pkv_store.get::<i32>("score").unwrap_or(0);

    let text_style = TextStyle {
        color: Color::WHITE,
        font_size: 28.0,
        font,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Px(UI_HEIGHT),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        left: Val::Px(0.),
                        right: Val::Px(0.),
                        top: Val::Px(GROUND_SIZE_HEIGHT),
                        bottom: Val::Px(0.),
                    },
                    ..default()
                },
                ..default()
            },
            UI_LAYER,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(format!("High Score {}", high_score), text_style.clone()),
                HighScoreText,
            ));
            parent.spawn((
                TextBundle::from_section(format!("Score {}", score), text_style.clone()),
                ScoreText,
            ));
        });
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct HighScoreText;

fn update_ui(
    mut params: ParamSet<(
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<HighScoreText>>,
    )>,
    pkv: Res<PkvStore>,
) {
    let score = pkv.get::<i32>("score").unwrap_or(0);
    let high_score = pkv.get::<i32>("high_score").unwrap_or(0);

    for mut text in &mut params.p0() {
        text.sections[0].value = format!("Score: {}", score);
    }
    for mut text in &mut params.p1() {
        text.sections[0].value = format!("Hi Score: {}", high_score);
    }
}

pub fn cleanup_ui(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction, &mut UiImage), With<Button>>,
) {
    for entity in &mut interaction_query.iter() {
        commands.entity(entity.0).despawn_recursive();
    }
}
