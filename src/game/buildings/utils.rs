use std::collections::HashMap;

use crate::engine::enums::{BuildingType, GameResource};

pub fn get_building_construction_cost(building_type: BuildingType) -> HashMap<GameResource, i32> {
    let mut cost = HashMap::new();
    match building_type {
        BuildingType::Warehouse => {
            cost.insert(GameResource::Wood, 70);
            cost.insert(GameResource::Stone, 30);
        }
        BuildingType::House => {
            cost.insert(GameResource::Wood, 50);
        }
    }

    return cost;
}

