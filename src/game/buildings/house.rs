use hecs::{Entity, World};
use raylib::prelude::*;

use crate::{
    engine::{
        collision::{CollisionBox, TriggerCollision},
        datatypes::Sprite,
        enums::BuildingType,
    },
    game::constants::TILE_SIZE,
};

use super::{
    datatypes::{Building, ConstructionPlacement, ConstructionStorage, House},
    utils::get_building_construction_cost,
};

pub fn spawn_finished_house(world: &mut World, position: Vector2) -> Entity {
    let sprite = Sprite::new(position, Vector2 { x: 2.0, y: 1.0 }, TILE_SIZE);

    let col_box = CollisionBox {
        rect: Rectangle {
            x: position.x,
            y: position.y,
            width: TILE_SIZE,
            height: TILE_SIZE,
        },
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
        TriggerCollision::new(),
    ));

    return warehouse;
}

pub fn spawn_house_placement(world: &mut World, position: Vector2) -> Entity {
    let sprite = Sprite::new(position, Vector2 { x: 2.0, y: 1.0 }, TILE_SIZE);

    let placement = ConstructionPlacement {
        position,
        building_rect: Rectangle {
            x: 2.0 * TILE_SIZE,
            y: 1.0 * TILE_SIZE,
            width: TILE_SIZE,
            height: TILE_SIZE,
        },
    };
    let col_box = CollisionBox {
        rect: Rectangle {
            x: position.x,
            y: position.y,
            width: TILE_SIZE,
            height: TILE_SIZE,
        },
    };

    let construction = ConstructionStorage {
        tasks_generated: false,
        required_item_list: get_building_construction_cost(BuildingType::House),
    };

    return world.spawn((
        Building,
        House,
        placement,
        sprite,
        construction,
        col_box,
        TriggerCollision::new(),
    ));
}
