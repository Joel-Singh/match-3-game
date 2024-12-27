use std::iter::repeat_with;

use bevy::prelude::*;

use crate::{
    board::{get_board_styling, get_shape_styling, shape::Shape, utils::get_index, BOARD_SIZE},
    GameState,
};

#[derive(Component)]
struct ExplanationScreen;

pub fn explanation_screen(app: &mut App) {
    app.add_systems(OnEnter(GameState::ExplanationScreen), setup)
        .add_systems(OnExit(GameState::ExplanationScreen), cleanup);
}

fn get_board_shapes(shape: Shape) -> [ShapeOrInvisible; BOARD_SIZE * BOARD_SIZE] {
    repeat_with(|| ShapeOrInvisible::Shape(shape))
        .take(BOARD_SIZE * BOARD_SIZE)
        .collect::<Vec<ShapeOrInvisible>>()
        .try_into()
        .unwrap()
}
fn setup(mut commands: Commands) {
    let liner_explanation_boards = &[
        spawn_board(&mut commands, get_liner_example1()),
        spawn_board(&mut commands, get_liner_example2()),
        spawn_board(&mut commands, get_liner_example3()),
    ];

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
        .add_children(liner_explanation_boards);

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

struct Invisible;

#[derive(Debug)]
enum ShapeOrInvisible {
    Shape(Shape),
    Invisible,
}

fn get_liner_example1() -> [ShapeOrInvisible; BOARD_SIZE * BOARD_SIZE] {
    let mut board_shapes = get_board_shapes(Shape::Blue);
    board_shapes[get_index(1, 3).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Red);
    board_shapes[get_index(2, 3).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Red);
    board_shapes[get_index(3, 3).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Red);
    board_shapes[get_index(4, 3).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Red);
    board_shapes[get_index(5, 3).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Red);

    return board_shapes;
}

fn get_liner_example2() -> [ShapeOrInvisible; BOARD_SIZE * BOARD_SIZE] {
    let mut board_shapes = get_board_shapes(Shape::Blue);
    board_shapes[get_index(3, 3).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::VerticalLiner);

    return board_shapes;
}

fn get_liner_example3() -> [ShapeOrInvisible; BOARD_SIZE * BOARD_SIZE] {
    let mut board_shapes = get_board_shapes(Shape::Blue);
    board_shapes[get_index(1, 3).unwrap() as usize] = ShapeOrInvisible::Invisible;
    board_shapes[get_index(2, 3).unwrap() as usize] = ShapeOrInvisible::Invisible;
    board_shapes[get_index(3, 3).unwrap() as usize] = ShapeOrInvisible::Invisible;
    board_shapes[get_index(4, 3).unwrap() as usize] = ShapeOrInvisible::Invisible;
    board_shapes[get_index(5, 3).unwrap() as usize] = ShapeOrInvisible::Invisible;

    return board_shapes;
}

fn spawn_board(
    commands: &mut Commands,
    shapes: [ShapeOrInvisible; BOARD_SIZE * BOARD_SIZE],
) -> Entity {
    let board = commands
        .spawn(get_board_styling())
        .with_children(|parent| {
            for shape_or_invisible in shapes {
                if let ShapeOrInvisible::Shape(shape) = shape_or_invisible {
                    parent.spawn(get_shape_styling(shape));
                } else {
                    parent
                        .spawn(get_shape_styling(Shape::Red))
                        .insert(BackgroundColor(Color::NONE));
                }
            }
        })
        .id();

    board
}

fn cleanup(mut commands: Commands, map: Query<Entity, With<ExplanationScreen>>) {
    commands.entity(map.single()).despawn_recursive();
}
