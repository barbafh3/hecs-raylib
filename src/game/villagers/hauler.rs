use hecs::{World, Entity};
use raylib::RaylibHandle;

use crate::{
    game::{tasks::HaulTask, constants::HAULER_CAPACITY}, 
    engine::{
        enums::VillagerState, datatypes::Sprite,
    }
};

use super::{datatypes::{GameItem, Backpack, LoadingState, CarryingState}, step::move_villager};

pub struct Hauler;


pub fn update_loading_state(world: &mut World, raylib_handle: &mut RaylibHandle) {
    let delta = raylib_handle.get_frame_time();
    let origin_missing_haulers: Vec<Entity> = vec![];

    let query = world.query_mut::<(&HaulTask, &mut Sprite)>().with::<LoadingState>();
    query.into_iter().for_each(|(_, (task, sprite))| {
        if let Some(target_position) = task.origin_position {
            if (target_position - sprite.position).length() > 1.0 {
                move_villager(target_position, sprite, delta);
            }
        }
    });

    origin_missing_haulers.into_iter().for_each(|hauler| {
        LoadingState::change_state_to(world, hauler, VillagerState::Idle);
    });
}

pub fn update_carrying_state(world: &mut World, raylib_handle: &mut RaylibHandle) {
    let delta = raylib_handle.get_frame_time();

    {
        let query = world.query_mut::<(&mut HaulTask, &mut Sprite)>().with::<CarryingState>();
        query.into_iter().for_each(|(_, (task, sprite))| {
            if let Some(target_position) = task.destination_position {
                if (target_position - sprite.position).length() > 1.0 {
                    move_villager(target_position, sprite, delta);
                }
            }
        });
    }
}

pub fn receive_resource(world: &mut World, hauler: Entity) -> Option<GameItem> {
    let mut m_item: Option<GameItem> = None;

    {
        let result = world.get::<HaulTask>(hauler);
        if let Ok(task) = result {
            m_item = Some(GameItem::new(task.resource, HAULER_CAPACITY));
        }
    }
    {
        let backpack = Backpack{ item: m_item.clone() };
        world.exchange_one::<Backpack, Backpack>(hauler, backpack).unwrap();
        LoadingState::change_state_to(world, hauler, VillagerState::Carrying);
    }

    return m_item;
}

pub fn deliver_resource(world: &mut World, hauler: Entity) -> Option<GameItem> {
    let mut m_item: Option<GameItem> = None;

    {
        let result = world.get::<HaulTask>(hauler);
        if let Ok(task) = result {
            m_item = Some(GameItem::new(task.resource, HAULER_CAPACITY));
        }
    }
    {
        let backpack = Backpack{ item: None };
        world.exchange_one::<Backpack, Backpack>(hauler, backpack).unwrap();
        world.remove_one::<HaulTask>(hauler).unwrap();
        CarryingState::change_state_to(world, hauler, VillagerState::Idle);
    }

    return m_item;
}
