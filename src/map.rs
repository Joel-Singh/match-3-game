use bevy::{color::palettes::tailwind::GRAY_50, prelude::*};

use crate::GameState;

#[derive(Component)]
pub struct Map;

pub fn map(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Map), setup)
        .add_systems(OnExit(GameState::Map), cleanup);
}

fn setup(mut commands: Commands) {
    commands.spawn((Map, NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        background_color: GRAY_50.into(),
        ..default()
    })).with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(50.),
                height: Val::Px(50.),
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            },
            ..default()
        });
    });
}

fn cleanup(mut commands: Commands, map: Query<Entity, With<Map>>) {
    commands.entity(map.single()).despawn_recursive();
}
