use bevy::{color::palettes::css::WHITE, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod board;
use board::{board, Board, MatchMade};

#[derive(Component)]
struct MatchCounter;

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
        .add_systems(Startup, spawn_match_counter)
        .add_systems(PostStartup, layout_nodes)
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

fn debug_matches(mut matches_made: EventReader<MatchMade>) {
    for _match_made in matches_made.read() {
        println!("Match made");
    }
}

fn spawn_match_counter(mut commands: Commands) {
    commands.spawn((
        MatchCounter,
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 100.0,
                color: WHITE.into(),
                ..default()
            }
        )
    ));
}
