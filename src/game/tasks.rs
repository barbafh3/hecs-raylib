use hecs::{World, Entity};
use raylib::prelude::*;

use crate::{
    engine::enums::{GameResource, VillagerState}, 
    game::villagers::{
        datatypes::IdleState,
        hauler::Hauler
    }, 
};

// STRUCTS ------
#[derive(Default)]
pub struct OpenTasks {
    pub haul_list: Vec<HaulTask>
}

#[derive(Default)]
pub struct HaulTask {
    pub origin: Option<Entity>,
    pub origin_position: Option<Vector2>,
    pub destination: Option<Entity>,
    pub destination_position: Option<Vector2>,
    pub resource: GameResource,
}

// FUNCTION ------
pub fn update_tasks(world: &mut World) {
    find_idle_hauler_for_task(world);
}

pub fn give_haul_task_to_idle(world: &mut World) {
    let query = world.query_mut::<&mut OpenTasks>();
    query.into_iter().for_each(|(_, open_tasks)| {
        let haul_task = HaulTask {
            origin: None,
            origin_position: Some(Vector2 { x: 304.0, y: 48.0 }),
            destination: None,
            destination_position: Some(Vector2{ x: 304.0, y: 248.0 }),
            resource: GameResource::Wood,
        };
        open_tasks.haul_list.push(haul_task);
    });
}

pub fn generate_haul_task(
    world: &mut World, 
    origin_position: Vector2, 
    destination_position: Vector2, 
    resource: GameResource
) {
    let query = world.query_mut::<&mut OpenTasks>();
    query.into_iter().for_each(|(_, open_tasks)| {
        let haul_task = HaulTask {
            origin: None,
            origin_position: Some(origin_position),
            destination: None,
            destination_position: Some(destination_position),
            resource
        };
        open_tasks.haul_list.push(haul_task);
    });
}

pub fn find_idle_hauler_for_task(world: &mut World) {
    let mut m_selected_hauler: Option<Entity> = None;
    let mut m_haul_task: Option<HaulTask> = None;

    {
        let query = &mut world.query::<&Hauler>().with::<IdleState>().without::<HaulTask>();
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
                m_haul_task = open_tasks.haul_list.pop();
            });
        }

        if let Some(haul_task) = m_haul_task {
            if haul_task.origin_position.is_some() && haul_task.destination_position.is_some() {
                world.insert(hauler, (haul_task,)).unwrap();
                IdleState::change_state_to(world, hauler, VillagerState::Loading);
            }
        }
    }
}
