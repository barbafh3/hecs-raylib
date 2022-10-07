use hecs::World;
use raylib::{RaylibHandle, RaylibThread, prelude::Vector2};

use crate::{
    engine::{
        enums::CollisionType, 
        ui::{datatypes::SelectedHauler, spawn_toggle_button, spawn_button}, 
        datatypes::Sprite
    },
    game::{
        constants::*,
        tilemap::generate_tilemap, 
        tasks::{OpenTasks, give_haul_task_to_idle}, 
        villagers::spawn_hauler, 
        buildings::{spawn_warehouse, spawn_house},
        input::{toggle_draw_collisions, toggle_debug_text}, 
    }, 
};

use super::constants::TILE_SIZE;

pub fn game_setup(
    world: &mut World, 
    raylib_handle: &mut RaylibHandle, 
    thread: &RaylibThread,
) {
    generate_tilemap(world, 100, 100);

    world.spawn((OpenTasks::default(),));

    spawn_buildings(world);
    spawn_villagers(world);
    spawn_ui(world);
}

pub fn spawn_villagers(world: &mut World) {
    let selected_hauler = spawn_hauler(
        world, 
        Vector2 { x: 48.0, y: 48.0 }, 
        Vector2 { x: 6.0, y: 12.0 },
        CollisionType::Trigger,
        None
    );
    world.spawn((SelectedHauler { hauler: selected_hauler },));
}

pub fn spawn_buildings(world: &mut World) {
    let sprite = Sprite::new(DEFAULT_IDLE_POINT, DEFAULT_IDLE_POINT_ATLAS_TILE, TILE_SIZE);
    world.spawn((sprite,));
    
    spawn_warehouse(world, Vector2 { x: 304.0, y: 48.0 }, false);
    spawn_warehouse(world, Vector2 { x: 304.0, y: 248.0 }, true);

    spawn_house(world, Vector2 { x: 16.0, y: 192.0 });
}

pub fn spawn_ui(world: &mut World) {
    spawn_toggle_button(
        world, 
        Vector2 { x: 10.0, y: (SCREEN_HEIGHT as f32) - 10.0 }, 
        Vector2::zero(),
        0,
        Vector2 { x: 0.0, y: 0.0 },
        TILE_SIZE,
        Some(toggle_draw_collisions),
        None,
    );

    spawn_toggle_button(
        world, 
        Vector2 { x: 50.0, y: (SCREEN_HEIGHT as f32) - 10.0 }, 
        Vector2::zero(),
        0,
        Vector2 { x: 48.0, y: 0.0 },
        TILE_SIZE,
        Some(toggle_debug_text),
        None,
    );

    spawn_button(
        world, 
        Vector2 { x: 90.0, y: (SCREEN_HEIGHT as f32) - 10.0 }, 
        Vector2::zero(),
        0,
        Vector2 { x: 96.0, y: 0.0 },
        TILE_SIZE,
        Some(give_haul_task_to_idle),
        None,
    );
}

