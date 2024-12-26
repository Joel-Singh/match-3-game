use std::iter::repeat_with;

use bevy::prelude::*;

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

    let explanation_board_container = commands
        .spawn((
            Node {
                column_gap: Val::Px(40.),
                ..default()
            },
            Name::new("Explanation Board Container"),
        ))
        .id();

    commands
        .entity(explanation_board_container)
        .add_children(&[red, blue, green]);

    let text = commands
        .spawn((
            Text::new("I love Daira"),
            Name::new("Explanation Text"),
            Node {
                margin: UiRect::top(Val::Auto),
                ..default()
            },
        ))
        .id();

    let mut root = commands.spawn((
        ExplanationScreen,
        Node {
            display: Display::Grid,
            width: Val::Vw(100.),
            height: Val::Vh(100.),
            grid_template_rows: vec![GridTrack::fr(0.3), GridTrack::fr(1.0)],
            grid_auto_flow: GridAutoFlow::Column,
            justify_items: JustifyItems::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Name::new("ExplanationScreen Root"),
    ));

    root.add_child(text);
    root.add_child(explanation_board_container);
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
