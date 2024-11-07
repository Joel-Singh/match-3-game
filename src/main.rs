use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component)]
struct Board(Vec<Entity>);

#[derive(Component, Default)]
struct Shape;

const BOARD_POSITION: Transform = Transform::from_xyz(-200.0, 200.0, 0.0);
const BOARD_SIZE: i32 = 10;
const BOARD_TOTAL_SHAPES: i32 = 3;

const SHAPE_DISTANCE: i32 = 2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            bevy::log::LogPlugin {
                level: bevy::log::Level::DEBUG,
                ..default()
            }
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, (
            spawn_board,
            setup
        ).chain())
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_board( mut commands: Commands
) { 
    commands.spawn((
        Board(Vec::new()),
        NodeBundle {
            style: Style {
                width: Val::Px(400.),
                height: Val::Px(400.),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Srgba::new(1.0,1.0,1.0,0.1).into(),
            ..default()
        },
    )).insert(BOARD_POSITION);
}

fn setup(
    mut board: Query<(&mut Board, Entity)>,
    mut commands: Commands
) {
    let (mut board, board_entity) = board.get_single_mut().unwrap();

    for _ in 0..BOARD_TOTAL_SHAPES {
        let spawned_shape = commands.spawn((
            Shape::default(), 
            ButtonBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgb(1.0, 0.0, 0.0).into(),
                ..default()
            },
        )).id();

        commands.entity(board_entity).push_children(&[spawned_shape]); 
        board.0.push(spawned_shape);
    }
}
