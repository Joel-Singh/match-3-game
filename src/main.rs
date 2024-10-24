use bevy::{prelude::*};

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
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec2::new(10.0, 10.0).extend(1.0),
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::linear_rgb(1.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
    ));
}
