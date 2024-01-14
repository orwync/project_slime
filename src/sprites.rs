use bevy::prelude::*;

pub fn square_sprite(color: Color, size: f32) -> Sprite {
    Sprite {
        color,
        custom_size: Some(Vec2::splat(size)),
        ..default()
    }
}
