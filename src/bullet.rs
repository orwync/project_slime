use crate::level_1::*;
use bevy::ecs::{entity, query};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub struct BulletPlugin;

pub const BULLET_LIFETIME: f32 = 5.0;
pub const BULLET_SPEED: f32 = 1000.;
pub const BULLET_SIZE: f32 = 10.0;
pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_bullets, bullet_despawn))
            .register_type::<Bullet>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Bullet {
    pub life_time: f32,
    pub movement_speed: f32,
    pub direction: Vec2,
}

// pub fn add_bullet(
//     mut commands: Commands,
//     player: Query<&Transform, (With<RigidBody>, With<Player>)>,
//     input: Res<Input<KeyCode>>,
//     _time: Res<Time>,
//     // time: Res<Time>,
// ) {
//     if !input.pressed(KeyCode::S) {
//         return;
//     }
//
//     let square_sprite = square_sprite(Color::BLUE, BULLET_SIZE);
//     let transform = player.single().clone();
//     let movement_direction = transform.forward() * Vec3::Z;
//     let direction = Vec2 {
//         x: movement_direction.x,
//         y: movement_direction.y,
//     };
//     let diff = direction - Vec2::new(transform.rotation.z, transform.rotation.w);
//
//     println!("Logging: {:?}{:?}", transform.rotation, transform.forward());
//
//     commands.spawn((
//         SpriteBundle {
//             sprite: square_sprite.clone(),
//             transform,
//             ..default()
//         },
//         RigidBody::Kinematic,
//         Collider::cuboid(BULLET_SIZE, BULLET_SIZE),
//         Name::new("Player"),
//         Rotation::default(),
//         Bullet {
//             life_time: BULLET_LIFETIME,
//             movement_speed: BULLET_SPEED,
//             direction: diff.normalize(),
//         },
//     ));
// }

pub fn update_bullets(
    mut bullet_query: Query<(&mut Bullet, &mut Transform, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut bullet, mut transform, entity) in bullet_query.iter_mut() {
        bullet.life_time -= time.delta_seconds();
        let moving = BULLET_SPEED * bullet.direction * time.delta_seconds();
        // let extents = Vec3::from((BOUNDS / 2.0, 0.0));
        // bound the player within the invisible level bounds let extents = Vec3::from((BOUNDS / 2.0, 0.0));
        // transform.translation = transform.translation.min(extents).max(-extents);

        transform.translation += Vec3::new(moving.x, moving.y, 0.);
        if bullet.life_time <= 0. {
            commands.entity(entity).despawn();
        }
    }
}
fn bullet_despawn(
    mut collision_event_reader: EventReader<Collision>,
    bullets: Query<(&mut Bullet, &mut Transform, Entity)>,
    walls: Query<(&mut Base, &mut Transform, Entity), Without<Bullet>>,
    mut commands: Commands,
) {
    for Collision(contact) in collision_event_reader.read() {
        if bullets.contains(contact.entity1) && walls.contains(contact.entity2) {
            commands.entity(contact.entity1).despawn();
        } else if bullets.contains(contact.entity2) && walls.contains(contact.entity1) {
            commands.entity(contact.entity2).despawn();
        }
        println!("{:?}{:?} ", contact.entity1, contact.entity2);
        for (_, _, entity) in walls.iter() {}
        println!(
            "{:?} {:?}{:?}{:?} ",
            bullets.contains(contact.entity1),
            walls.contains(contact.entity2),
            bullets.contains(contact.entity2),
            walls.contains(contact.entity1)
        );
    }
}
