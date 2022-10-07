pub mod datatypes;
pub mod step;
pub mod hauler;

use hecs::{World, Entity};
use raylib::prelude::*;

use crate::{
    engine::collision::{CollisionBox, TriggerCollision, BodyCollision}, 
    engine::{enums::CollisionType, datatypes::Sprite}
};

use self::{datatypes::{IdleState, Backpack, IdleInfo}, hauler::Hauler};

use super::constants::{TILE_SIZE, DEFAULT_IDLE_POINT};


pub fn spawn_hauler(
    world: &mut World, 
    position: Vector2, 
    atlas_tile: Vector2, 
    collision_type: CollisionType,
    opt_idle_point: Option<Vector2>
) -> Entity {
    let sprite = Sprite::new(
        position,
        atlas_tile,
        TILE_SIZE
    );
    let rect = Rectangle {
        x: position.x,
        y: position.y,
        width: TILE_SIZE,
        height: TILE_SIZE
    };
    let idle_point: Vector2;

    match opt_idle_point {
        Some(point) => idle_point = point,
        None => idle_point = DEFAULT_IDLE_POINT
    }

    let hauler: Entity = world.spawn((
        Hauler, 
        IdleInfo::default(idle_point),
        IdleState,
        Backpack::default(),
        sprite, 
        CollisionBox {
            rect
    }));

    match collision_type {
        CollisionType::Body => {
            world.insert_one(hauler, BodyCollision::default()).unwrap();
        },
        CollisionType::Trigger => {
            world.insert_one(hauler, TriggerCollision::default()).unwrap();
        },
        CollisionType::All => {
            world.insert_one(
                hauler, 
                (
                    BodyCollision::default(), 
                    TriggerCollision::default()
                )
            ).unwrap();
        }
    }

    return hauler;
}
