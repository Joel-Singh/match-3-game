use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::Rng;

#[derive(Component)]
struct Board(Vec<Entity>);

#[derive(Component, Default)]
struct Shape;

const BOARD_POSITION: Transform = Transform::from_xyz(-200.0, 200.0, 0.0);
const BOARD_SIZE: i32 = 10;
const BOARD_TOTAL_SHAPES: i32 = BOARD_SIZE * BOARD_SIZE;

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
                row_gap: Val::Px(5.),
                column_gap: Val::Px(5.),
                padding: UiRect::all(Val::Px(5.)),
                grid_template_columns: RepeatedGridTrack::fr(10, 1.0),
                grid_template_rows: RepeatedGridTrack::fr(10, 1.0),
                display: Display::Grid,
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
        let mut rng = rand::thread_rng();
        let spawned_shape = commands.spawn((
            Shape::default(), 
            ButtonBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    ..default()
                },
                background_color: Color::srgb(rng.gen(), rng.gen(), rng.gen()).into(),
                ..default()
            },
        )).id();

        commands.entity(board_entity).add_child(spawned_shape);
        board.0.push(spawned_shape);
    }
}
