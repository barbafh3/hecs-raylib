use hecs::{World, Entity};
use raylib::prelude::RaylibDraw;
use raylib::{prelude::*, texture::Texture2D};

use crate::constants::TILE_SIZE;
use crate::tilemap::Tileset;

// TAGS ------
pub struct MouseSelection;
pub struct DebugUI;

// FUNCTIONS ------
pub fn draw_ui(world: &mut World, draw_handle: &mut RaylibDrawHandle) {
    // draw_handle.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

    let mut debug_query = world.query::<&DebugUI>();
    for (_, _) in debug_query.into_iter() {
        draw_handle.draw_fps(10, 10);
    }
}

pub fn draw_mouse_selection(world: &mut World, draw_handle: &mut RaylibMode2D<RaylibDrawHandle>, mouse_pos: Vector2) {
    let mut selection_query = world.query::<&MouseSelection>();
    for (_, _) in selection_query.into_iter() {
        let mut query = world.query::<(&Tileset, &Texture2D)>();
        let mut current_tile_x = (mouse_pos.x / TILE_SIZE) as i32;
        let mut current_tile_y = (mouse_pos.y / TILE_SIZE) as i32;

        if mouse_pos.x < 0.0 {
            current_tile_x -= 1;
        }
        if mouse_pos.y < 0.0 {
            current_tile_y -= 1;
        }

        let tile_position = Vector2 { 
            x: (current_tile_x) as f32 * TILE_SIZE, 
            y: (current_tile_y) as f32 * TILE_SIZE 
        };

        let rect = Rectangle {
            x: 96.0,
            y: 672.0,
            height: TILE_SIZE,
            width: TILE_SIZE,
        };
        for (_, (_, tileset)) in query.into_iter() {
            draw_handle.draw_texture_rec(
                tileset,
                rect,
                tile_position,
                Color::WHITE
            );
        }
    }
}

pub fn toggle_mouse_selection(world: &mut World) {
    let mut entity_list: Vec<Entity> = vec![];
    {
        let mut selection_query = world.query::<&MouseSelection>();
        for (entity, _) in selection_query.into_iter() {
            {
                entity_list.push(entity);
            }
        }
    }
    if entity_list.len() <= 0 {
        world.spawn((MouseSelection,));
    } else {
        for entity in entity_list {
            world.despawn(entity).unwrap();
        }
    }
}

pub fn toggle_debug_ui(world: &mut World, enable: bool) {
    let mut entity_list: Vec<Entity> = vec![];
    {
        let mut selection_query = world.query::<&DebugUI>();
        for (entity, _) in selection_query.into_iter() {
            {
                entity_list.push(entity);
            }
        }
    }
    if enable {
        if entity_list.len() <= 0 {
            world.spawn((DebugUI,));
        }
    } else {
        for entity in entity_list {
            world.despawn(entity).unwrap();
        }
    }
}
