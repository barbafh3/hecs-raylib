use hecs::World;
use raylib::RaylibHandle;

use crate::{entities::update_idle_state, collision::detect_body_collisions};


pub fn update_game(world: &mut World, raylib_handle: &mut RaylibHandle) {
    // move_haulers(world, raylib_handle);
    update_idle_state(world, raylib_handle);
    detect_body_collisions(world);
}
