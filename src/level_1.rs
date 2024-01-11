use bevy::prelude::*;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::prelude::*;


pub struct Level1Plugin;


impl Plugin for Level1Plugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_base);
    }
}

pub struct Size(i32, i32);
pub struct Pos(f32, f32);

#[derive(Component)]
pub struct Base{
    pub dimen :Size,
    pub pos: Pos, 
}

pub fn add_base(mut commands :Commands) {
    let square_sprite = Sprite {
        color: Color::rgba(255.0, 0.0, 0.0, 255.0),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };

    commands.spawn((
            SpriteBundle {
                sprite: square_sprite.clone(),
                transform: Transform::from_xyz(0.0, -350.0, 0.0)
                    .with_scale(Vec3::new(26.0, 1.0, 1.0)),
                    ..default()
            },
            RigidBody::Static,
            Collider::cuboid(50.0, 50.0),
            Name::new("base2")
            ));
    commands.spawn((
            SpriteBundle {
                sprite: square_sprite.clone(),
                transform: Transform::from_xyz(400.0, -150.0, 0.0)
                    .with_scale(Vec3::new(10.0, 1.0, 1.0)),
                    ..default()
            },
            RigidBody::Static,
            Collider::cuboid(50.0, 50.0),
            Name::new("base")
            ));
commands.spawn((
            SpriteBundle {
                sprite: square_sprite.clone(),
                transform: Transform::from_xyz(650.0, -150.0, 0.0)
                    .with_scale(Vec3::new(1.0, 20.0, 1.0)),
                    ..default()
            },
            RigidBody::Static,
            Collider::cuboid(50.0, 50.0),
            Name::new("wall_1")
            ));
commands.spawn((
            SpriteBundle {
                sprite: square_sprite.clone(),
                transform: Transform::from_xyz(-870.0, -150.0, 0.0)
                    .with_scale(Vec3::new(10.0, 20.0, 1.0)),
                    ..default()
            },
            RigidBody::Static,
            Collider::cuboid(50.0, 50.0),
            Name::new("wall_2")
            ));

}

