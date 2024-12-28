use bevy::prelude::*;

use crate::board::MatchMade;

pub fn dev_hotkeys(app: &mut App) {
    app.add_systems(FixedUpdate, increment_match_on_z);
}

fn increment_match_on_z(mut match_made: EventWriter<MatchMade>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.pressed(KeyCode::KeyZ) {
        match_made.send(MatchMade::default());
    }
}
