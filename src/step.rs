use hecs::World;
use raylib::RaylibHandle;

use crate::{entities::{update_idle_state, update_collision_box_position}, collision::{detect_body_collisions, detect_trigger_collisions}, tasks::check_finished_haul_tasks};


pub fn update_game(world: &mut World, raylib_handle: &mut RaylibHandle) {
    update_idle_state(world, raylib_handle);
    update_collision_box_position(world);
    detect_body_collisions(world);
    detect_trigger_collisions(world);
    check_finished_haul_tasks(world);
}
