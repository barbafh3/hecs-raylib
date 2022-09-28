use hecs::{World, Entity};

use crate::{
    game_core::enums::{GameResource, VillagerState}, villagers::datatypes::{Hauler, IdleState, CarryingState, LoadingState, Backpack}, 
};

// STRUCTS ------
pub struct HaulTask {
    pub origin: i32,
    pub destination: i32,
    pub resource: GameResource,
    pub resource_amount: i32,
    pub delivered_amount: i32
}

pub fn update_tasks(world: &mut World) {
    check_hauler_loaded(world);
    check_finished_haulers(world);
}

// FUNCTION ------
pub fn give_haul_task_to_idle(world: &mut World) {
    let mut selected_hauler: Option<Entity> = None;

    {
        let query = &mut world.query::<&Hauler>().with::<IdleState>().without::<HaulTask>();
        query.into_iter().for_each(|(ety, _)| {
            println!("Hauler: {:?}", ety);
            if selected_hauler.is_none() {
                selected_hauler = Some(ety);
            }
        });
    }


    if let Some(hauler) = selected_hauler {
        let haul_task = HaulTask {
            origin: 0,
            destination: 0,
            resource: GameResource::Wood,
            resource_amount: 100,
            delivered_amount: 0
        };
        world.insert(hauler, (haul_task,)).unwrap();
        IdleState::change_state_to(world, hauler, VillagerState::Loading);
    }
}

pub fn check_hauler_loaded(world: &mut World) {
    let mut loaded_haulers: Vec<Entity> = vec![];

    {
        let mut query = world.query::<(&HaulTask, &Backpack)>().with::<Hauler>().with::<LoadingState>();
        query.into_iter().for_each(|(entity, (task, backpack))| {
            if let Some(item) = &backpack.item {
                if task.resource == item.resource && 
                    task.resource_amount == item.amount && !loaded_haulers.contains(&entity) {
                    loaded_haulers.push(entity);
                }
            }
        });
    }

    loaded_haulers.into_iter().for_each(|entity| {
        LoadingState::change_state_to(world, entity, VillagerState::Carrying);
    });
}

pub fn check_finished_haulers(world: &mut World) {
    let mut finished_haulers: Vec<Entity> = vec![];

    {
        let mut query = world.query::<&HaulTask>().with::<Hauler>().with::<CarryingState>();
        query.into_iter().for_each(|(ety, task)| {
            if task.delivered_amount == task.resource_amount {
                println!("Finishing delivery...");
                if !finished_haulers.contains(&ety) {
                    finished_haulers.push(ety);
                }
            }
        });
    }

    finished_haulers.into_iter().for_each(|entity| {
        world.remove_one::<HaulTask>(entity).unwrap();
        world.exchange_one::<Backpack, Backpack>(entity, Backpack::default()).unwrap();
        CarryingState::change_state_to(world, entity, VillagerState::Idle);
    });
}
