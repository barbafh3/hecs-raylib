pub mod step;

use std::collections::HashMap;

use hecs::{World, Entity};
use raylib::prelude::*;

use crate::engine::{
        collision::{CollisionBox, TriggerCollision}, 
        enums::{GameResource, BuildingType}, 
        datatypes::Sprite
    };

use super::constants::TILE_SIZE;

// TAGS ------
pub struct Building;
pub struct Warehouse;
pub struct House;
pub struct Storage;

// STRUCTS ------
pub struct StorageSpace {
    pub item_list: HashMap<GameResource, i32>,
    pub reserved_item_list: HashMap<GameResource, i32>
}

pub struct ConstructionStorage {
    pub tasks_generated: bool,
    pub required_item_list: HashMap<GameResource, i32>,
}

pub fn spawn_warehouse(world: &mut World, position: Vector2, empty: bool) -> Entity {
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
        TriggerCollision::default()
    ));

    return warehouse;
}

pub fn spawn_house(world: &mut World, position: Vector2) -> Entity {
    let sprite = Sprite::new(
        position,
        Vector2 { x: 2.0, y: 1.0 },
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

    let construction = ConstructionStorage { 
        tasks_generated: false,
        required_item_list: get_building_construction_cost(BuildingType::House),
    };

    let warehouse = world.spawn((
        Building,
        House,
        construction,
        sprite,
        col_box,
        TriggerCollision::default()
    ));

    return warehouse;
}

pub fn get_building_construction_cost(building_type: BuildingType) -> HashMap<GameResource, i32> {
    let mut cost = HashMap::new();
    match building_type {
        BuildingType::Warehouse => {
            cost.insert(GameResource::Wood, 70);
            cost.insert(GameResource::Stone, 30);
        },
        BuildingType::House => {
            cost.insert(GameResource::Wood, 50);
        }
    }

    return cost;
}
