use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod board;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(board::board)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
