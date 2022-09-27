use hecs::{World, Entity};
use raylib::prelude::*;

use crate::{
    game_core::collision::{CollisionBox, TriggerCollision}, 
    game_core::{constants::TILE_SIZE, datatypes::Sprite} 
};

// TYPE ALIAS ------
type Item = (String, i32);

// TAGS ------
pub struct Building;
pub struct Warehouse;
pub struct Storage;

// STRUCTS ------
pub struct StorageSpace {
    pub item_list: Vec<Item>
}

pub struct ConstructionStorage {
    pub required_item_list: Vec<Item>,
    pub item_list: Vec<Item>
}

pub fn spawn_warehouse(world: &mut World, position: Vector2) -> Entity {
    let mut storage: Vec<Item> = vec![];
    storage.push(("Wood".to_string(), 50)); 
    let sprite = Sprite::new(
        position,
        Vector2 { x: 6.0, y: 4.0 }
    );

    let col_box = CollisionBox { 
        rect: Rectangle { 
            x: position.x, 
            y: position.y,
            width: TILE_SIZE,
            height: TILE_SIZE
        }
    };

    let warehouse = world.spawn((
        Warehouse,
        Storage,
        storage,
        sprite,
        col_box,
        TriggerCollision { colliding: false }
    ));

    return warehouse;
}


