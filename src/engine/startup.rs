use hecs::World;
use raylib::{RaylibThread, RaylibHandle, prelude::Font};

use crate::engine::{TILESET, UI_ATLAS};

pub fn world_setup<'a>(
    world: &mut World, 
    raylib_handle: &mut RaylibHandle, 
    thread: &RaylibThread,
    font_path: &str,
    tileset_path: &str,
    ui_atlas_path: &str,
    m_game_setup: Option<fn(&mut World, &mut RaylibHandle, thread: &RaylibThread) -> ()>
) -> Result<Font, String> {
    load_tileset(raylib_handle, thread, tileset_path, ui_atlas_path)?;

    if let Some(game_setup) = m_game_setup { 
        (game_setup)(world, raylib_handle, thread);
    }

    return raylib_handle.load_font(thread, &font_path);
}

fn load_tileset(
    raylib_handle: &mut RaylibHandle, 
    thread: &RaylibThread, 
    tileset_path: &str, 
    ui_atlas_path: &str
) -> Result<(), String> {
    let result = raylib_handle
            .load_texture(&thread, &tileset_path);
    match result {
        Ok(texture) => {
            if let Err(_) = TILESET.set(texture) {
                error!("Failed to set tileset static variable.");
                return Err("Crash".to_string());
            }
            info!("Tileset loaded");
        },
        Err(..) => {
            error!("Failed to load tileset on path {:?}", tileset_path);
            return Err("Crash".to_string());
        }
    }

    let result = raylib_handle
            .load_texture(&thread, &ui_atlas_path);
    match result {
        Ok(texture) => {
            if let Err(_) = UI_ATLAS.set(texture) {
                error!("Failed to set ui atlas static variable.");
                return Err("Crash".to_string());
            }
            info!("UI atlas loaded");
        },
        Err(..) => {
            error!("Failed to load tileset on path {:?}", ui_atlas_path);
            return Err("Crash".to_string());
        }
    }

    Ok(())
}
