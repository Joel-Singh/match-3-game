use bevy::{color::palettes::tailwind::GRAY_50, prelude::*};

use crate::GameState;

#[derive(Component)]
pub struct Map;

#[derive(Component)]
enum BoardButton {
    First,
    Second,
    Third,
}

pub fn map(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Map), setup)
        .add_systems(FixedUpdate,
            (
                go_to_board_on_click,
            ).run_if(in_state(GameState::Map))
        )
        .add_systems(OnExit(GameState::Map), cleanup);
}

fn setup(mut commands: Commands) {
    commands.spawn((Map, NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        ..default()
    })).with_children(|parent| {
        parent.spawn(get_board_button(BoardButton::Third));
        parent.spawn(get_board_button(BoardButton::Second));
        parent.spawn(get_board_button(BoardButton::First));
    });
}

fn cleanup(mut commands: Commands, map: Query<Entity, With<Map>>) {
    commands.entity(map.single()).despawn_recursive();
}

fn get_board_button(area: BoardButton) -> (ButtonBundle, BoardButton) {
    (ButtonBundle {
        style: Style {
            width: Val::Px(50.),
            height: Val::Px(50.),
            margin: UiRect::all(Val::Px(5.)),
            ..default()
        },
        background_color: GRAY_50.into(),
        ..default()
    },
    area
    )
}

fn go_to_board_on_click(
    mut state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<BoardButton>)>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            state.set(GameState::Board);
        }
    }
}

