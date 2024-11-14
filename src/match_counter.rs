
use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{GameState, TotalMatches};

#[derive(Component)]
pub struct MatchCounter;

pub(crate) fn match_counter(app: &mut App) {
    app
        .add_systems(FixedUpdate,
            update_match_counter.run_if(in_state(GameState::Board))
        )
        .add_systems(OnEnter(GameState::Board), spawn_match_counter)
        .add_systems(OnExit(GameState::Board), cleanup);
}

fn spawn_match_counter(mut commands: Commands) {
    commands.spawn((
        MatchCounter,
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 100.0,
                color: WHITE.into(),
                ..default()
            }
        )
    ));
}

fn update_match_counter(
    total_matches: Res<TotalMatches>,
    mut match_counter_text: Query<&mut Text, With<MatchCounter>>
) {
    let mut text = match_counter_text.single_mut();
    text.sections[0].value = total_matches.0.to_string();
}

fn cleanup(
    mut commands: Commands, match_counter: Query<Entity, With<MatchCounter>>
) {
    commands.entity(match_counter.single()).despawn_recursive();
}
