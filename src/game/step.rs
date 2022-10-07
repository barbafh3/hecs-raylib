use hecs::World;
use raylib::RaylibHandle;

use crate::{
    engine::collision::{detect_body_collisions, detect_trigger_collisions}, 
    game::{
        villagers::step::update_villagers, 
        tasks::update_tasks, 
        buildings::step::update_buildings
    }
};


pub fn update_game(world: &mut World, raylib_handle: &mut RaylibHandle) {
    update_tasks(world);
    update_villagers(world, raylib_handle);
    update_buildings(world);
    detect_body_collisions(world);
    detect_trigger_collisions(world);
}
