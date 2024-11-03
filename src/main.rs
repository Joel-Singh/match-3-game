use bevy::prelude::*;

#[derive(Resource)]
struct Board(Vec<Vec<Entity>>);

#[derive(Component)]
struct Shape;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            bevy::log::LogPlugin {
                level: bevy::log::Level::DEBUG,
                ..default()
            }
        ))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup)
        .insert_resource(Board(Vec::new()))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup(
    mut commands: Commands,
    mut board: ResMut<Board>
) {
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
    board.0.push(Vec::new());
    board.0[0].push(spawned_shape.id());
}
