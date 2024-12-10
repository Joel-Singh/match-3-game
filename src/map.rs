use bevy::{
    color::palettes::tailwind::{GRAY_50, GRAY_950},
    prelude::*,
};

use crate::{CurrentMap, GameState, MapFinishes, NeededMatches};

#[derive(Component)]
pub struct Map;

#[derive(Component, Clone, Copy, Debug)]
enum BoardButton {
    First,
    Second,
    Third,
}

impl BoardButton {
    fn as_str(&self) -> &str {
        match self {
            BoardButton::First => "1",
            BoardButton::Second => "2",
            BoardButton::Third => "3",
        }
    }

    fn map_available(&self, map_finishes: &MapFinishes) -> bool {
        match self {
            BoardButton::First => !map_finishes.map1 && !map_finishes.map2 && !map_finishes.map3,
            BoardButton::Second => map_finishes.map1 && !map_finishes.map2 && !map_finishes.map3,
            BoardButton::Third => map_finishes.map1 && map_finishes.map2 && !map_finishes.map3,
        }
    }
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
            Name::new("BoardButton Container"),
        ))
        .with_children(|parent| {
            insert_board_button_bundle(BoardButton::Third, &mut parent.spawn(()));
            insert_board_button_bundle(BoardButton::Second, &mut parent.spawn(()));
            insert_board_button_bundle(BoardButton::First, &mut parent.spawn(()));
        });
}

fn cleanup(mut commands: Commands, map: Query<Entity, With<Map>>) {
    commands.entity(map.single()).despawn_recursive();
}

fn insert_board_button_bundle(area: BoardButton, commands: &mut EntityCommands) {
    commands.insert((
        Button::default(),
        BackgroundColor(GRAY_50.into()),
        Node {
            width: Val::Px(50.),
            height: Val::Px(50.),
            margin: UiRect::all(Val::Px(5.)),
            ..default()
        },
        Name::new("BoardButton"),
        Text::new(area.as_str()),
        TextLayout {
            justify: JustifyText::Center,
            ..default()
        },
        TextColor(GRAY_950.into()),
        area,
    ));
}

fn go_to_board_on_click(
    mut state: ResMut<NextState<GameState>>,
    mut current_map: ResMut<CurrentMap>,
    map_finishes: ResMut<MapFinishes>,
    mut needed_matches: ResMut<NeededMatches>,
    mut interaction_query: Query<(&Interaction, &BoardButton), Changed<Interaction>>,
) {
    let mut configure_board = |next_needed_matches: u32, next_map: CurrentMap| {
        needed_matches.0 = next_needed_matches;
        *current_map = next_map;
        state.set(GameState::Board);
    };

    for (interaction, board_button) in &mut interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if !board_button.map_available(&map_finishes) {
            return;
        }

        match *board_button {
            BoardButton::First => configure_board(10, CurrentMap::One),
            BoardButton::Second => configure_board(20, CurrentMap::Two),
            BoardButton::Third => configure_board(30, CurrentMap::Three),
        }
    }
}
