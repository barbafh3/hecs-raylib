pub mod game;
pub mod engine;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use engine::{
    ui::datatypes::CameraZoom, 
    draw::engine_draw,
    startup::world_setup
};
use game::{
    constants::{SCREEN_WIDTH, SCREEN_HEIGHT, FONT_PATH, TILESET_PATH, UI_ATLAS_PATH, TILE_SIZE}, 
    input::handle_input, 
    step::update_game, 
    draw::draw_game, startup::game_setup, 
};
use hecs::World;
use raylib::{
    prelude::*, 
    math::Vector2,
};

fn main() -> Result<(), String>{
    pretty_env_logger::init();

    let (mut raylib_handle, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hecs Test")
        .build();

    let mut world = World::new();

    let font: Font = world_setup(
        &mut world, 
        &mut raylib_handle, 
        &thread, 
        &FONT_PATH, 
        &TILESET_PATH, 
        &UI_ATLAS_PATH, 
        Some(game_setup)
        )?; 

    let zoom = 2.0;

    let mut camera = Camera2D {
        offset: Vector2::zero(), 
        target: Vector2::zero(), 
        rotation: 0.0, 
        zoom
    };
    world.spawn((CameraZoom(zoom),));

    raylib_handle.set_target_fps(75);

    while !raylib_handle.window_should_close() {
        handle_input(&mut world, &mut raylib_handle, &mut camera)?;
        update_game(&mut world, &mut raylib_handle, &font)?;
        engine_draw(
            &mut world, 
            &mut raylib_handle.begin_drawing(&thread), 
            &camera, 
            &font,
            TILE_SIZE,
            Some(draw_game),
            None
        );
    }

    Ok(())
}
