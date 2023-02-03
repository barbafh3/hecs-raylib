pub mod engine;
pub mod game;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use engine::{draw::engine_draw, startup::world_setup, ui::datatypes::CameraZoom};
use game::{
    constants::{FONT_PATH, SCREEN_HEIGHT, SCREEN_WIDTH, TILESET_PATH, TILE_SIZE, UI_ATLAS_PATH},
    draw::draw_game,
    input::handle_input,
    scenes::{ActiveScene, Scene},
    startup::game_setup,
    step::update_game,
};
use hecs::World;
use raylib::{math::Vector2, prelude::*};

fn main() -> Result<(), String> {
    pretty_env_logger::init();

    let (mut raylib_handle, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hecs Test")
        .build();

    let mut world = World::new();

    // Set staring scene
    world.spawn((ActiveScene {
        scene: Scene::MainMenu,
    },));

    let font: Font = world_setup(
        &mut world,
        &mut raylib_handle,
        &thread,
        &FONT_PATH,
        &TILESET_PATH,
        &UI_ATLAS_PATH,
        Some(game_setup),
    )?;

    let zoom = 2.0;

    let mut camera = Camera2D {
        offset: Vector2::zero(),
        target: Vector2::zero(),
        rotation: 0.0,
        zoom,
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
            None,
        );
    }

    Ok(())
}
