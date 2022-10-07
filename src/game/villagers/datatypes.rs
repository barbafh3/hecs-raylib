use hecs::{Entity, World};
use raylib::prelude::*;

use crate::engine::enums::*;

// TAGS ------
pub struct WorkingState;

// STRUCTS ------
pub struct IdleInfo {
    pub idle_point: Vector2,
    pub idle_timer: f32,
    pub timer_range: (f32, f32),
    pub radius: f32,
    pub target_position: Vector2
}

impl IdleInfo {
    pub fn default(idle_point: Vector2) -> IdleInfo {
        IdleInfo {
            idle_point,
            idle_timer: 0.0,
            timer_range: (3.0, 5.0),
            radius: 20.0,
            target_position: Vector2::zero()
        }
    }
}

pub struct IdleState;

impl IdleState {
    pub fn change_state_to(world: &mut World, entity: Entity, new_state: VillagerState) {
        println!("Changed state to {:?}", new_state);
        match new_state {
            VillagerState::Loading => world.insert_one(entity, LoadingState).unwrap(),
            VillagerState::Working => world.insert_one(entity, WorkingState).unwrap(),
            _ => {}
        }
        world.remove_one::<IdleState>(entity).unwrap();
    }
}

pub struct LoadingState;

impl LoadingState {
    pub fn change_state_to(world: &mut World, entity: Entity, new_state: VillagerState) {
        println!("Changed state to {:?}", new_state);
        match new_state {
            VillagerState::Idle => world.insert_one(entity, IdleState).unwrap(),
            VillagerState::Carrying => world.insert_one(entity, CarryingState).unwrap(),
            _ => {}
        }
        world.remove_one::<LoadingState>(entity).unwrap();
    }
}

pub struct CarryingState;

impl CarryingState {
    pub fn change_state_to(world: &mut World, entity: Entity, new_state: VillagerState) {
        println!("Changed state to {:?}", new_state);
        match new_state {
            VillagerState::Idle => world.insert_one(entity, IdleState).unwrap(),
            _ => {}
        }
        world.remove_one::<CarryingState>(entity).unwrap();
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GameItem {
    pub resource: GameResource,
    pub amount: i32
}

impl GameItem {
    pub fn new(resource: GameResource, amount: i32) -> GameItem {
        GameItem { resource , amount }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Backpack {
    pub item: Option<GameItem>
}

impl Backpack {
    pub fn clone_item(&self) -> Option<GameItem> {
        return self.item.clone();
    }
}

