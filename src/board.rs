use bevy::{color::palettes::tailwind::*, prelude::*};

#[derive(Component)]
pub struct Board;

use match_counter::MatchCounter;
use shape::*;

use crate::{GameState, TotalMatches};

#[derive(Event, Default)]
pub struct MatchMade();

#[derive(Event)]
pub struct SwapShapes(Entity, Entity);

impl Board {
    fn get_entity(row: usize, col: usize, board: &Children) -> Option<&Entity> {
        match Board::get_index(row, col) {
            Some(index) => board.get(index),
            None => None,
        }
    }

    fn get_index(row: usize, col: usize) -> Option<usize> {
        if row < 1 || col < 1 ||  row > BOARD_SIZE || col > BOARD_SIZE {
            return None;
        }

        Some(((((row - 1) * BOARD_SIZE) + col) - 1) as usize)
    }

    fn get_row_col(index: usize) -> (usize, usize) {
        let row = index / BOARD_SIZE + 1;
        let col = (index % BOARD_SIZE) + 1;
        return (row, col);
    }
}

const BOARD_POSITION: Transform = Transform::from_xyz(-200.0, 200.0, 0.0);
const BOARD_SIZE: usize = 10;
const BOARD_TOTAL_SHAPES: usize = BOARD_SIZE * BOARD_SIZE;

pub(crate) fn board(app: &mut App) {
    app.add_event::<SwapShapes>()
        .add_event::<MatchMade>()
        .add_systems(
            OnEnter(GameState::Board),
            (
                spawn_board,
                match_counter::spawn,
                spawn_shapes_into_board,
                layout_nodes,
            )
                .chain(),
        )
        .add_systems(
            FixedUpdate,
            (
                (write_swap_shape_event, handle_swap_shape_events, handle_bomb_matches, handle_regular_matches).chain(),
                update_shape_color,
                match_counter::update,
            )
                .run_if(in_state(GameState::Board)),
        )
        .add_systems(
            OnExit(GameState::Board),
            (
                delete_entities,
                match_counter::delete_entities,
                reset_total_matches,
            ),
        );
}

fn layout_nodes(
    board: Query<Entity, With<Board>>,
    match_counter: Query<Entity, With<MatchCounter>>,
    mut commands: Commands,
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

fn write_swap_shape_event(
    mut interaction_query: Query<
        (&Interaction, Entity),
        (Changed<Interaction>, With<Button>, With<Shape>),
    >,
    mut last_pressed_button: Local<Option<Entity>>,
    mut commands: Commands,
    mut swap_shapes_event: EventWriter<SwapShapes>,
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
            }
            Some(last_pressed_button_e) => {
                swap_shapes_event.send(SwapShapes(last_pressed_button_e, just_pressed_button));

                commands
                    .entity(last_pressed_button_e)
                    .insert(Outline::default());

                *last_pressed_button = None;
            }
        }
    }
}

fn handle_swap_shape_events(
    mut board_children: Query<&mut Children, With<Board>>,
    shapes: Query<&Shape>,
    mut swap_shapes: EventReader<SwapShapes>,
    mut commands: Commands,
) {
    for SwapShapes(button1, button2) in swap_shapes.read() {
        let mut board_children = board_children.single_mut();

        let button1_i = get_index(&board_children, *button1);
        let button2_i = get_index(&board_children, *button2);

        let is_next_to = is_next_to(button1_i, button2_i);
        if is_next_to {
            board_children.swap(button1_i, button2_i);
        }

        explode_if_bomb((*shapes.get(*button1).unwrap(), *button1), &board_children, &mut commands);
        explode_if_bomb((*shapes.get(*button2).unwrap(), *button2), &board_children, &mut commands);

        fn explode_if_bomb(shape: (Shape, Entity), board_children: &Children,  commands: &mut Commands) {
            let (maybe_bomb, shape_e) = shape;
            if maybe_bomb == Shape::Bomb {
                explode_surrounding_cells(&shape_e, &board_children, commands);
                randomize_shape(&shape_e, commands);
            }
        }

        fn explode_surrounding_cells(
            bomb: &Entity,
            board_children: &Children,
            commands: &mut Commands,
        ) {
            let board_index = get_index(board_children, *bomb);
            let (row, col) = Board::get_row_col(board_index);

            let surrounding_shapes = get_surrounding_shapes(board_children, row, col);
            for shape in surrounding_shapes {
                randomize_shape(&shape, commands)
            }
        }

        fn randomize_shape(shape: &Entity, commands: &mut Commands) {
            commands.entity(*shape).insert(get_random_shape());
        }

        fn get_surrounding_shapes(board_children: &Children, row: usize, col: usize) -> Vec<Entity> {
            let surrounding_shapes = [
                Board::get_entity(row - 1, col - 1, &board_children),
                Board::get_entity(row - 1, col, &board_children),
                Board::get_entity(row - 1, col + 1, &board_children),
                Board::get_entity(row, col - 1, &board_children),
                Board::get_entity(row, col + 1, &board_children),
                Board::get_entity(row + 1, col - 1, &board_children),
                Board::get_entity(row + 1, col, &board_children),
                Board::get_entity(row + 1, col + 1, &board_children),
            ];

           surrounding_shapes
                .iter()
                .filter(|s| s.is_some())
                .map(|s| *s.unwrap())
                .collect()
        }
    }

    fn get_index(board_children: &Children, last_pressed_button_e: Entity) -> usize {
        board_children
            .iter()
            .position(|e| *e == last_pressed_button_e)
            .unwrap()
    }

    fn is_next_to(last_pressed_button_i: usize, just_pressed_button_i: usize) -> bool {
        let (x_1, y_1) = Board::get_row_col(last_pressed_button_i);
        let (x_2, y_2) = Board::get_row_col(just_pressed_button_i);
        let delta_x = (x_1 as i32 - x_2 as i32).abs();
        let delta_y = (y_1 as i32 - y_2 as i32).abs();

        let is_next_to = (delta_x + delta_y) == 1;
        is_next_to
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
    mut match_made: EventWriter<MatchMade>,
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
    mut match_made: EventWriter<MatchMade>,
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
        return Board::get_entity(row as usize, col as usize, board);
    };

    let all_the_same_color = |shapes: &[&Shape]| {
        let first_shape = shapes[0];
        for shape in shapes {
            if *shape != first_shape {
                return false;
            }
        }
        return true;
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
            let first_shape = Board::get_entity(row, col, board).unwrap();
            let next_shape = Board::get_entity(row, col + 1, board).unwrap();
            let next_next_shape = Board::get_entity(row, col + 2, board).unwrap();

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
            let first_shape = Board::get_entity(row, col, board).unwrap();
            let next_shape = Board::get_entity(row + 1, col, board).unwrap();
            let next_next_shape = Board::get_entity(row + 2, col, board).unwrap();

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

fn delete_entities(mut commands: Commands, board: Query<Entity, With<Board>>) {
    commands.entity(board.single()).despawn_recursive();
}

fn reset_total_matches(mut total_matches: ResMut<TotalMatches>) {
    total_matches.0 = 0;
}

mod shape {
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
                },
            ),
        ));
    }

    pub fn update(
        total_matches: Res<TotalMatches>,
        mut match_counter_text: Query<&mut Text, With<MatchCounter>>,
        needed_matches: Res<NeededMatches>,
    ) {
        let mut text = match_counter_text.single_mut();
        text.sections[0].value = total_matches.0.to_string() + "/" + &needed_matches.0.to_string();
    }

    pub fn delete_entities(
        mut commands: Commands,
        match_counter: Query<Entity, With<MatchCounter>>,
    ) {
        commands.entity(match_counter.single()).despawn_recursive();
    }
}
