use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod board;
use board::{board, Board, MatchMade};
use match_counter::{MatchCounter, match_counter};

mod match_counter;

mod map;
use map::map;

#[derive(Resource)]
pub struct TotalMatches(u32);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    Map,
    #[default]
    Board,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(board)
        .add_plugins(match_counter)
        .add_plugins(map)
        .add_systems(Startup, setup_camera)
        .add_systems(PostStartup, layout_nodes)
        .add_systems(FixedUpdate, increment_total_matches)
        .add_systems(FixedUpdate, go_to_map_after_20_matches)
        .insert_resource(TotalMatches(0))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn layout_nodes(
    board: Query<Entity, With<Board>>,
    match_counter: Query<Entity, With<MatchCounter>>,
    mut commands: Commands
) {
    let mut container = commands.spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        ..default()
    });

    container.add_child(board.single());
    container.add_child(match_counter.single());
}

fn increment_total_matches(
    mut matches_made: EventReader<MatchMade>,
    mut total_matches: ResMut<TotalMatches>
) {
    for _match_made in matches_made.read() {
        total_matches.0 += 1;
    }
}

fn go_to_map_after_20_matches(
    total_matches: Res<TotalMatches>,
    mut state: ResMut<NextState<GameState>>
) {
    if total_matches.0 >= 20 {
        state.set(GameState::Map);
    }
}
