use raylib::consts::MouseButton::*;
use std::collections::HashMap;

use hecs::{Entity, World};
use raylib::prelude::{Camera2D, Vector2};
use raylib::RaylibHandle;

use crate::engine::collision::{CollisionBox, TriggerCollision};
use crate::game::constants::TILE_SIZE;
use crate::{
    engine::{
        datatypes::Sprite,
        enums::{GameResource, VillagerState, VillagerType},
    },
    game::{
        constants::HAULER_CAPACITY,
        tasks::{generate_haul_task, HaulTask},
        villagers::{
            datatypes::{CarryingState, GameItem, IdleState, LoadingState},
            hauler::{deliver_resource, receive_resource},
        },
    },
};

use super::datatypes::{
    Building, ConstructionPlacement, ConstructionStorage, OngoingConstruction, StorageSpace,
};

pub fn update_buildings(world: &mut World) -> Result<(), String> {
    generate_construction_haul_tasks(world);
    check_storage_collided_with_entity(world);
    check_construction_collided_with_entity(world);
    check_construction_resources(world)?;

    Ok(())
}

pub fn check_construction_collided_with_entity(world: &mut World) {
    let mut collided_entities: HashMap<Entity, Entity> = HashMap::new();

    {
        let mut query = world
            .query::<&TriggerCollision>()
            .with::<Building>()
            .with::<ConstructionStorage>();
        query.into_iter().for_each(|(ety, trigger_col)| {
            if trigger_col.colliding {
                if let Some(other) = trigger_col.other_trigger {
                    collided_entities.insert(ety, other);
                }
            }
        });
    }

    collided_entities
        .into_iter()
        .for_each(|(building, villager)| {
            let mut m_villager_info: Option<(VillagerType, VillagerState)> = None;

            if let Ok(_) = world.get::<IdleState>(villager) {
                m_villager_info = Some((VillagerType::Hauler, VillagerState::Idle));
            } else if let Ok(_) = world.get::<LoadingState>(villager) {
                m_villager_info = Some((VillagerType::Hauler, VillagerState::Loading));
            } else if let Ok(_) = world.get::<CarryingState>(villager) {
                m_villager_info = Some((VillagerType::Hauler, VillagerState::Carrying));
            }

            if let Some((villager_type, villager_state)) = m_villager_info {
                match villager_type {
                    VillagerType::Hauler => {
                        construction_handle_hauler(world, building, villager, villager_state)
                    }
                    _ => {}
                }
            }
        });
}

pub fn check_storage_collided_with_entity(world: &mut World) {
    let mut collided_entities: HashMap<Entity, Entity> = HashMap::new();

    {
        let mut query = world
            .query::<&TriggerCollision>()
            .with::<Building>()
            .without::<ConstructionStorage>();
        query.into_iter().for_each(|(ety, trigger_col)| {
            if trigger_col.colliding {
                if let Some(other) = trigger_col.other_trigger {
                    collided_entities.insert(ety, other);
                }
            }
        });
    }

    collided_entities
        .into_iter()
        .for_each(|(building, villager)| {
            let mut m_villager_info: Option<(VillagerType, VillagerState)> = None;

            if let Ok(_) = world.get::<IdleState>(villager) {
                m_villager_info = Some((VillagerType::Hauler, VillagerState::Idle));
            } else if let Ok(_) = world.get::<LoadingState>(villager) {
                m_villager_info = Some((VillagerType::Hauler, VillagerState::Loading));
            } else if let Ok(_) = world.get::<CarryingState>(villager) {
                m_villager_info = Some((VillagerType::Hauler, VillagerState::Carrying));
            }

            if let Some((villager_type, villager_state)) = m_villager_info {
                match villager_type {
                    VillagerType::Hauler => {
                        storage_handle_hauler(world, building, villager, villager_state)
                    }
                    _ => {}
                }
            }
        });
}

pub fn construction_handle_hauler(
    world: &mut World,
    building: Entity,
    hauler: Entity,
    state: VillagerState,
) {
    let mut building_position: Vector2 = Vector2::zero();
    let mut is_hauler_destination: bool = false;
    let mut construction_finished: bool = false;

    {
        let result = world.get::<ConstructionStorage>(building);
        if let Ok(storage) = result {
            construction_finished = is_storage_empty(&storage.required_item_list);
        }
    }

    if construction_finished {
        world.remove_one::<ConstructionStorage>(building).unwrap();
        return;
    }

    if let Ok(sprite) = world.get::<Sprite>(building) {
        building_position = sprite.position;
    }

    if let Ok(task) = world.get::<HaulTask>(hauler) {
        if let Some(destination) = task.destination_position {
            if destination == building_position {
                is_hauler_destination = true;
            }
        }
    }

    match state {
        VillagerState::Carrying => {
            if is_hauler_destination {
                let m_item = deliver_resource(world, hauler);
                if let Some(item) = m_item {
                    place_construction_resource(world, building, item);
                }
            }
        }
        _ => {}
    }
}

pub fn storage_handle_hauler(
    world: &mut World,
    building: Entity,
    hauler: Entity,
    state: VillagerState,
) {
    let mut building_position: Vector2 = Vector2::zero();
    let mut is_hauler_origin: bool = false;
    let mut is_hauler_destination: bool = false;

    if let Ok(sprite) = world.get::<Sprite>(building) {
        building_position = sprite.position;
    }

    if let Ok(task) = world.get::<HaulTask>(hauler) {
        if let Some(origin) = task.origin_position {
            if origin == building_position {
                is_hauler_origin = true;
            }
        }
        if let Some(destination) = task.destination_position {
            if destination == building_position {
                is_hauler_destination = true;
            }
        }
    }

    match state {
        VillagerState::Loading => {
            if is_hauler_origin {
                let m_item = receive_resource(world, hauler);
                if let Some(item) = m_item {
                    remove_from_storage(world, building, item);
                }
            }
        }
        VillagerState::Carrying => {
            if is_hauler_destination {
                let m_item = deliver_resource(world, hauler);
                if let Some(item) = m_item {
                    add_to_storage(world, building, item);
                }
            }
        }
        _ => {}
    }
}

pub fn generate_construction_haul_tasks(world: &mut World) {
    let mut task_data_list: Vec<(Vector2, HashMap<GameResource, i32>)> = vec![];

    {
        let query = world
            .query_mut::<(&mut ConstructionStorage, &Sprite)>()
            .without::<ConstructionPlacement>();
        query
            .into_iter()
            .for_each(|(entity, (mut construction, sprite))| {
                if !construction.tasks_generated {
                    println!("Listing tasks to be generated for {:?}", entity);
                    task_data_list.push((sprite.position, construction.required_item_list.clone()));
                    construction.tasks_generated = true;
                }
            });
    }

    task_data_list
        .into_iter()
        .for_each(|(destination, resource_list)| {
            resource_list.into_iter().for_each(|(resource, amount)| {
                let task_count: i32 = amount / HAULER_CAPACITY;

                for _ in 0..task_count {
                    let mut m_origin_pos: Option<Vector2> = None;
                    {
                        let mut origin_query = world
                            .query::<(&StorageSpace, &Sprite)>()
                            .without::<ConstructionStorage>();
                        origin_query.into_iter().for_each(|(_, (storage, sprite))| {
                            if storage_has_required_resource(&storage, resource, HAULER_CAPACITY)
                                && m_origin_pos.is_none()
                            {
                                m_origin_pos = Some(sprite.position);
                            }
                        });
                    }
                    generate_haul_task(world, m_origin_pos, destination, resource);
                }
            });
        });
}

pub fn storage_has_required_resource(
    storage: &StorageSpace,
    resource: GameResource,
    amount: i32,
) -> bool {
    if storage.item_list.contains_key(&resource) {
        if storage.reserved_item_list.contains_key(&resource) {
            return storage.item_list[&resource] - storage.reserved_item_list[&resource] >= amount;
        } else {
            return storage.item_list[&resource] >= amount;
        }
    } else {
        return false;
    }
}

pub fn remove_from_storage(world: &mut World, building: Entity, item: GameItem) -> bool {
    let result = world.get_mut::<StorageSpace>(building);
    if let Ok(mut storage) = result {
        println!("Storage got");
        if storage.item_list.contains_key(&item.resource) {
            println!("Giving resource to hauler");
            *storage.item_list.get_mut(&item.resource).unwrap() -= item.amount;
            return true;
        }
    }

    return false;
}

pub fn add_to_storage(world: &mut World, building: Entity, item: GameItem) -> bool {
    let result = world.get_mut::<&mut StorageSpace>(building);
    if let Ok(mut storage) = result {
        if storage.item_list.contains_key(&item.resource) {
            *storage.item_list.get_mut(&item.resource).unwrap() += item.amount;
        } else {
            storage.item_list.insert(item.resource, item.amount);
        }
        return true;
    }

    return false;
}

pub fn place_construction_resource(world: &mut World, building: Entity, item: GameItem) {
    let result = world.get_mut::<&mut ConstructionStorage>(building);
    if let Ok(mut storage) = result {
        if storage.required_item_list.contains_key(&item.resource) {
            *storage.required_item_list.get_mut(&item.resource).unwrap() -= item.amount;
        }
    }
}

pub fn is_storage_empty(list: &HashMap<GameResource, i32>) -> bool {
    let mut count: i32 = 0;
    list.into_iter().for_each(|(_, amount)| count += amount);
    return count <= 0;
}

pub fn update_construction_hover(
    world: &mut World,
    raylib_handle: &mut RaylibHandle,
    camera: &Camera2D,
) {
    let mut query = world.query::<(&mut ConstructionPlacement, &mut CollisionBox, &mut Sprite)>();
    query
        .into_iter()
        .for_each(|(_, (placement, col_box, sprite))| {
            let mouse_pos =
                raylib_handle.get_screen_to_world2D(raylib_handle.get_mouse_position(), camera);
            let mut current_tile_x = (mouse_pos.x / TILE_SIZE) as i32;
            let mut current_tile_y = (mouse_pos.y / TILE_SIZE) as i32;

            if mouse_pos.x < 0.0 {
                current_tile_x -= 1;
            }
            if mouse_pos.y < 0.0 {
                current_tile_y -= 1;
            }

            placement.position.x = current_tile_x as f32 * TILE_SIZE;
            placement.position.y = current_tile_y as f32 * TILE_SIZE;
            col_box.rect.x = placement.position.x;
            col_box.rect.y = placement.position.y;
            sprite.position = placement.position;
        });
}

pub fn place_hovering_building(world: &mut World, raylib_handle: &mut RaylibHandle) -> Result<(), String> {
    if raylib_handle.is_mouse_button_released(MOUSE_LEFT_BUTTON) {
        let mut m_building: Option<Entity> = None;
        {
            let query =
                world.query_mut::<(&ConstructionPlacement, &TriggerCollision, &mut Sprite)>();
            query
                .into_iter()
                .for_each(|(entity, (placement, trigger, mut sprite))| {
                    if !trigger.colliding {
                        sprite.position = placement.position;
                        m_building = Some(entity);
                    }
                });
        }

        {
            if let Some(building) = m_building {
                world.remove_one::<ConstructionPlacement>(building).map_err(|_| "Component error")?;
            }
        }
    }
    
    Ok(())
}

pub fn check_construction_resources(world: &mut World) -> Result<(), String> {
    let mut building_list: Vec<Entity> = vec![];

    {
        let mut query = world.query::<&ConstructionStorage>();
        query.into_iter().for_each(|(entity, storage)| {
            if is_storage_empty(&storage.required_item_list) {
                building_list.push(entity);
            }
        });
    }

    for building in building_list.into_iter() {
        world.remove_one::<ConstructionStorage>(building).map_err(|_| "Component error")?;
        world
            .insert_one(
                building,
                OngoingConstruction {
                    work_required: 100.0,
                },
            )
            .map_err(|_| "No such entity")?;
    };

    Ok(())
}
