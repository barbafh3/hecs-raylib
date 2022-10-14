use hecs::World;
use raylib::{RaylibHandle, text::Font};

use crate::{
    engine::collision::{detect_body_collisions, detect_trigger_collisions}, 
    game::{
        villagers::step::update_villagers, 
        tasks::update_tasks, 
        buildings::step::update_buildings
    }
};

use super::ui::step::update_ui;


pub fn update_game(world: &mut World, raylib_handle: &mut RaylibHandle, font: &Font) {
    let delta = raylib_handle.get_frame_time();

    update_tasks(world);
    update_villagers(world, delta);
    update_buildings(world);
    detect_body_collisions(world);
    detect_trigger_collisions(world);

    update_ui(world, font);
}
