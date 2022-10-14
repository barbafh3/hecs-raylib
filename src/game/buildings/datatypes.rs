use std::collections::HashMap;

use raylib::prelude::*;

use crate::engine::enums::GameResource;

// TAGS ------
pub struct Building;
pub struct Warehouse;
pub struct House;
pub struct Storage;

// STRUCTS ------
pub struct ConstructionPlacement {
    pub position: Vector2,
    pub building_rect: Rectangle
}

pub struct StorageSpace {
    pub item_list: HashMap<GameResource, i32>,
    pub reserved_item_list: HashMap<GameResource, i32>
}

pub struct ConstructionStorage {
    pub tasks_generated: bool,
    pub required_item_list: HashMap<GameResource, i32>,
}
