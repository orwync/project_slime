use crate::player::Player;
use crate::sprites::*;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub struct BulletPlugin;

const BULLET_LIFETIME: f32 = 10.0;
const BULLET_SPEED: f32 = 1000.;
const BULLET_SIZE: f32 = 10.0;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_bullet, update_bullets))
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

pub fn add_bullet(
    mut commands: Commands,
    player: Query<&Transform, (With<RigidBody>, With<Player>)>,
    input: Res<Input<KeyCode>>,
    _time: Res<Time>,
    // time: Res<Time>,
) {
    if !input.pressed(KeyCode::S) {
        return;
    }

    let square_sprite = square_sprite(Color::BLUE);
    let transform = player.single().clone();
    let movement_direction = transform.forward() * Vec3::Z;
    let direction = Vec2 {
        x: movement_direction.x,
        y: movement_direction.y,
    };
    let diff = direction - Vec2::new(transform.rotation.z, transform.rotation.w);

    println!("Logging: {:?}{:?}", transform.rotation, transform.forward());

    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform,
            ..default()
        },
        RigidBody::Kinematic,
        Collider::cuboid(BULLET_SIZE, BULLET_SIZE),
        Name::new("Player"),
        Rotation::default(),
        Bullet {
            life_time: BULLET_LIFETIME,
            movement_speed: BULLET_SPEED,
            direction: diff.normalize(),
        },
    ));
}

pub fn update_bullets(
    mut bullet_query: Query<(&mut Bullet, &mut Transform, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut bullet, mut transform, entity) in bullet_query.iter_mut() {
        bullet.life_time -= time.delta_seconds();
        let moving = bullet.movement_speed * bullet.direction * time.delta_seconds();
        transform.translation += Vec3::new(moving.x, moving.y, 0.);
        if bullet.life_time <= 0. {
            commands.entity(entity).despawn();
        }
    }
}
