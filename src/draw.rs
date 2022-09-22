use hecs::World;
use raylib::prelude::*;

use crate::{tilemap::draw_tilemap, entities::draw_sprites, ui::{draw_mouse_selection, draw_ui}};


pub fn draw_game(world: &mut World, raylib_handle: &mut RaylibHandle, thread: &RaylibThread, camera: &Camera2D) {
    let mut draw_handle = raylib_handle.begin_drawing(&thread);
    draw_handle.clear_background(Color::RAYWHITE);

    {
        // CAMERA BLOCK
        let mut mode2d = draw_handle.begin_mode2D(camera);

        let mouse_pos = mode2d.get_screen_to_world2D(mode2d.get_mouse_position(), camera);

        draw_tilemap(world, &mut mode2d);
        draw_sprites(world, &mut mode2d);

        draw_mouse_selection(world, &mut mode2d, mouse_pos);
    }

    draw_ui(world, &mut draw_handle);
}
