use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod board;
use board::{board, MatchMade};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(board)
        .add_systems(Startup, setup_camera)
        .add_systems(FixedUpdate, debug_matches)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn debug_matches(mut matches_made: EventReader<MatchMade>) {
    for _match_made in matches_made.read() {
        println!("Match made");
    }
}
