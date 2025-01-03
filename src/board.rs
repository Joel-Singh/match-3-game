use bevy::{color::palettes::tailwind::*, prelude::*};
use rand::prelude::*;
use std::iter::zip;

#[derive(Component)]
pub struct Board;

use match_counter::MatchCounter;
use shape::*;

use crate::{CurrentMap, GameState, MapFinishes, TotalMatches};

use utils::*;

#[derive(Event, Default)]
pub struct MatchMade();

#[derive(Event)]
pub struct SwapShapes(Entity, Entity);

#[derive(Component)]
pub struct Deletion;

#[derive(Resource)]
pub struct JustSwappedShapes([Entity; 2]);

#[derive(Component)]
pub struct BoardNodeRoot;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum BoardState {
    #[default]
    InPlay,
    AnimatingFallingShapes,
}

const BOARD_POSITION: Transform = Transform::from_xyz(-200.0, 200.0, 0.0);
pub const BOARD_SIZE: usize = 5;
const BOARD_TOTAL_SHAPES: usize = BOARD_SIZE * BOARD_SIZE;

pub(crate) fn board(app: &mut App) {
    app.add_event::<SwapShapes>()
        .add_event::<MatchMade>()
        .init_state::<BoardState>()
        .insert_resource(JustSwappedShapes([
            Entity::from_raw(0),
            Entity::from_raw(0),
        ]))
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
                    (
                        write_swap_shape_event,
                        handle_swap_shape_events,
                        spawn_eliminators_from_matches.run_if(eliminator_unlocked),
                        spawn_bombs_from_matches.run_if(bomb_unlocked),
                        spawn_liners_from_matches.run_if(liner_unlocked),
                        handle_regular_matches,
                        update_board_after_deletions,
                    )
                        .chain()
                        .run_if(in_state(BoardState::InPlay)),
                    animate_shape_bottoms_to_0.run_if(in_state(BoardState::AnimatingFallingShapes)),
                )
                    .chain(),
                update_shape_color,
                match_counter::update,
            )
                .run_if(in_state(GameState::Board)),
        )
        .add_systems(
            OnExit(GameState::Board),
            (delete_entities, reset_total_matches, update_map_finishes),
        );
}

fn eliminator_unlocked(map_finishes: Res<MapFinishes>) -> bool {
    map_finishes.map3
}

fn bomb_unlocked(map_finishes: Res<MapFinishes>) -> bool {
    map_finishes.map1
}

fn liner_unlocked(map_finishes: Res<MapFinishes>) -> bool {
    map_finishes.map2
}

fn layout_nodes(
    board: Query<Entity, With<Board>>,
    match_counter: Query<Entity, With<MatchCounter>>,
    mut commands: Commands,
) {
    let mut root = commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        Name::new("Board and match counter container"),
        BoardNodeRoot,
    ));

    root.add_child(board.single());
    root.add_child(match_counter.single());
}

pub fn spawn_board(mut commands: Commands) {
    commands
        .spawn((Board, get_board_styling(), Name::new("Board")))
        .insert(BOARD_POSITION);
}

pub fn get_board_styling() -> (Node, BackgroundColor) {
    (
        Node {
            width: Val::Px(400.),
            height: Val::Px(400.),
            padding: UiRect::all(Val::Px(5.)),
            grid_template_columns: RepeatedGridTrack::fr(BOARD_SIZE as u16, 1.0),
            grid_template_rows: RepeatedGridTrack::fr(BOARD_SIZE as u16, 1.0),
            display: Display::Grid,
            margin: UiRect::all(Val::Auto),
            overflow: Overflow::clip(),
            ..default()
        },
        BackgroundColor(Srgba::new(1.0, 1.0, 1.0, 0.1).into()),
    )
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
    mut just_swapped_shapes: ResMut<JustSwappedShapes>,
    mut swap_shapes: EventReader<SwapShapes>,
    mut commands: Commands,
) {
    for SwapShapes(button1, button2) in swap_shapes.read() {
        let mut board_children = board_children.single_mut();

        let is_next_to = is_next_to(button1, button2, &board_children);
        if is_next_to {
            swap(*button1, *button2, &mut board_children);
            just_swapped_shapes.0 = [*button1, *button2];
        }

        for b in [button1, button2] {
            let entity = *b;
            let shape = *shapes.get(entity).unwrap();

            take_action_for_special(
                explode_bomb,
                Shape::Bomb,
                (shape, entity),
                &board_children,
                &mut commands,
            );

            take_action_for_special(
                remove_line,
                Shape::HorizontalLiner,
                (shape, entity),
                &board_children,
                &mut commands,
            );

            take_action_for_special(
                remove_line,
                Shape::VerticalLiner,
                (shape, entity),
                &board_children,
                &mut commands,
            );

            take_action_for_special(
                eliminate,
                Shape::Eliminator,
                (shape, entity),
                &board_children,
                &mut commands,
            );
        }

        type SpecialShapeCallback = fn((Shape, Entity), &Children, &mut Commands);
        fn take_action_for_special(
            action: SpecialShapeCallback,
            special_shape: Shape,
            shape: (Shape, Entity),
            board_children: &Children,
            commands: &mut Commands,
        ) {
            if shape.0 == special_shape {
                action(shape, board_children, commands);
            }
        }

        fn eliminate((_, e): (Shape, Entity), board: &Children, commands: &mut Commands) {
            remove_random_shapes(board, commands);

            commands.entity(e).insert(Deletion);

            fn remove_random_shapes(board: &Children, commands: &mut Commands) {
                let board = board.iter().collect::<Vec<_>>();
                let random_shapes = board
                    .choose_multiple(&mut rand::thread_rng(), BOARD_SIZE * 3)
                    .map(|e| **e)
                    .collect::<Vec<_>>();

                for shape in random_shapes {
                    commands.entity(shape).insert(Deletion);
                }
            }
        }

        fn remove_line(
            (shape, entity): (Shape, Entity),
            board_children: &Children,
            commands: &mut Commands,
        ) {
            remove_cells_in_a_line(
                &entity,
                shape == Shape::HorizontalLiner,
                board_children,
                commands,
            );
            commands.entity(entity).insert(Deletion);

            fn remove_cells_in_a_line(
                liner: &Entity,
                is_horizontal: bool,
                board_children: &Children,
                commands: &mut Commands,
            ) {
                let (row, col) = get_row_col(liner, board_children);

                let line_of_shapes =
                    get_line_of_shapes(board_children, row as i32, col as i32, is_horizontal);

                for shape in line_of_shapes {
                    commands.entity(shape).insert(Deletion);
                }

                fn get_line_of_shapes(
                    board_children: &Children,
                    row: i32,
                    col: i32,
                    is_horizontal: bool,
                ) -> Vec<Entity> {
                    let mut locations: Vec<(i32, i32)>;
                    if is_horizontal {
                        locations = (1..=BOARD_SIZE).map(|c| (row, c as i32)).collect();
                    } else {
                        locations = (1..=BOARD_SIZE).map(|r| (r as i32, col)).collect();
                    }

                    locations.retain(|&x| x != (row, col));

                    let line_of_shapes = locations
                        .iter()
                        .filter_map(|(r, c)| get_entity(*r, *c, board_children))
                        .copied()
                        .collect::<Vec<_>>();

                    line_of_shapes
                }
            }
        }

        fn explode_bomb(
            (_, entity): (Shape, Entity),
            board_children: &Children,
            commands: &mut Commands,
        ) {
            explode_surrounding_cells(&entity, board_children, commands);
            commands.entity(entity).insert(Deletion);
        }

        fn explode_surrounding_cells(
            bomb: &Entity,
            board_children: &Children,
            commands: &mut Commands,
        ) {
            let (row, col) = get_row_col(bomb, board_children);

            let surrounding_shapes = get_surrounding_shapes(board_children, row as i32, col as i32);
            for shape in surrounding_shapes {
                commands.entity(shape).insert(Deletion);
            }

            fn get_surrounding_shapes(
                board_children: &Children,
                row: i32,
                col: i32,
            ) -> Vec<Entity> {
                let surrounding_shapes = [
                    get_entity(row - 1, col - 1, board_children),
                    get_entity(row - 1, col, board_children),
                    get_entity(row - 1, col + 1, board_children),
                    get_entity(row, col - 1, board_children),
                    get_entity(row, col + 1, board_children),
                    get_entity(row + 1, col - 1, board_children),
                    get_entity(row + 1, col, board_children),
                    get_entity(row + 1, col + 1, board_children),
                ];

                surrounding_shapes
                    .iter()
                    .filter(|s| s.is_some())
                    .map(|s| *s.unwrap())
                    .collect()
            }
        }

        fn swap(entity1: Entity, entity2: Entity, children: &mut Children) {
            let index1 = children.iter().position(|&e| e == entity1).unwrap();
            let index2 = children.iter().position(|&e| e == entity2).unwrap();
            children.swap(index1, index2);
        }
    }

    fn is_next_to(
        last_pressed_button: &Entity,
        just_pressed_button: &Entity,
        children: &Children,
    ) -> bool {
        let (x_1, y_1) = get_row_col(last_pressed_button, children);
        let (x_2, y_2) = get_row_col(just_pressed_button, children);
        let delta_x = (x_1 as i32 - x_2 as i32).abs();
        let delta_y = (y_1 as i32 - y_2 as i32).abs();

        (delta_x + delta_y) == 1
    }
}

fn update_shape_color(
    mut shape: Query<(&Shape, Entity), Or<(Changed<Shape>, Changed<Deletion>)>>,
    shapes_being_deleted: Query<&Deletion>,
    mut commands: Commands,
) {
    for (shape, e) in shape.iter_mut() {
        match shapes_being_deleted.get(e) {
            Ok(_) => {
                commands
                    .entity(e)
                    .entry::<BackgroundColor>()
                    .and_modify(|mut bg| {
                        bg.0 = Color::srgba(0.0, 0.0, 0.0, 0.0);
                    });
            }
            Err(_) => {
                commands.entity(e).insert(shape.color());
            }
        };
    }
}

fn spawn_eliminators_from_matches(
    board: Query<&Children, With<Board>>,
    shape_q: Query<&Shape>,
    deleted_shapes_q: Query<&Deletion>,
    mut commands: Commands,
    mut match_made: EventWriter<MatchMade>,
) {
    let board = board.single();
    let matches = get_matches_eliminator(board, &shape_q, &deleted_shapes_q);

    for r#match in matches {
        for entity in r#match.matched_shapes {
            commands.entity(entity).insert(Deletion);
        }
        commands.entity(r#match.center).insert(Shape::Eliminator);

        match_made.send(MatchMade::default());
    }

    fn get_matches_eliminator(
        board: &Children,
        shape_q: &Query<&Shape>,
        deleted_shapes_q: &Query<&Deletion>,
    ) -> Vec<Match> {
        let mut horizontal_matches = get_matches_general(
            board,
            shape_q,
            deleted_shapes_q,
            [(0, -1), (0, -2), (0, 1), (0, 2)],
        );

        let mut vertical_matches = get_matches_general(
            board,
            shape_q,
            deleted_shapes_q,
            [(-1, 0), (-2, 0), (1, 0), (2, 0)],
        );

        horizontal_matches.append(&mut vertical_matches);

        horizontal_matches
    }
}

fn spawn_liners_from_matches(
    board: Query<&Children, With<Board>>,
    shape_q: Query<&Shape>,
    deleted_shapes_q: Query<&Deletion>,
    just_swapped_shapes: Res<JustSwappedShapes>,
    mut commands: Commands,
    mut match_made: EventWriter<MatchMade>,
) {
    let board = board.single();
    let (horizontal_matches, vertical_matches) =
        get_matches_liner(board, &shape_q, &deleted_shapes_q);

    for board_match in horizontal_matches.iter().chain(vertical_matches.iter()) {
        for entity in &board_match.matched_shapes {
            commands.entity(*entity).insert(Deletion);
        }
        commands.entity(board_match.center).insert(Deletion);

        match_made.send(MatchMade::default());
    }

    let matches_and_shapes = [
        (&vertical_matches, Shape::VerticalLiner),
        (&horizontal_matches, Shape::HorizontalLiner),
    ];

    for (matches, shape) in matches_and_shapes.iter() {
        for r#match in *matches {
            let mut all_shapes = r#match.matched_shapes.clone();
            all_shapes.push(r#match.center);

            let just_swapped_shape_in_liner_match = all_shapes.iter().find(|e| {
                let just_swapped_shapes = just_swapped_shapes.0;
                *e == &just_swapped_shapes[0] || *e == &just_swapped_shapes[1]
            });

            if let Some(just_swapped_shape_in_liner_match) = just_swapped_shape_in_liner_match {
                commands
                    .entity(*just_swapped_shape_in_liner_match)
                    .insert(*shape);
                commands
                    .entity(*just_swapped_shape_in_liner_match)
                    .remove::<Deletion>();
            } else {
                commands.entity(r#match.center).insert(Shape::VerticalLiner);
                commands.entity(r#match.center).remove::<Deletion>();
            }
        }
    }

    // 4 in a row
    fn get_matches_liner(
        board: &Children,
        shape_q: &Query<&Shape>,
        deleted_shapes_q: &Query<&Deletion>,
    ) -> (Vec<Match>, Vec<Match>) {
        let horizontal_matches =
            get_matches_general(board, shape_q, deleted_shapes_q, [(0, 1), (0, 2), (0, 3)]);

        let vertical_matches =
            get_matches_general(board, shape_q, deleted_shapes_q, [(1, 0), (2, 0), (3, 0)]);

        (horizontal_matches, vertical_matches)
    }
}

fn spawn_bombs_from_matches(
    board: Query<&Children, With<Board>>,
    shape_q: Query<&Shape>,
    deleted_shapes_q: Query<&Deletion>,
    mut commands: Commands,
    mut match_made: EventWriter<MatchMade>,
) {
    let board = board.single();
    let bomb_matches = get_bomb_matches(board, &shape_q, &deleted_shapes_q);

    for bomb_match in bomb_matches {
        for shape in bomb_match.matched_shapes {
            commands.entity(shape).insert(Deletion);
        }
        commands.entity(bomb_match.center).insert(Shape::Bomb);
        match_made.send(MatchMade::default());
    }
}

fn handle_regular_matches(
    board: Query<&Children, With<Board>>,
    shape_q: Query<&Shape>,
    deleted_shapes_q: Query<&Deletion>,
    mut commands: Commands,
    mut match_made: EventWriter<MatchMade>,
) {
    let board = board.single();
    let matches = get_matches_three(board, &shape_q, &deleted_shapes_q);

    for board_match in matches {
        let commands: &mut Commands = &mut commands;
        for entity in board_match.matched_shapes {
            commands.entity(entity).insert(Deletion);
        }
        commands.entity(board_match.center).insert(Deletion);
        match_made.send(MatchMade::default());
    }
}

fn animate_shape_bottoms_to_0(
    mut shapes_node_q: Query<&mut Node, With<Shape>>,
    mut state: ResMut<NextState<BoardState>>,
    time: Res<Time>,
) {
    for mut shape in shapes_node_q.iter_mut() {
        let delta_seconds = time.delta_secs();
        const FALLING_SPEED: f32 = 300.0;
        if let Val::Percent(bottom) = shape.bottom {
            let new_percentage = bottom - delta_seconds * FALLING_SPEED;
            shape.bottom = Val::Percent(new_percentage.max(0 as f32));
        }
    }

    let all_shapes_have_fallen = shapes_node_q.iter().all(|node| {
        if let Val::Percent(bottom) = node.bottom {
            0 as f32 == bottom
        } else {
            panic!("Expected Val::Percent for node top")
        }
    });

    if all_shapes_have_fallen {
        state.set(BoardState::InPlay);
    }
}

fn update_board_after_deletions(
    deleted_shapes_q: Query<Entity, With<Deletion>>,
    board_children: Query<&Children, With<Board>>,
    board: Query<Entity, With<Board>>,
    mut state: ResMut<NextState<BoardState>>,
    mut commands: Commands,
) {
    if deleted_shapes_q.iter().count() == 0 {
        return;
    } else {
        state.set(BoardState::AnimatingFallingShapes);
    }

    let board_children = board_children.single();
    let board = board.single();

    for shape in deleted_shapes_q.iter() {
        commands.entity(shape).remove::<Deletion>();
        commands.entity(shape).insert(get_random_shape());
    }

    let mut new_board_state = board_children.iter().copied().collect::<Vec<_>>();
    for col in 1..=BOARD_SIZE {
        let mut original_column: Vec<Entity> = vec![];
        for row in 1..=BOARD_SIZE {
            let e = get_entity(row as i32, col as i32, board_children).unwrap();
            original_column.push(*e);
        }

        let mut new_column: Vec<Entity> = vec![];
        for e in original_column.clone() {
            if deleted_shapes_q.get(e).is_ok() {
                new_column.insert(0, e);
            } else {
                new_column.push(e);
            }
        }

        for shape in original_column.clone() {
            let original_row = original_column.iter().position(|&e| e == shape).unwrap() + 1;
            let new_row = new_column.iter().position(|&e| e == shape).unwrap() + 1;
            let num_of_deleted_shapes = original_column
                .clone()
                .iter()
                .filter(|&e| deleted_shapes_q.get(*e).is_ok())
                .count();

            if deleted_shapes_q.get(shape).is_err() {
                commands
                    .entity(shape)
                    .entry::<Node>()
                    .and_modify(move |mut node| {
                        node.bottom = Val::Percent((new_row as f32 - original_row as f32) * 100.0);
                    });
            } else {
                commands
                    .entity(shape)
                    .entry::<Node>()
                    .and_modify(move |mut node| {
                        node.bottom = Val::Percent(num_of_deleted_shapes as f32 * 100.0);
                    });
            }
        }

        for (shape, row) in zip(new_column, 1..=BOARD_SIZE) {
            new_board_state[get_index(row as i32, col as i32).unwrap() as usize] = shape;
        }
    }

    commands
        .entity(board)
        .replace_children(&new_board_state[..]);
}

#[derive(Debug)]
struct Match {
    center: Entity,
    matched_shapes: Vec<Entity>,
}

fn get_matches_general<const N: usize>(
    board: &Children,
    shape_q: &Query<&Shape>,
    deleted_shapes: &Query<&Deletion>,
    neighbors: [(i32, i32); N],
) -> Vec<Match> {
    let all_the_same_color = |shapes: &[&Shape]| {
        let first_shape = shapes[0];
        for shape in shapes {
            if *shape != first_shape {
                return false;
            }
        }
        true
    };

    let not_already_matched = |shapes: &[Entity]| {
        for shape in shapes {
            if deleted_shapes.get(*shape).is_ok() {
                return false;
            }
        }
        true
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

            let mut entities = neighbors[..].to_vec();
            entities.push(center_entity);

            let not_special = shapes.iter().all(|s| !s.is_special());
            if all_the_same_color(&shapes[..]) && not_already_matched(&entities[..]) && not_special
            {
                matches.push(Match {
                    center: center_entity,
                    matched_shapes: neighbors[..].to_vec(),
                });
            }
        }
    }
    matches
}

// Matches in an L shape
fn get_bomb_matches(
    board: &Children,
    shape_q: &Query<&Shape>,
    deleted_shapes_q: &Query<&Deletion>,
) -> Vec<Match> {
    let above = (-1, 0);
    let above_2 = (-2, 0);
    let below = (1, 0);
    let below_2 = (2, 0);
    let left = (0, -1);
    let left_2 = (0, -2);
    let right = (0, 1);
    let right_2 = (0, 2);

    let get_matches = |neighbors: [(i32, i32); 4]| {
        get_matches_general(board, shape_q, deleted_shapes_q, neighbors)
    };

    let mut matches: Vec<Match> = vec![];

    matches.extend(get_matches([left, left_2, below, below_2]));
    matches.extend(get_matches([left, left_2, above, above_2]));
    matches.extend(get_matches([right, right_2, below, below_2]));
    matches.extend(get_matches([right, right_2, above, above_2]));

    matches
}

fn get_matches_three(
    board: &Children,
    shape_q: &Query<&Shape>,
    deleted_shapes_q: &Query<&Deletion>,
) -> Vec<Match> {
    let mut matches: Vec<Match> = vec![];

    matches.extend(get_matches_general(
        board,
        shape_q,
        deleted_shapes_q,
        [(0, 1), (0, 2)],
    ));
    matches.extend(get_matches_general(
        board,
        shape_q,
        deleted_shapes_q,
        [(1, 0), (2, 0)],
    ));

    matches
}

fn delete_entities(mut commands: Commands, board: Query<Entity, With<BoardNodeRoot>>) {
    commands.entity(board.single()).despawn_recursive();
}

fn reset_total_matches(mut total_matches: ResMut<TotalMatches>) {
    total_matches.0 = 0;
}

fn update_map_finishes(mut map_finishes: ResMut<MapFinishes>, current_map: Res<CurrentMap>) {
    match current_map.get() {
        CurrentMap::One => map_finishes.map1 = true,
        CurrentMap::Two => map_finishes.map2 = true,
        CurrentMap::Three => map_finishes.map3 = true,
        CurrentMap::Four => map_finishes.map4 = true,
        CurrentMap::None => {}
    }
}

pub mod utils {
    use crate::board::BOARD_SIZE;
    use bevy::prelude::*;

    pub fn get_entity(row: i32, col: i32, board: &Children) -> Option<&Entity> {
        match get_index(row, col) {
            Some(index) => board.get(index as usize),
            None => None,
        }
    }

    pub fn get_index(row: i32, col: i32) -> Option<i32> {
        if row < 1 || col < 1 || row > BOARD_SIZE as i32 || col > BOARD_SIZE as i32 {
            return None;
        }

        Some((((row - 1) * BOARD_SIZE as i32) + col) - 1)
    }

    // From the top-left, e.g rows are 1, 2, 3 going down. Columns are 1, 2, 3 going right.
    pub fn get_row_col(shape: &Entity, board: &Children) -> (usize, usize) {
        let index = board.iter().position(|&e| e == *shape).unwrap();
        let row = index / BOARD_SIZE + 1;
        let col = (index % BOARD_SIZE) + 1;
        (row, col)
    }
}

pub mod shape {
    use bevy::{color::palettes::tailwind::*, prelude::*};
    use rand::seq::SliceRandom;

    use super::get_shape_styling;

    #[derive(Component, Reflect, Clone, Copy, PartialEq, Debug)]
    #[require(Button, Node, BackgroundColor)]
    pub enum Shape {
        Red,
        Blue,
        Green,
        Pink,
        Bomb,
        HorizontalLiner,
        VerticalLiner,
        Eliminator,
    }

    impl Shape {
        pub fn color(&self) -> BackgroundColor {
            match self {
                Shape::Red => RED_500.into(),
                Shape::Blue => BLUE_500.into(),
                Shape::Green => GREEN_500.into(),
                Shape::Pink => PINK_500.into(),
                Shape::Bomb => GRAY_950.into(),
                Shape::HorizontalLiner | Shape::VerticalLiner => YELLOW_500.into(),
                Shape::Eliminator => PURPLE_500.into(),
            }
        }

        pub fn is_special(&self) -> bool {
            match self {
                Shape::Bomb | Shape::HorizontalLiner | Shape::VerticalLiner | Shape::Eliminator => {
                    true
                }
                Shape::Red | Shape::Blue | Shape::Green | Shape::Pink => false,
            }
        }
    }

    pub fn get_random_shape() -> Shape {
        let mut rng = rand::thread_rng();
        let colors = [Shape::Red, Shape::Pink, Shape::Blue, Shape::Green];
        let random_color = *colors.choose(&mut rng).unwrap();

        random_color
    }

    pub fn create_shape(shape: Shape) -> (Shape, Button, (Node, BackgroundColor), Name) {
        (shape, Button, get_shape_styling(shape), Name::new("Shape"))
    }
}

pub fn get_shape_styling(shape: Shape) -> (Node, BackgroundColor) {
    (
        Node {
            width: Val::Auto,
            height: Val::Auto,
            margin: UiRect::all(Val::Px(2.)),
            bottom: Val::Percent(0.0),
            ..default()
        },
        shape.color(),
    )
}

mod match_counter {
    use bevy::{color::palettes::css::WHITE, prelude::*};

    use crate::{NeededMatches, TotalMatches};

    #[derive(Component)]
    pub struct MatchCounter;

    pub fn spawn(mut commands: Commands) {
        commands.spawn((
            MatchCounter,
            Text::new("0"),
            TextFont {
                font_size: 100.0,
                ..default()
            },
            TextColor(WHITE.into()),
        ));
    }

    pub fn update(
        total_matches: Res<TotalMatches>,
        mut match_counter_text: Query<&mut Text, With<MatchCounter>>,
        needed_matches: Res<NeededMatches>,
    ) {
        let mut text = match_counter_text.single_mut();
        text.0 = total_matches.0.to_string() + "/" + &needed_matches.0.to_string();
    }
}
