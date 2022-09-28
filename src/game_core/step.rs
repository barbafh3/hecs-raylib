use hecs::World;
use raylib::RaylibHandle;

use crate::{
    game_core::collision::{detect_body_collisions, detect_trigger_collisions}, 
    villagers::step::update_villagers, tasks::update_tasks
};


pub fn update_game(world: &mut World, raylib_handle: &mut RaylibHandle) {
    update_tasks(world);
    update_villagers(world, raylib_handle);
    detect_body_collisions(world);
    detect_trigger_collisions(world);
}
