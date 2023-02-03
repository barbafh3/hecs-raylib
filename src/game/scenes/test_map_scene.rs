use hecs::World;
use raylib::{text::Font, RaylibHandle, RaylibThread};

use crate::{
    engine::collision::{detect_body_collisions, detect_trigger_collisions},
    game::{
        buildings::step::update_buildings,
        startup::{spawn_buildings, spawn_ui, spawn_villagers},
        tasks::{update_tasks, OpenTasks},
        tilemap::generate_tilemap,
        ui::step::update_ui,
        villagers::step::update_villagers,
    },
};

pub fn update_test_map_scene(world: &mut World, font: &Font, delta: f32) -> Result<(), String> {
    update_tasks(world)?;
    update_villagers(world, delta);
    update_buildings(world)?;
    detect_body_collisions(world);
    detect_trigger_collisions(world);

    update_ui(world, font)?;

    Ok(())
}

pub fn setup_test_map(
    world: &mut World,
    _raylib_handle: &mut RaylibHandle,
    _thread: &RaylibThread,
) -> Result<(), String> {
    generate_tilemap(world, 1024, 1024);

    world.spawn((OpenTasks::default(),));

    spawn_buildings(world);
    spawn_villagers(world);
    spawn_ui(world)?;

    Ok(())
}
