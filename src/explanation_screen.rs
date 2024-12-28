use std::iter::repeat_with;

use bevy::prelude::*;

use crate::{
    board::{get_board_styling, get_shape_styling, shape::Shape, utils::get_index, BOARD_SIZE},
    GameState, MapFinishes,
};

#[derive(Component)]
struct ExplanationScreen;

pub fn explanation_screen(app: &mut App) {
    app.add_systems(OnEnter(GameState::ExplanationScreen), setup)
        .add_systems(
            FixedUpdate,
            go_to_map_on_click.run_if(in_state(GameState::ExplanationScreen)),
        )
        .add_systems(OnExit(GameState::ExplanationScreen), cleanup);
}

fn get_board_shapes(shape: Shape) -> [ShapeOrInvisible; BOARD_SIZE * BOARD_SIZE] {
    repeat_with(|| ShapeOrInvisible::Shape(shape))
        .take(BOARD_SIZE * BOARD_SIZE)
        .collect::<Vec<ShapeOrInvisible>>()
        .try_into()
        .unwrap()
}
fn setup(mut commands: Commands, map_finishes: Res<MapFinishes>) {
    let explanation_board_container = commands
        .spawn((
            Node {
                column_gap: Val::Px(40.),
                ..default()
            },
            Name::new("Explanation Board Container"),
        ))
        .id();

    let mut explanation_boards: Option<[Entity; 3]> = None;
    if map_finishes.map3 {
        explanation_boards = Some(spawn_eliminator_explanation_boards(&mut commands));
    } else if map_finishes.map2 {
        explanation_boards = Some(spawn_liner_explanation_boards(&mut commands));
    } else if map_finishes.map1 {
        explanation_boards = Some(spawn_bomb_explanation_boards(&mut commands));
    }

    let explanation_boards = explanation_boards.unwrap();

    commands
        .entity(explanation_board_container)
        .add_children(&explanation_boards);

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
        Button,
    ));

    root.add_child(text);
    root.add_child(explanation_board_container);
}

fn go_to_map_on_click(
    mut game_state: ResMut<NextState<GameState>>,
    interactions: Query<&Interaction, Changed<Interaction>>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::Map)
        }
    }
}

#[derive(Debug)]
enum ShapeOrInvisible {
    Shape(Shape),
    Invisible,
}

fn spawn_board(
    commands: &mut Commands,
    shapes: [ShapeOrInvisible; BOARD_SIZE * BOARD_SIZE],
    name: Name,
) -> Entity {
    let board = commands
        .spawn((get_board_styling(), name))
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

fn spawn_liner_explanation_boards(mut commands: &mut Commands) -> [Entity; 3] {
    return [
        spawn_board(
            &mut commands,
            get_liner_example1(),
            Name::new("Liner Example 1"),
        ),
        spawn_board(
            &mut commands,
            get_liner_example2(),
            Name::new("Liner Example 2"),
        ),
        spawn_board(
            &mut commands,
            get_liner_example3(),
            Name::new("Liner Example 3"),
        ),
    ];

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
        board_shapes[get_index(3, 3).unwrap() as usize] =
            ShapeOrInvisible::Shape(Shape::VerticalLiner);

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
}

fn spawn_bomb_explanation_boards(mut commands: &mut Commands) -> [Entity; 3] {
    return [
        spawn_board(
            &mut commands,
            get_bomb_example1(),
            Name::new("Bomb Example 1"),
        ),
        spawn_board(
            &mut commands,
            get_bomb_example2(),
            Name::new("Bomb Example 2"),
        ),
        spawn_board(
            &mut commands,
            get_bomb_example3(),
            Name::new("Bomb Example 3"),
        ),
    ];

    fn get_bomb_example1() -> [ShapeOrInvisible; BOARD_SIZE * BOARD_SIZE] {
        let mut board_shapes = get_board_shapes(Shape::Blue);
        board_shapes[get_index(2, 2).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Red);
        board_shapes[get_index(2, 3).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Red);
        board_shapes[get_index(2, 4).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Red);
        board_shapes[get_index(3, 4).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Red);
        board_shapes[get_index(4, 4).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Red);

        return board_shapes;
    }

    fn get_bomb_example2() -> [ShapeOrInvisible; BOARD_SIZE * BOARD_SIZE] {
        let mut board_shapes = get_board_shapes(Shape::Blue);
        board_shapes[get_index(2, 4).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Bomb);

        return board_shapes;
    }

    fn get_bomb_example3() -> [ShapeOrInvisible; BOARD_SIZE * BOARD_SIZE] {
        let mut board_shapes = get_board_shapes(Shape::Blue);
        board_shapes[get_index(1, 3).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(1, 4).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(1, 5).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(2, 3).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(2, 4).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(2, 5).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(3, 3).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(3, 4).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(3, 5).unwrap() as usize] = ShapeOrInvisible::Invisible;

        return board_shapes;
    }
}

fn spawn_eliminator_explanation_boards(mut commands: &mut Commands) -> [Entity; 3] {
    return [
        spawn_board(
            &mut commands,
            get_eliminator_example1(),
            Name::new("Eliminator Board 1"),
        ),
        spawn_board(
            &mut commands,
            get_eliminator_example2(),
            Name::new("Eliminator Board 2"),
        ),
        spawn_board(
            &mut commands,
            get_eliminator_example3(),
            Name::new("Eliminator Board 3"),
        ),
    ];

    fn get_eliminator_example1() -> [ShapeOrInvisible; BOARD_SIZE * BOARD_SIZE] {
        let mut board_shapes = get_board_shapes(Shape::Blue);
        board_shapes[get_index(3, 1).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Red);
        board_shapes[get_index(3, 2).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Red);
        board_shapes[get_index(3, 3).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Red);
        board_shapes[get_index(3, 4).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Red);
        board_shapes[get_index(3, 5).unwrap() as usize] = ShapeOrInvisible::Shape(Shape::Red);

        return board_shapes;
    }

    fn get_eliminator_example2() -> [ShapeOrInvisible; BOARD_SIZE * BOARD_SIZE] {
        let mut board_shapes = get_board_shapes(Shape::Blue);
        board_shapes[get_index(3, 3).unwrap() as usize] =
            ShapeOrInvisible::Shape(Shape::Eliminator);

        return board_shapes;
    }

    fn get_eliminator_example3() -> [ShapeOrInvisible; BOARD_SIZE * BOARD_SIZE] {
        let mut board_shapes = get_board_shapes(Shape::Blue);

        board_shapes[get_index(3, 3).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(3, 2).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(1, 5).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(1, 1).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(4, 4).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(2, 3).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(5, 3).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(2, 1).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(4, 1).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(1, 3).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(4, 2).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(5, 2).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(1, 2).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(2, 5).unwrap() as usize] = ShapeOrInvisible::Invisible;
        board_shapes[get_index(4, 5).unwrap() as usize] = ShapeOrInvisible::Invisible;

        return board_shapes;
    }
}

fn cleanup(mut commands: Commands, explanation_screen: Query<Entity, With<ExplanationScreen>>) {
    commands
        .entity(explanation_screen.single())
        .despawn_recursive();
}
