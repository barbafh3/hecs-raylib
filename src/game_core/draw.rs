use hecs::World;
use raylib::prelude::*;

use crate::{
    tilemap::draw_tilemap, 
    game_core::ui::{draw_mouse_selection, draw_ui}, 
    game_core::collision::draw_collisions, TILESET
};

use super::datatypes::Sprite;


pub fn draw_game(world: &mut World, draw_handle: &mut RaylibDrawHandle, camera: &Camera2D, font: &Font) {
    draw_handle.clear_background(Color::RAYWHITE);

    {
        // CAMERA BLOCK
        let mut mode2d = draw_handle.begin_mode2D(camera);

        let mouse_pos = mode2d.get_screen_to_world2D(mode2d.get_mouse_position(), camera);

        draw_tilemap(world, &mut mode2d);
        draw_sprites(world, &mut mode2d);
        draw_collisions(world, &mut mode2d);

        draw_mouse_selection(world, &mut mode2d, mouse_pos);
    }

    draw_ui(world, draw_handle, font);
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
