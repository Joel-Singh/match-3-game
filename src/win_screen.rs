use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct WinScreen;

pub fn win_screen(app: &mut App) {
    app.add_systems(OnEnter(GameState::WinScreen), setup)
        .add_systems(OnExit(GameState::WinScreen), cleanup);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        WinScreen,
        Text::new("You win!"),
        Node {
            margin: UiRect::all(Val::Auto),
            ..default()
        },
    ));
}

fn cleanup(mut commands: Commands, map: Query<Entity, With<WinScreen>>) {
    commands.entity(map.single()).despawn_recursive();
}
