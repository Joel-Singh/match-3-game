use bevy::{color::palettes::tailwind::*, prelude::*};

#[derive(Component)]
pub struct Board;

use match_counter::MatchCounter;
use shape::*;

use crate::{GameState, TotalMatches};

use utils::*;

#[derive(Event, Default)]
pub struct MatchMade();

#[derive(Event)]
pub struct SwapShapes(Entity, Entity);

mod utils {
    use crate::board::BOARD_SIZE;
    use bevy::prelude::*;

    pub fn get_entity(row: i32, col: i32, board: &Children) -> Option<&Entity> {
        fn get_index(row: i32, col: i32) -> Option<i32> {
            if row < 1 || col < 1 || row > BOARD_SIZE as i32 || col > BOARD_SIZE as i32 {
                return None;
            }

            Some((((row - 1) * BOARD_SIZE as i32) + col) - 1)
        }

        return match get_index(row, col) {
            Some(index) => board.get(index as usize),
            None => None,
        };
    }

    pub fn get_row_col(shape: &Entity, board: &Children) -> (usize, usize) {
        let index = board.iter().position(|&e| e == *shape).unwrap();
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
                (
                    write_swap_shape_event,
                    handle_swap_shape_events,
                    handle_bomb_matches,
                    handle_regular_matches,
                )
                    .chain(),
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

fn randomize_shape(shape: &Entity, commands: &mut Commands) {
    commands.entity(*shape).insert(get_random_shape());
}

fn handle_swap_shape_events(
    mut board_children: Query<&mut Children, With<Board>>,
    shapes: Query<&Shape>,
    mut swap_shapes: EventReader<SwapShapes>,
    mut commands: Commands,
) {
    for SwapShapes(button1, button2) in swap_shapes.read() {
        let mut board_children = board_children.single_mut();

        let is_next_to = is_next_to(button1, button2, &board_children);
        if is_next_to {
            swap(*button1, *button2, &mut board_children);
        }

        explode_if_bomb(
            (*shapes.get(*button1).unwrap(), *button1),
            &board_children,
            &mut commands,
        );
        explode_if_bomb(
            (*shapes.get(*button2).unwrap(), *button2),
            &board_children,
            &mut commands,
        );

        fn explode_if_bomb(
            shape: (Shape, Entity),
            board_children: &Children,
            commands: &mut Commands,
        ) {
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
            let (row, col) = get_row_col(bomb, board_children);

            let surrounding_shapes = get_surrounding_shapes(board_children, row as i32, col as i32);
            for shape in surrounding_shapes {
                randomize_shape(&shape, commands)
            }
        }

        fn swap(entity1: Entity, entity2: Entity, children: &mut Children) {
            let index1 = children.iter().position(|&e| e == entity1).unwrap();
            let index2 = children.iter().position(|&e| e == entity2).unwrap();
            children.swap(index1, index2);
        }

        fn get_surrounding_shapes(board_children: &Children, row: i32, col: i32) -> Vec<Entity> {
            let surrounding_shapes = [
                get_entity(row - 1, col - 1, &board_children),
                get_entity(row - 1, col, &board_children),
                get_entity(row - 1, col + 1, &board_children),
                get_entity(row, col - 1, &board_children),
                get_entity(row, col + 1, &board_children),
                get_entity(row + 1, col - 1, &board_children),
                get_entity(row + 1, col, &board_children),
                get_entity(row + 1, col + 1, &board_children),
            ];

            surrounding_shapes
                .iter()
                .filter(|s| s.is_some())
                .map(|s| *s.unwrap())
                .collect()
        }
    }

    fn is_next_to(
        last_pressed_button: &Entity,
        just_pressed_button: &Entity,
        children: &Children,
    ) -> bool {
        let (x_1, y_1) = get_row_col(last_pressed_button, &children);
        let (x_2, y_2) = get_row_col(just_pressed_button, &children);
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
            randomize_shape(&shape, &mut commands);
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
            randomize_shape(&entity, commands);
        }
        match_made.send(MatchMade::default());
    }
}

struct Match {
    center: Entity,
    matched_shapes: Vec<Entity>,
}

fn get_matches_general<const N: usize>(
    board: &Children,
    shape_q: &Query<&Shape>,
    neighbors: [(i32, i32); N],
) -> Vec<Match> {
    let all_the_same_color = |shapes: &[&Shape]| {
        let first_shape = shapes[0];
        for shape in shapes {
            if *shape != first_shape {
                return false;
            }
        }
        return true;
    };

    let mut matches: Vec<Match> = vec![];
    for row in 1..=BOARD_SIZE {
        for col in 1..=BOARD_SIZE {
            let row = row as i32;
            let col = col as i32;

            let neighbors = neighbors
                .iter()
                .map(|(row_offset, col_offset)| {
                    get_entity(row + row_offset, col + col_offset, board)
                })
                .collect::<Vec<_>>();

            let center = get_entity(row, col, board);

            if neighbors.iter().any(|s| s.is_none()) || center.is_none() {
                continue;
            }

            let neighbors: [Entity; N] = neighbors
                .iter()
                .map(|s| *s.unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let center_entity = *center.unwrap();

            let mut shapes = shape_q.many(neighbors).to_vec();
            let center_shape = shape_q.get(center_entity).unwrap();

            shapes.push(center_shape);

            if all_the_same_color(&shapes[..]) {
                matches.push(Match {
                    center: center_entity,
                    matched_shapes: neighbors[..].to_vec(),
                });
            }
        }
    }
    matches
}

fn get_bomb_matches(board: &Children, shape_q: &Query<&Shape>) -> Vec<Match> {
    let above = (-1, 0);
    let above_2 = (-2, 0);
    let below = (1, 0);
    let below_2 = (2, 0);
    let left = (0, -1);
    let left_2 = (0, -2);
    let right = (0, 1);
    let right_2 = (0, 2);
    let get_matches = |neighbors: [(i32, i32); 4]| get_matches_general(board, shape_q, neighbors);

    let mut matches: Vec<Match> = vec![];

    matches.extend(get_matches([left, left_2, below, below_2]));
    matches.extend(get_matches([left, left_2, above, above_2]));
    matches.extend(get_matches([right, right_2, below, below_2]));
    matches.extend(get_matches([right, right_2, above, above_2]));

    matches
}

fn get_matches(board: &Children, shape_q: &Query<&Shape>) -> Vec<[Entity; 3]> {
    let mut matches: Vec<[Entity; 3]> = vec![];

    // Check horizontally
    for row in 1..=BOARD_SIZE {
        for col in 1..=(BOARD_SIZE - 2) {
            let row = row as i32;
            let col = col as i32;

            let first_shape = get_entity(row, col, board).unwrap();
            let next_shape = get_entity(row, col + 1, board).unwrap();
            let next_next_shape = get_entity(row, col + 2, board).unwrap();

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
            let row = row as i32;
            let col = col as i32;

            let first_shape = get_entity(row, col, board).unwrap();
            let next_shape = get_entity(row + 1, col, board).unwrap();
            let next_next_shape = get_entity(row + 2, col, board).unwrap();

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
