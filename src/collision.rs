use bevy::{prelude::*, utils::HashMap};
use bevy_xpbd_2d::prelude::*;

const PLAYER_MOVMENT_SPEED:i32= 100;
const PLAYER_POSITION:(i32,i32,i32)=(0,100,0);

#[derive(Component)]
pub struct Collider{
    pub size: (f32,f32),
    pub colliding_entities: Vec<Entity>,
}

impl Collider{
    pub fn new(size: (f32,f32)) -> Self {
        Self {
            size,
            colliding_entities: vec![],
        }
    }
}


pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // First phase: Detect collisions.
    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a
                    .translation()
                    .distance(transform_b.translation());
                println!("{} {}", distance - collider_a.size.0/2.0 + collider_b.size.0/2.0, distance);
                if distance - collider_a.size.0/2.0 + collider_b.size.0/2.0 <= 0.0 || distance - collider_a.size.1/2.0 + collider_b.size.1/2.0 <= 0.0  {
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }

    // Second phase: Update colliders.
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}

