pub mod datatypes;
pub mod step;

use hecs::{World, Entity};
use raylib::prelude::*;

use crate::{
    game_core::collision::{CollisionBox, TriggerCollision, BodyCollision}, 
    game_core::constants::{TILE_SIZE, DEFAULT_IDLE_POINT}, 
    game_core::{enums::CollisionType, datatypes::Sprite}
};

use self::datatypes::{Hauler, IdleState};


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

    let hauler: Entity;

    match collision_type {
        CollisionType::Body => {
            hauler = world.spawn((
                Hauler, 
                IdleState::default(idle_point),
                sprite, 
                CollisionBox {
                    rect
                },
                BodyCollision {
                    colliding: false
                }
            ));
        },
        CollisionType::Trigger => {
            hauler = world.spawn((
                Hauler, 
                IdleState::default(idle_point),
                sprite, 
                CollisionBox {
                    rect
                },
                TriggerCollision {
                    colliding: false
                }
            ));
        },
        CollisionType::All => {
            hauler = world.spawn((
                Hauler, 
                IdleState::default(idle_point),
                sprite, 
                CollisionBox {
                    rect
                },
                BodyCollision {
                    colliding: false
                },
                TriggerCollision {
                    colliding: false
                }
            ));
        }
    }

    return hauler;
}
