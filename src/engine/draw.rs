use hecs::World;
use raylib::ffi::Font;
use raylib::prelude::*;

use super::{datatypes::Sprite, TILESET};

/// Engine function that draws core components to the screen. It receives the world and
/// raylib core rendering components.
///
/// Can be extended by receiving methods for in-game (inside camera) or UI (outside camera) rendering.
pub fn engine_draw(
    world: &mut World, 
    draw_handle: &mut RaylibDrawHandle, 
    camera: &Camera2D, 
    font: &Font,
    m_draw_game_func: Option<fn(&mut World, &mut RaylibMode2D<RaylibDrawHandle>, &Camera2D) -> ()>,
    m_draw_ui_func: Option<fn(&mut World, &mut RaylibDrawHandle, &Camera2D, &Font) -> ()>
) {
    draw_handle.clear_background(Color::RAYWHITE);

    {
        let mut mode2d = draw_handle.begin_mode2D(camera);
        if let Some(draw_game_func) = m_draw_game_func {
            (draw_game_func)(world, &mut mode2d, camera);
        }
    }

    if let Some(draw_ui_func) = m_draw_ui_func {
        (draw_ui_func)(world, draw_handle, camera, font);
    }
}


pub fn draw_sprites(world: &mut World,draw_handle: &mut RaylibMode2D<RaylibDrawHandle>) {
    let mut query = world.query::<&Sprite>();
    query.into_iter().for_each(|(_, sprite)| {
        draw_handle.draw_texture_rec(
            TILESET.get().unwrap(),
            sprite.rect,
            sprite.position,
            Color::WHITE);
    });
}
