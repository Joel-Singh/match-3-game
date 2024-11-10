use bevy::{color::palettes::tailwind::*, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::seq::SliceRandom;

#[derive(Component)]
struct Board(Vec<Entity>);

#[derive(Component, Clone, Copy)]
enum Shape {
    Red,
    Blue,
    Green,
    Pink,
    Empty,
}

const BOARD_POSITION: Transform = Transform::from_xyz(-200.0, 200.0, 0.0);
const BOARD_SIZE: i32 = 10;
const BOARD_TOTAL_SHAPES: i32 = BOARD_SIZE * BOARD_SIZE;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            bevy::log::LogPlugin {
                level: bevy::log::Level::DEBUG,
                ..default()
            }
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, (
            spawn_board,
            setup
        ).chain())
        .add_systems(FixedUpdate, swap_buttons_on_press)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_board( mut commands: Commands
) { 
    commands.spawn((
        Board(Vec::new()),
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
            background_color: Srgba::new(1.0,1.0,1.0,0.1).into(),
            ..default()
        },
    )).insert(BOARD_POSITION);
}

fn setup(
    mut board: Query<(&mut Board, Entity)>,
    mut commands: Commands
) {
    let (mut board, board_entity) = board.get_single_mut().unwrap();

    for _ in 0..BOARD_TOTAL_SHAPES {
        let mut rng = rand::thread_rng();
        let colors = [Shape::Red, Shape::Pink, Shape::Blue, Shape::Green];
        let random_color = *colors.choose(&mut rng).unwrap();

        let spawned_shape = commands.spawn(create_shape(random_color)).id();

        commands.entity(board_entity).add_child(spawned_shape);
        board.0.push(spawned_shape);
    }
}

fn swap_buttons_on_press(
    mut interaction_query: Query<
        (&Interaction, Entity),
        (Changed<Interaction>, With<Button>, With<Shape>),
    >,
    mut board_children_q: Query<&mut Children, With<Board>>,
    mut last_pressed_button: Local<Option<Entity>>,
) {
    for (
        interaction,
        just_pressed_button
    ) in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            match *last_pressed_button {
                None => *last_pressed_button = Some(just_pressed_button),
                Some(last_pressed_button_e) => {
                    let mut board_children = board_children_q.get_single_mut().unwrap();
                    let last_pressed_button_i = board_children.iter().position(|e| *e == last_pressed_button_e).unwrap();
                    let just_pressed_button_i = board_children.iter().position(|e| *e == just_pressed_button).unwrap();

                    board_children.swap(last_pressed_button_i, just_pressed_button_i);

                    *last_pressed_button = None;
                }
            }
        }
    }
}

fn create_shape(shape: Shape) -> (Shape, ButtonBundle) {
    let color = match shape {
        Shape::Red => RED_500,
        Shape::Blue => BLUE_500,
        Shape::Green => GREEN_500,
        Shape::Pink => PINK_500,
        Shape::Empty => ZINC_900,
    };

    (
        shape, 
        ButtonBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Auto,
                ..default()
            },
            background_color: color.into(),
            ..default()
        },
    )
}
