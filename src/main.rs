use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod board;
use board::{board, Board};
use match_counter::{MatchCounter, match_counter};

mod match_counter;

#[derive(Resource)]
pub struct TotalMatches(u32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(board)
        .add_plugins(match_counter)
        .add_systems(Startup, setup_camera)
        .add_systems(PostStartup, layout_nodes)
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
