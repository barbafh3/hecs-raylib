use std::collections::HashMap;

use hecs::{Entity, World};
use raylib::prelude::*;

use crate::{
    engine::{
        datatypes::Sprite,
        enums::{GameResource, VillagerState},
        utils::get_id,
    },
    game::villagers::datatypes::{Hauler, IdleState},
};

use super::{
    buildings::{
        datatypes::{ConstructionStorage, StorageSpace},
        step::storage_has_required_resource,
    },
    constants::HAULER_CAPACITY,
};

// STRUCTS ------
#[derive(Default, Clone)]
pub struct OpenTasks {
    pub haul_list: Vec<HaulTask>,
}

#[derive(Default, Clone)]
pub struct HaulTask {
    pub id: usize,
    pub origin_position: Option<Vector2>,
    pub destination_position: Option<Vector2>,
    pub resource: GameResource,
}

// FUNCTION ------
pub fn update_tasks(world: &mut World) -> Result<(), String> {
    find_idle_hauler_for_task(world)?;
    find_storage_source_for_haul_task(world);

    Ok(())
}

pub fn give_haul_task_to_idle(world: &mut World) {
    let query = world.query_mut::<&mut OpenTasks>();
    query.into_iter().for_each(|(_, open_tasks)| {
        let haul_task = HaulTask {
            id: get_id(),
            origin_position: Some(Vector2 { x: 304.0, y: 48.0 }),
            destination_position: Some(Vector2 { x: 304.0, y: 248.0 }),
            resource: GameResource::Wood,
        };
        open_tasks.haul_list.push(haul_task);
    });
}

pub fn generate_haul_task(
    world: &mut World,
    origin_position: Option<Vector2>,
    destination_position: Vector2,
    resource: GameResource,
) {
    let query = world.query_mut::<&mut OpenTasks>();
    query.into_iter().for_each(|(_, open_tasks)| {
        let haul_task = HaulTask {
            id: get_id(),
            origin_position,
            destination_position: Some(destination_position),
            resource,
        };
        open_tasks.haul_list.push(haul_task);
    });
}

pub fn find_idle_hauler_for_task(world: &mut World) -> Result<(), String> {
    let mut m_selected_hauler: Option<Entity> = None;
    let mut m_haul_task: Option<HaulTask> = None;

    {
        let query = &mut world
            .query::<&Hauler>()
            .with::<IdleState>()
            .without::<HaulTask>();
        query.into_iter().for_each(|(ety, _)| {
            if m_selected_hauler.is_none() {
                m_selected_hauler = Some(ety);
            }
        });
    }

    if let Some(hauler) = m_selected_hauler {
        {
            let query = world.query_mut::<&mut OpenTasks>();
            query.into_iter().for_each(|(_, open_tasks)| {
                if m_haul_task.is_none() {
                    let last = open_tasks.haul_list.last();
                    if last.is_some() && last.unwrap().origin_position.is_some() {
                        m_haul_task = open_tasks.haul_list.pop();
                    }
                };
            });
        }

        if let Some(haul_task) = m_haul_task {
            if haul_task.origin_position.is_some() && haul_task.destination_position.is_some() {
                world.insert(hauler, (haul_task,)).map_err(|_| "No such entity")?;
                IdleState::change_state_to(world, hauler, VillagerState::Loading);
            }
        }
    }

    Ok(())
}

pub fn find_storage_source_for_haul_task(world: &mut World) {
    let mut sourceless_tasks: HashMap<usize, HaulTask> = HashMap::new();

    {
        let mut query = world.query::<&OpenTasks>();
        query.into_iter().for_each(|(_, open_tasks)| {
            open_tasks.haul_list.clone().into_iter().for_each(|task| {
                if task.origin_position.is_none() {
                    sourceless_tasks.insert(task.id.clone(), task.clone());
                }
            });
        });
    }

    {
        sourceless_tasks.iter_mut().for_each(|(_, task)| {
            let mut origin_query = world
                .query::<(&StorageSpace, &Sprite)>()
                .without::<ConstructionStorage>();
            origin_query.into_iter().for_each(|(_, (storage, sprite))| {
                if storage_has_required_resource(&storage, task.resource, HAULER_CAPACITY) {
                    task.origin_position = Some(sprite.position);
                }
            });
        })
    }

    {
        let query = world.query_mut::<&mut OpenTasks>();
        query.into_iter().for_each(|(_, open_tasks)| {
            open_tasks.haul_list.iter_mut().for_each(|mut task| {
                if sourceless_tasks.contains_key(&task.id)
                    && sourceless_tasks[&task.id].origin_position.is_some()
                    && task.origin_position.is_none()
                {
                    task.origin_position = sourceless_tasks[&task.id].origin_position;
                }
            });
        });
    }
}
