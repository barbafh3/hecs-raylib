use hecs::World;
use raylib::RaylibHandle;

use crate::{
    game_core::collision::{detect_body_collisions, detect_trigger_collisions}, 
    tasks::check_finished_haul_tasks, villagers::step::update_villagers
};


pub fn update_game(world: &mut World, raylib_handle: &mut RaylibHandle) {
    update_villagers(world, raylib_handle);
    detect_body_collisions(world);
    detect_trigger_collisions(world);
    check_finished_haul_tasks(world);
}
