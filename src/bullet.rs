use crate::level_1::*;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub struct BulletPlugin;

pub const BULLET_LIFETIME: f32 = 5.0;
pub const BULLET_SPEED: f32 = 1000.;
pub const BULLET_SIZE: f32 = 10.0;

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

pub fn update_bullets(
    mut bullet_query: Query<(&mut Bullet, &mut Transform, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut bullet, mut transform, entity) in bullet_query.iter_mut() {
        bullet.life_time -= time.delta_seconds();
        let moving = BULLET_SPEED * bullet.direction * time.delta_seconds();

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
    }
}
