pub mod villagers;
pub mod tilemap;
pub mod tasks;
pub mod buildings;
pub mod game_core;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use game_core::{draw::draw_game, step::update_game, constants::*, startup::world_setup};
use once_cell::sync::OnceCell;
use hecs::World;
use game_core::input::game_input;
use raylib::{
    prelude::*, 
    math::Vector2,
};
use game_core::ui::CameraZoom;

// GLOBAL TEXTURES
pub static TILESET: OnceCell<Texture2D> = OnceCell::new();
pub static UI_ATLAS: OnceCell<Texture2D> = OnceCell::new();

fn main() -> Result<(), String>{
    pretty_env_logger::init();

    let (mut raylib_handle, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hecs Test")
        .build();

    let mut world = World::new();

    let font: Font;
    match world_setup(&mut world, &mut raylib_handle, &thread) {
        Err(err) => return Err(err),
        Ok(f) => font = f,
    }

    let zoom = 2.0;

    let mut camera = Camera2D {
        offset: Vector2::zero(), 
        target: Vector2::zero(), 
        rotation: 0.0, 
        zoom
    };
    world.spawn((CameraZoom(zoom),));

    raylib_handle.set_target_fps(60);

    while !raylib_handle.window_should_close() {
        game_input(&mut world, &mut raylib_handle, &mut camera);
        update_game(&mut world, &mut raylib_handle);

        let mut draw_handle = raylib_handle.begin_drawing(&thread);
        draw_game(&mut world, &mut draw_handle, &camera, &font);
    }

    Ok(())
}
