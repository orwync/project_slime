use bevy::prelude::*;

pub fn square_sprite(color: Color) -> Sprite {
    Sprite {
        color,
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    }
}
