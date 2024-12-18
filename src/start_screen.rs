use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
struct StartScreen;

pub fn start_screen(app: &mut App) {
    app.add_systems(OnEnter(GameState::StartScreen), setup)
        .add_systems(OnExit(GameState::StartScreen), cleanup)
        .add_systems(
            FixedUpdate,
            start_if_clicked.run_if(in_state(GameState::StartScreen)),
        );
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                height: Val::Vh(100.0),
                width: Val::Vw(100.0),
                align_items: AlignItems::Center,
                padding: UiRect::top(Val::Px(100.0)),
                row_gap: Val::Px(30.0),
                ..default()
            },
            StartScreen,
            Name::new("StartScreen Root"),
            Button,
        ))
        .with_children(|root| {
            root.spawn(Text::new("Joel's Match 3 Game With Rust And Bevy"));
            root.spawn(Text::new("Click to start"));
        });
}

fn start_if_clicked(
    mut game_state: ResMut<NextState<GameState>>,
    interaction: Query<&Interaction, Changed<Interaction>>,
) {
    if interaction.iter().any(|i| *i == Interaction::Pressed) {
        game_state.set(GameState::Map);
    }
}

fn cleanup(mut commands: Commands, map: Query<Entity, With<StartScreen>>) {
    commands.entity(map.single()).despawn_recursive();
}
