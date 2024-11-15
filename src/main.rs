use bevy::prelude::*; use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod board;
use board::{board, MatchMade};

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
        .add_plugins(map)
        .add_systems(Startup, setup_camera)
        .add_systems(FixedUpdate, increment_total_matches)
        .add_systems(FixedUpdate, go_to_map_after_30_matches)
        .insert_resource(TotalMatches(0))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn increment_total_matches(
    mut matches_made: EventReader<MatchMade>,
    mut total_matches: ResMut<TotalMatches>
) {
    for _match_made in matches_made.read() {
        total_matches.0 += 1;
    }
}

fn go_to_map_after_30_matches(
    total_matches: Res<TotalMatches>,
    mut state: ResMut<NextState<GameState>>
) {
    if total_matches.0 >= 30 {
        state.set(GameState::Map);
    }
}
