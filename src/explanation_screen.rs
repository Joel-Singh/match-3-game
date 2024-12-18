use bevy::{color::palettes::tailwind::RED_200, prelude::*};

use crate::GameState;

#[derive(Component)]
struct ExplanationScreen;

pub fn explanation_screen(app: &mut App) {
    app.add_systems(OnEnter(GameState::ExplanationScreen), setup)
        .add_systems(OnExit(GameState::ExplanationScreen), cleanup);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        ExplanationScreen,
        Text::new("Explanation Screen"),
        TextLayout {
            justify: JustifyText::Center,
            ..default()
        },
        Node {
            height: Val::Px(300.),
            width: Val::Px(300.),
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        Name::new("ExplanationScreen Root"),
        BackgroundColor(RED_200.into()),
    ));
}

fn cleanup(mut commands: Commands, map: Query<Entity, With<ExplanationScreen>>) {
    commands.entity(map.single()).despawn_recursive();
}
