use hecs::World;
use raylib::prelude::*;

use crate::{tilemap::draw_tilemap, entities::draw_sprites, ui::{draw_mouse_selection, draw_ui}, collision::draw_collisions};


pub fn draw_game(world: &mut World, draw_handle: &mut RaylibDrawHandle, camera: &Camera2D) {
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

    draw_ui(world, draw_handle);
}
