use hecs::World;
use raylib::{RaylibHandle, RaylibThread, prelude::Vector2, text::Font};

use crate::{
    tilemap::generate_tilemap, TILESET, UI_ATLAS, 
    villagers::spawn_hauler, 
    buildings::spawn_warehouse, tasks::give_haul_task_to_idle
};

use super::{
    constants::*, 
    enums::*, 
    datatypes::Sprite, 
    ui::{spawn_toggle_button, spawn_button, SelectedHauler}, 
    input::{toggle_draw_collisions, toggle_debug_text}
};

pub fn world_setup(world: &mut World, raylib_handle: &mut RaylibHandle, thread: &RaylibThread) -> Result<Font, String> {
    if let Err(err) = load_tileset(raylib_handle, thread) {
        return Err(err)
    }

    let font: Font;
    match raylib_handle.load_font(thread, FONT_PATH) {
        Err(err) => return Err(err),
        Ok(f) => font = f,
    }

    generate_tilemap(world, 100, 100);

    spawn_buildings(world);
    spawn_villagers(world);
    spawn_ui(world);

    Ok(font)
}

pub fn load_tileset(raylib_handle: &mut RaylibHandle, thread: &RaylibThread) -> Result<(), String> {
    let result = raylib_handle
            .load_texture(&thread, TILESET_PATH);
    match result {
        Ok(texture) => {
            if let Err(_) = TILESET.set(texture) {
                error!("Failed to set tileset static variable.");
                return Err("Crash".to_string());
            }
            info!("Tileset loaded");
        },
        Err(..) => {
            error!("Failed to load tileset on path {:?}", TILESET_PATH);
            return Err("Crash".to_string());
        }
    }

    let result = raylib_handle
            .load_texture(&thread, UI_ATLAS_PATH);
    match result {
        Ok(texture) => {
            if let Err(_) = UI_ATLAS.set(texture) {
                error!("Failed to set ui atlas static variable.");
                return Err("Crash".to_string());
            }
            info!("UI atlas loaded");
        },
        Err(..) => {
            error!("Failed to load tileset on path {:?}", UI_ATLAS_PATH);
            return Err("Crash".to_string());
        }
    }

    Ok(())
}

pub fn spawn_villagers(world: &mut World) {
    let selected_hauler = spawn_hauler(
        world, 
        Vector2 { x: 48.0, y: 48.0 }, 
        Vector2 { x: 6.0, y: 12.0 },
        CollisionType::Body,
        None
    );
    world.spawn((SelectedHauler { hauler: selected_hauler },));
}

pub fn spawn_buildings(world: &mut World) {
    let sprite = Sprite::new(DEFAULT_IDLE_POINT, DEFAULT_IDLE_POINT_ATLAS_TILE);
    world.spawn((sprite,));
    
    spawn_warehouse(world, Vector2 { x: 304.0, y: 48.0 });
}

pub fn spawn_ui(world: &mut World) {
    spawn_toggle_button(
        world, 
        Vector2 { x: 10.0, y: (SCREEN_HEIGHT as f32) - 10.0 }, 
        Vector2 { x: 0.0, y: 0.0 },
        Some(toggle_draw_collisions),
        None,
    );

    spawn_toggle_button(
        world, 
        Vector2 { x: 50.0, y: (SCREEN_HEIGHT as f32) - 10.0 }, 
        Vector2 { x: 48.0, y: 0.0 },
        Some(toggle_debug_text),
        None,
    );

    spawn_button(
        world, 
        Vector2 { x: 90.0, y: (SCREEN_HEIGHT as f32) - 10.0 }, 
        Vector2 { x: 96.0, y: 0.0 },
        Some(give_haul_task_to_idle),
        None,
    );
}

