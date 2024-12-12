use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod board;
use board::{board, MatchMade};

mod map;
use map::map;

mod win_screen;
use win_screen::win_screen;

#[derive(Resource)]
pub struct TotalMatches(u32);

#[derive(Resource)]
pub struct NeededMatches(u32);

#[derive(Resource, Debug)]
pub struct MapFinishes {
    map1: bool,
    map2: bool,
    map3: bool,
    map4: bool,
}

impl Default for MapFinishes {
    fn default() -> Self {
        MapFinishes {
            map1: false,
            map2: false,
            map3: false,
            map4: false,
        }
    }
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
        return self;
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Map,
    Board,
    WinScreen,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            //level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(board)
        .add_plugins(map)
        .add_plugins(win_screen)
        .add_systems(Startup, setup_camera)
        .add_systems(FixedUpdate, increment_total_matches)
        .add_systems(FixedUpdate, go_to_map_or_winscreen_after_enough_matches)
        .insert_resource(TotalMatches(0))
        .insert_resource(NeededMatches(30))
        .insert_resource(MapFinishes::default())
        .insert_resource(CurrentMap::None)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn increment_total_matches(
    mut matches_made: EventReader<MatchMade>,
    mut total_matches: ResMut<TotalMatches>,
) {
    for _match_made in matches_made.read() {
        total_matches.0 += 1;
    }
}

fn go_to_map_or_winscreen_after_enough_matches(
    total_matches: Res<TotalMatches>,
    needed_matches: Res<NeededMatches>,
    mut state: ResMut<NextState<GameState>>,
    current_map: Res<CurrentMap>,
) {
    if total_matches.0 >= needed_matches.0 {
        if *current_map == CurrentMap::Four {
            state.set(GameState::WinScreen);
        } else {
            state.set(GameState::Map);
        }
    }
}
