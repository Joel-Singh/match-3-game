use bevy::{color::palettes::tailwind::GRAY_50, prelude::*};

use crate::{CurrentMap, GameState, NeededMatches};

#[derive(Component)]
pub struct Map;

#[derive(Component, Clone, Copy)]
enum BoardButton {
    First,
    Second,
    Third,
}

pub fn map(app: &mut App) {
    app.add_systems(OnEnter(GameState::Map), setup)
        .add_systems(
            FixedUpdate,
            (go_to_board_on_click,).run_if(in_state(GameState::Map)),
        )
        .add_systems(OnExit(GameState::Map), cleanup);
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Map,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Auto),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(get_board_button(BoardButton::Third));
            parent.spawn(get_board_button(BoardButton::Second));
            parent.spawn(get_board_button(BoardButton::First));
        });
}

fn cleanup(mut commands: Commands, map: Query<Entity, With<Map>>) {
    commands.entity(map.single()).despawn_recursive();
}

fn get_board_button(area: BoardButton) -> (Button, BackgroundColor, Node, BoardButton) {
    (
        Button::default(),
        BackgroundColor(GRAY_50.into()),
        Node {
            width: Val::Px(50.),
            height: Val::Px(50.),
            margin: UiRect::all(Val::Px(5.)),
            ..default()
        },
        area,
    )
}

fn go_to_board_on_click(
    mut state: ResMut<NextState<GameState>>,
    mut current_map: ResMut<CurrentMap>,
    mut needed_matches: ResMut<NeededMatches>,
    mut interaction_query: Query<(&Interaction, &BoardButton), Changed<Interaction>>,
) {
    let mut configure_board = |next_needed_matches: u32, next_map: CurrentMap| {
        needed_matches.0 = next_needed_matches;
        *current_map = next_map;
        state.set(GameState::Board);
    };

    for (interaction, board_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match *board_button {
                BoardButton::First => configure_board(50, CurrentMap::One),
                BoardButton::Second => configure_board(51, CurrentMap::Two),
                BoardButton::Third => configure_board(52, CurrentMap::Three),
            }
        }
    }
}
