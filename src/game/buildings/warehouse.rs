use std::collections::HashMap;

use hecs::{World, Entity};
use raylib::prelude::*;

use crate::{
    engine::{
        enums::GameResource, 
        datatypes::Sprite, 
        collision::{CollisionBox, TriggerCollision}
    }, 
    game::constants::TILE_SIZE
};

use super::datatypes::{StorageSpace, Building, Warehouse, Storage};


pub fn spawn_finished_warehouse(world: &mut World, position: Vector2, empty: bool) -> Entity {
    let mut storage: HashMap<GameResource, i32> = HashMap::new();
    if !empty {
        storage.insert(GameResource::Wood, 150); 
    }
    let storage_space: StorageSpace = StorageSpace { 
        item_list: storage, 
        reserved_item_list: HashMap::new() 
    };
    let sprite = Sprite::new(
        position,
        Vector2 { x: 6.0, y: 4.0 },
        TILE_SIZE
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
        Building,
        Warehouse,
        Storage,
        storage_space,
        sprite,
        col_box,
        TriggerCollision::new()
    ));

    return warehouse;
}
