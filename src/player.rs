use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::bullet::*;
use crate::movement::MovingObjectBundle;
use crate::sprites::*;

pub struct PlayerPlugin;
const PLAYER_POSITION: (f32, f32, f32) = (0.0, 100.0, 0.0);
const PLAYER_SIZE: f32 = 50.;
pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_player)
            .register_type::<Player>()
            .add_systems(Update, character_movement);
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub rotation_speed: f32,
    pub movement_speed: f32,
}

pub fn character_movement(
    mut player: Query<(&mut Transform, &Player), With<RigidBody>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut transform, player) in &mut player {
        let mut rotation_factor = 0.0;
        let mut movement_factor = Vec2::new(0., 0.);
        let mut side_movement_factor = 0.0;
        let mut movement_direction = Vec3::ZERO;
        let mut movement_distance = 0.0;

        if input.pressed(KeyCode::A) {
            rotation_factor += 1.0;
        }

        if input.pressed(KeyCode::D) {
            rotation_factor -= 1.0;
        }

        if input.pressed(KeyCode::Up) {
            movement_factor.y += 1.0;
            movement_distance = movement_factor.y * player.movement_speed * time.delta_seconds();
            movement_direction = transform.rotation * Vec3::Y;
        } else if input.pressed(KeyCode::Down) {
            movement_factor.y -= 1.0;
            movement_distance = movement_factor.y * player.movement_speed * time.delta_seconds();
            movement_direction = transform.rotation * Vec3::Y;
        }
        if input.pressed(KeyCode::Left) {
            movement_factor.x += -1.0;
            movement_distance = movement_factor.x * player.movement_speed * time.delta_seconds();
            movement_direction = transform.rotation * Vec3::X;
        } else if input.pressed(KeyCode::Right) {
            side_movement_factor += 1.0;
            movement_distance = side_movement_factor * player.movement_speed * time.delta_seconds();
            movement_direction = transform.rotation * Vec3::X;
        }
        if input.pressed(KeyCode::S) {
            let square_sprite = square_sprite(Color::BLUE, BULLET_SIZE);
            let mut transform = transform.clone();
            transform.rotate_z(player.rotation_speed);

            movement_direction = transform.rotation * Vec3::Y;
            println!("Logging: {:?}{:?}", transform.rotation, transform.forward());

            commands.spawn((
                SpriteBundle {
                    sprite: square_sprite.clone(),
                    transform: Transform {
                        translation: transform.translation,
                        rotation: transform.rotation,
                        ..default()
                    },
                    ..default()
                },
                RigidBody::Kinematic,
                Collider::cuboid(BULLET_SIZE, BULLET_SIZE),
                Name::new("bullet"),
                Rotation::default(),
                Bullet {
                    life_time: BULLET_LIFETIME,
                    direction: Vec2 {
                        x: movement_direction.x,
                        y: movement_direction.y,
                    },
                    movement_speed: 100.0,
                },
            ));
        }
        transform.rotate_z(rotation_factor * player.rotation_speed * time.delta_seconds());
        // create the change in translation using the new movement direction and distance
        let translation_delta = movement_direction * movement_distance;
        // the player translation with our new translation delta
        transform.translation += translation_delta;

        let extents = Vec3::from((BOUNDS / 2.0, 0.0));
        // bound the player within the invisible level bounds let extents = Vec3::from((BOUNDS / 2.0, 0.0));
        transform.translation = transform.translation.min(extents).max(-extents);
    }
}

pub fn add_player(mut commands: Commands) {
    let square_sprite = square_sprite(Color::WHITE, PLAYER_SIZE);
    let (x, y, z) = PLAYER_POSITION;

    commands.spawn((
        SpriteBundle {
            sprite: square_sprite,
            transform: Transform::from_xyz(x, y, z).with_scale(Vec3::new(1.0, 1.0, 1.0)),
            ..default()
        },
        RigidBody::Kinematic,
        Collider::cuboid(PLAYER_SIZE, PLAYER_SIZE),
        Player {
            movement_speed: 500.0,                  // meters per second
            rotation_speed: f32::to_radians(360.0), // degrees per second
        },
        Name::new("Player"),
        Rotation::default(),
        Sensor,
    ));
}
