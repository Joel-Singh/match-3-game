use std::iter::repeat_with;

use bevy::{
    color::palettes::tailwind::{GRAY_950, RED_50},
    prelude::*,
};

use crate::{
    board::{get_board_styling, get_shape_styling, shape::Shape, BOARD_SIZE},
    GameState,
};

#[derive(Component)]
struct ExplanationScreen;

pub fn explanation_screen(app: &mut App) {
    app.add_systems(OnEnter(GameState::ExplanationScreen), setup)
        .add_systems(OnExit(GameState::ExplanationScreen), cleanup);
}

fn setup(mut commands: Commands) {
    fn get_board_shapes(shape: Shape) -> [Shape; BOARD_SIZE * BOARD_SIZE] {
        repeat_with(|| shape)
            .take(BOARD_SIZE * BOARD_SIZE)
            .collect::<Vec<Shape>>()
            .try_into()
            .unwrap()
    }

    let red = spawn_board(&mut commands, get_board_shapes(Shape::Red));
    let blue = spawn_board(&mut commands, get_board_shapes(Shape::Blue));
    let green = spawn_board(&mut commands, get_board_shapes(Shape::Green));

    let mut root = commands.spawn((
        ExplanationScreen,
        Node {
            display: Display::Flex,
            ..default()
        },
        Name::new("ExplanationScreen Root"),
    ));

    root.add_children(&[red, blue, green]);
}

fn spawn_board(commands: &mut Commands, shapes: [Shape; BOARD_SIZE * BOARD_SIZE]) -> Entity {
    let board = commands
        .spawn(get_board_styling())
        .with_children(|parent| {
            for shape in shapes {
                parent.spawn(get_shape_styling(shape));
            }
        })
        .id();

    board
}

fn cleanup(mut commands: Commands, map: Query<Entity, With<ExplanationScreen>>) {
    commands.entity(map.single()).despawn_recursive();
}
