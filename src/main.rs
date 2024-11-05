use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component)]
struct Board(Vec<Entity>);

#[derive(Component)]
struct Shape;

const BOARD_POSITION: Transform = Transform::from_xyz(0.0, 0.0, 0.0);
const BOARD_SIZE: i32 = 10;
const BOARD_TOTAL_SHAPES: i32 = BOARD_SIZE * BOARD_SIZE;

const SHAPE_SIZE: i32 = 35;

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
        .add_systems(FixedUpdate, position_board_elements)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_board( mut commands: Commands
) { 
    commands.spawn(Board(Vec::new()));
}

fn setup(
    mut board: Query<&mut Board>,
    mut commands: Commands
) {
    let mut board = board.get_single_mut().expect("Getting just spawned board");

    for _ in 0..BOARD_TOTAL_SHAPES {
        let spawned_shape = commands.spawn((
            Shape,
            SpriteBundle {
                transform: Transform {
                    scale: Vec2::new(10.0, 10.0).extend(1.0),
                    ..default()
                },
                ..default()
            },
        ));

        board.0.push(spawned_shape.id());
    }
}

fn position_board_elements(
    board: Query<&Board>,
    mut shapes: Query<&mut Transform, With<Shape>>
) {
    let board = board.get_single().unwrap();

    for (index, entity) in board.0.iter().enumerate() {
        let column: i32 = (index as i32 % 10) + 1;
        let row: i32 = (index as i32 / 10) + 1;

        let [mut shape_transform] = shapes.get_many_mut([*entity]).unwrap();

        shape_transform.translation.x = (SHAPE_SIZE * column) as f32;
        shape_transform.translation.y = (SHAPE_SIZE * row * -1) as f32;
    }
}
