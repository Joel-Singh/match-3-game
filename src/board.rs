use bevy::{color::palettes::tailwind::*, prelude::*};

#[derive(Component)]
pub struct Board;

use match_counter::MatchCounter;
use shape::*;

use crate::{GameState, TotalMatches};

#[derive(Event, Default)]
pub struct MatchMade();

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
    app
        .add_event::<MatchMade>()
        .add_systems(OnEnter(GameState::Board), (
            spawn_board,
            match_counter::spawn,
            spawn_shapes_into_board,
            layout_nodes
        ).chain())
        .add_systems(FixedUpdate, (
            swap_shapes_on_press,
            (handle_bomb_matches, handle_regular_matches).chain(),
            update_shape_color,
            match_counter::update
        ).run_if(in_state(GameState::Board)))
        .add_systems(OnExit(GameState::Board), (
            delete_entities,
            match_counter::delete_entities,
            reset_total_matches
        ));
}

fn layout_nodes(
    board: Query<Entity, With<Board>>,
    match_counter: Query<Entity, With<MatchCounter>>,
    mut commands: Commands
) {
    let mut container = commands.spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        ..default()
    });

    container.add_child(board.single());
    container.add_child(match_counter.single());
}

pub fn spawn_board(mut commands: Commands) {
    commands
        .spawn((
            Board,
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
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                background_color: Srgba::new(1.0, 1.0, 1.0, 0.1).into(),
                ..default()
            },
        ))
        .insert(BOARD_POSITION);
}


fn spawn_shapes_into_board(mut board: Query<Entity, With<Board>>, mut commands: Commands) {
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
                let mut board_children = board_children_q.single_mut();
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

fn handle_bomb_matches(
    board: Query<&Children, With<Board>>,
    shape_q: Query<&Shape>,
    mut commands: Commands,
    mut match_made: EventWriter<MatchMade>
) {
    let board = board.single();
    let bomb_matches = get_bomb_matches(board, &shape_q);

    for bomb_match in bomb_matches {
        for shape in bomb_match.matched_shapes {
            commands.entity(shape).insert(get_random_shape());
        }
        commands.entity(bomb_match.center).insert(Shape::Bomb);
        match_made.send(MatchMade::default());
    }
}

fn handle_regular_matches(
    board: Query<&Children, With<Board>>,
    shape_q: Query<&Shape>,
    mut commands: Commands,
    mut match_made: EventWriter<MatchMade>
) {
    let board = board.single();
    let matches = get_matches(board, &shape_q);

    for board_match in matches {
        let commands: &mut Commands = &mut commands;
        for entity in board_match {
            commands.entity(entity).insert(get_random_shape());
        }
        match_made.send(MatchMade::default());
    }
}


struct BombMatch {
    center: Entity,
    matched_shapes: Vec<Entity>,
}

fn get_bomb_matches(board: &Children, shape_q: &Query<&Shape>) -> Vec<BombMatch> {
    let get = |row: i32, col: i32| {
        if row < 1 || row > BOARD_SIZE as i32 || col < 1 || col > BOARD_SIZE as i32 {
            return None;
        }

        return board.get(Board::get_index(row as usize, col as usize));
    };

    let all_the_same_color = |shapes: &[&Shape]| {
        let first_shape = shapes[0];
        for shape in shapes {
            if *shape != first_shape {
                return false;
            }
        }
        return true
    };

    let mut matches: Vec<BombMatch> = vec![];
    for row in 1..=BOARD_SIZE {
        for col in 1..=BOARD_SIZE {
            let row = row as i32;
            let col = col as i32;

            let neighbors = [
                get(row, col),
                get(row, col - 1),
                get(row, col - 2),
                get(row + 1, col),
                get(row + 2, col),
            ];

            if neighbors.iter().any(|s| s.is_none()) {
                continue;
            }

            let neighbors = neighbors.map(|s| *s.unwrap());

            let shapes = shape_q.many(neighbors);

            if all_the_same_color(&shapes[..]) {
                matches.push(BombMatch {
                    center: neighbors[0],
                    matched_shapes: neighbors[1..].to_vec(), // Exclude the center
                });
            }
        }
    }

    matches 
}

fn get_matches(board: &Children, shape_q: &Query<&Shape>) -> Vec<[Entity; 3]> {
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


fn delete_entities(
    mut commands: Commands,
    board: Query<Entity, With<Board>>,
) {
    commands.entity(board.single()).despawn_recursive();
}

fn reset_total_matches(mut total_matches: ResMut<TotalMatches>) {
    total_matches.0 = 0;
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
        Bomb,
    }

    impl Shape {
        pub fn color(&self) -> BackgroundColor {
            match self {
                Shape::Red => RED_500.into(),
                Shape::Blue => BLUE_500.into(),
                Shape::Green => GREEN_500.into(),
                Shape::Pink => PINK_500.into(),
                Shape::Bomb => GRAY_950.into(),
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

mod match_counter {
    use bevy::{color::palettes::css::WHITE, prelude::*};

    use crate::{NeededMatches, TotalMatches};

    #[derive(Component)]
    pub struct MatchCounter;

    pub fn spawn(mut commands: Commands) {
        commands.spawn((
            MatchCounter,
            TextBundle::from_section(
                "0",
                TextStyle {
                    font_size: 100.0,
                    color: WHITE.into(),
                    ..default()
                }
            )
        ));
    }

    pub fn update(
        total_matches: Res<TotalMatches>,
        mut match_counter_text: Query<&mut Text, With<MatchCounter>>,
        needed_matches: Res<NeededMatches>
    ) {
        let mut text = match_counter_text.single_mut();
        text.sections[0].value = total_matches.0.to_string() + "/" + &needed_matches.0.to_string();
    }

    pub fn delete_entities(
        mut commands: Commands, match_counter: Query<Entity, With<MatchCounter>>
    ) {
        commands.entity(match_counter.single()).despawn_recursive();
    }
}
