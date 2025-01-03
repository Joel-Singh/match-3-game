use bevy::prelude::*;

mod board;
use board::{board, MatchMade};

mod map;
use map::map;

mod win_screen;
use win_screen::win_screen;

mod start_screen;
use start_screen::start_screen;

mod explanation_screen;
use explanation_screen::explanation_screen;

mod dev_hotkeys;

#[derive(Resource)]
pub struct TotalMatches(u32);

#[derive(Resource)]
pub struct NeededMatches(u32);

#[derive(Resource, Debug, Default)]
pub struct MapFinishes {
    map1: bool,
    map2: bool,
    map3: bool,
    map4: bool,
}

#[derive(Resource, Default, PartialEq)]
pub enum CurrentMap {
    #[default]
    None,
    One,
    Two,
    Three,
    Four,
}

impl CurrentMap {
    pub fn get(&self) -> &Self {
        self
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    Map,
    Board,
    WinScreen,
    #[default]
    StartScreen,
    ExplanationScreen,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            //level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .init_state::<GameState>()
        //.add_plugins(WorldInspectorPlugin::new())
        //.add_plugins(dev_hotkeys)
        .add_plugins(board)
        .add_plugins(map)
        .add_plugins(win_screen)
        .add_plugins(start_screen)
        .add_plugins(explanation_screen)
        .add_systems(Startup, setup_camera)
        .add_systems(FixedUpdate, increment_total_matches)
        .add_systems(FixedUpdate, go_to_next_screen)
        .insert_resource(TotalMatches(0))
        .insert_resource(NeededMatches(30))
        .insert_resource(MapFinishes::default())
        .insert_resource(CurrentMap::None)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn increment_total_matches(
    mut matches_made: EventReader<MatchMade>,
    mut total_matches: ResMut<TotalMatches>,
) {
    for _match_made in matches_made.read() {
        total_matches.0 += 1;
    }
}

fn go_to_next_screen(
    total_matches: Res<TotalMatches>,
    needed_matches: Res<NeededMatches>,
    mut state: ResMut<NextState<GameState>>,
    current_map: Res<CurrentMap>,
) {
    if total_matches.0 >= needed_matches.0 {
        if *current_map == CurrentMap::Four {
            state.set(GameState::WinScreen);
        } else {
            state.set(GameState::ExplanationScreen);
        }
    }
}
