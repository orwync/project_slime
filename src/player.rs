use std::cmp::Ordering;

use bevy::{prelude::*, math::Vec2};
use bevy_xpbd_2d::prelude::*;

pub struct PlayerPlugin;

const PLAYER_POSITION:(f32,f32,f32)=(0.0,100.0,0.0);
const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_player).register_type::<Player>()
            .add_systems(Update,  character_movement);
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub rotation_speed:f32,
    pub movement_speed:f32,
}


pub fn character_movement(
    mut player: Query<(&mut Transform, &Player), With<RigidBody>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut player{
        let mut rotation_factor = 0.0;
        let mut movement_factor = 0.0;

        if input.pressed(KeyCode::Left) {
            rotation_factor += 2.0;
        }

        if input.pressed(KeyCode::Right) {
            rotation_factor -= 2.0;
        }

        if input.pressed(KeyCode::Up) {
            movement_factor += 1.0;
        }

        // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
        transform.rotate_z(rotation_factor * player.rotation_speed * time.delta_seconds());

        // get the player's forward vector by applying the current rotation to the ships initial facing
        // vector
        let movement_direction = transform.rotation * Vec3::Y;
        // get the distance the player will move based on direction, the ship's movement speed and delta
        // time
        let movement_distance = movement_factor * player.movement_speed * time.delta_seconds();
        // create the change in translation using the new movement direction and distance
        let translation_delta = movement_direction * movement_distance;
        // update the player translation with our new translation delta
        transform.translation += translation_delta;

         let extents = Vec3::from((BOUNDS / 2.0, 0.0));
        // bound the player within the invisible level bounds let extents = Vec3::from((BOUNDS / 2.0, 0.0));
        transform.translation = transform.translation.min(extents).max(-extents);    }
}

pub fn kinematic_gravity( 
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut players: Query<(&mut Transform, &Player, Entity), With<RigidBody>>,
    time: Res<Time>,
) {

    for (mut transform, player, entity) in &mut players{
        let movement_amount = player.movement_speed* time.delta_seconds();
        for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
            if entity1.cmp(&entity) == Ordering::Equal || entity2.cmp(&entity) == Ordering::Equal{
                println!(
                    "Entities {:?} and {:?} started colliding",
                    entity1,
                    entity2,
                    );
                return;
            }
        }
        transform.translation.y -= movement_amount;

    }
}

// fn kinematic_collision(mut collision_event_reader: EventReader<CollisionStarted>,
//     mut player: Query<(&mut LinearVelocity, &Player), With<RigidBody>>,
// ) {
//     for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
//         for (mut linear_velocity, _player) in &mut player{
//             linear_velocity.y = -1.0;
//             println!(
//                 "Entities {:?} and {:?} started colliding",
//                 entity1,
//                 entity2,
//                 );
//         }
//     }
// }

pub fn add_player(mut commands :Commands) {
    let square_sprite = Sprite {
        color: Color::rgba(67.0, 67.0, 217.0, 255.0),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };

    let (x,y,z)= PLAYER_POSITION;

    commands.spawn((
            SpriteBundle {
                sprite: square_sprite.clone(),
                transform: Transform::from_xyz(x, y, z)
                    .with_scale(Vec3::new(1.0, 1.0, 1.0)),
                    ..default()
            },
            RigidBody::Kinematic,
            Collider::cuboid(50.0, 50.0),
            Player {  
                movement_speed: 500.0,                  // meters per second
                rotation_speed: f32::to_radians(360.0), // degrees per second
            },
            Name::new("Player"),
            Rotation::default(),
        ));
}

