use bevy::{color::palettes::tailwind::*, prelude::*};

#[derive(Component)]
struct Board();

use shape::*;

impl Board {
    fn get_index(row: usize, col: usize) -> usize {
        ((((row - 1) * BOARD_SIZE) + col) - 1) as usize
    }

    fn get_row_col(index: usize) -> (usize, usize) {
        let row = index / BOARD_SIZE + 1;
        let col = (index % BOARD_SIZE) + 1;
        return (row, col)
    }
}

const BOARD_POSITION: Transform = Transform::from_xyz(-200.0, 200.0, 0.0);
const BOARD_SIZE: usize = 10;
const BOARD_TOTAL_SHAPES: usize = BOARD_SIZE * BOARD_SIZE;

pub(crate) fn board(app: &mut App) {
    app.add_systems(Startup, (spawn_board, setup).chain())
        .add_systems(FixedUpdate, swap_shapes_on_press)
        .add_systems(FixedUpdate, replace_matches)
        .add_systems(FixedUpdate, update_shape_color);
}

fn spawn_board(mut commands: Commands) {
    commands
        .spawn((
            Board(),
            NodeBundle {
                style: Style {
                    width: Val::Px(400.),
                    height: Val::Px(400.),
                    row_gap: Val::Px(5.),
                    column_gap: Val::Px(5.),
                    padding: UiRect::all(Val::Px(5.)),
                    grid_template_columns: RepeatedGridTrack::fr(BOARD_SIZE as u16, 1.0),
                    grid_template_rows: RepeatedGridTrack::fr(BOARD_SIZE as u16, 1.0),
                    display: Display::Grid,
                    ..default()
                },
                background_color: Srgba::new(1.0, 1.0, 1.0, 0.1).into(),
                ..default()
            },
        ))
        .insert(BOARD_POSITION);
}


fn setup(mut board: Query<Entity, With<Board>>, mut commands: Commands) {
    let board = board.get_single_mut().unwrap();

    for _ in 0..BOARD_TOTAL_SHAPES {
        let spawned_shape = commands.spawn(create_shape(get_random_shape())).id();

        commands.entity(board).add_child(spawned_shape);
    }
}

fn swap_shapes_on_press(
    mut interaction_query: Query<
        (&Interaction, Entity),
        (Changed<Interaction>, With<Button>, With<Shape>),
    >,
    mut board_children_q: Query<&mut Children, With<Board>>,
    mut last_pressed_button: Local<Option<Entity>>,
    mut commands: Commands
) {
    for (interaction, just_pressed_button) in &mut interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match *last_pressed_button {
            None => {
                *last_pressed_button = Some(just_pressed_button);
                commands.entity(just_pressed_button).insert(Outline {
                    width: Val::Px(3.0),
                    color: PINK_950.into(),
                    ..default()
                });
            },
            Some(last_pressed_button_e) => {
                let mut board_children = board_children_q.get_single_mut().unwrap();
                let last_pressed_button_i = board_children
                    .iter()
                    .position(|e| *e == last_pressed_button_e)
                    .unwrap();
                let just_pressed_button_i = board_children
                    .iter()
                    .position(|e| *e == just_pressed_button)
                    .unwrap();

                let (x_1, y_1) = Board::get_row_col(last_pressed_button_i);
                let (x_2, y_2) = Board::get_row_col(just_pressed_button_i);
                let delta_x = ( x_1 as i32 - x_2 as i32).abs();
                let delta_y = ( y_1 as i32 - y_2 as i32).abs();

                let is_next_to = (delta_x + delta_y) == 1;
                if is_next_to {
                    board_children.swap(last_pressed_button_i, just_pressed_button_i);
                }

                commands.entity(last_pressed_button_e).insert(Outline::default());

                *last_pressed_button = None;
            }
        }
    }
}


fn update_shape_color(mut shape: Query<(&Shape, Entity), Changed<Shape>>, mut commands: Commands) {
    for (shape, e) in shape.iter_mut() {
        commands.entity(e).insert(shape.color());
    }
}

fn replace_matches(
    shape_q: Query<&Shape>,
    board: Query<&Children, With<Board>>,
    mut commands: Commands,
) {
    let board = board.get_single().unwrap();
    let matches = get_matches(board, shape_q);

    for board_match in matches {
        for entity in board_match {
            commands.entity(entity).insert(get_random_shape());
        }
    }
}

fn get_matches(board: &Children, shape_q: Query<&Shape>) -> Vec<[Entity; 3]> {
    let mut matches: Vec<[Entity; 3]> = vec![];

    // Check horizontally
    for row in 1..=BOARD_SIZE {
        for col in 1..=(BOARD_SIZE - 2) {
            let first_shape = board.get(Board::get_index(row, col)).unwrap();
            let next_shape = board.get(Board::get_index(row, col + 1)).unwrap();
            let next_next_shape = board.get(Board::get_index(row, col + 2)).unwrap();

            let shapes = shape_q
                .get_many([*first_shape, *next_shape, *next_next_shape])
                .unwrap();

            if *shapes[0] == *shapes[1] && *shapes[0] == *shapes[2] {
                matches.push([*first_shape, *next_shape, *next_next_shape])
            }
        }
    }

    // Check vertically
    for row in 1..=BOARD_SIZE - 2 {
        for col in 1..=(BOARD_SIZE) {
            let first_shape = board.get(Board::get_index(row, col)).unwrap();
            let next_shape = board.get(Board::get_index(row + 1, col)).unwrap();
            let next_next_shape = board.get(Board::get_index(row + 2, col)).unwrap();

            let shapes = shape_q
                .get_many([*first_shape, *next_shape, *next_next_shape])
                .unwrap();

            if *shapes[0] == *shapes[1] && *shapes[0] == *shapes[2] {
                matches.push([*first_shape, *next_shape, *next_next_shape])
            }
        }
    }

    return matches;
}

mod shape  {
    use bevy::{color::palettes::tailwind::*, prelude::*};
    use rand::seq::SliceRandom;

    #[derive(Component, Reflect, Clone, Copy, PartialEq)]
    pub enum Shape {
        Red,
        Blue,
        Green,
        Pink,
    }

    impl Shape {
        pub fn color(&self) -> BackgroundColor {
            match self {
                Shape::Red => RED_500.into(),
                Shape::Blue => BLUE_500.into(),
                Shape::Green => GREEN_500.into(),
                Shape::Pink => PINK_500.into(),
            }
        }
    }

    pub fn get_random_shape() -> Shape {
        let mut rng = rand::thread_rng();
        let colors = [Shape::Red, Shape::Pink, Shape::Blue, Shape::Green];
        let random_color = *colors.choose(&mut rng).unwrap();

        return random_color;
    }

    pub fn create_shape(shape: Shape) -> (Shape, ButtonBundle) {
        (
            shape,
            ButtonBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    ..default()
                },
                background_color: shape.color(),
                ..default()
            },
        )
    }
}
