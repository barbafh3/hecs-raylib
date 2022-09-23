pub mod entities;
pub mod tilemap;
pub mod constants;
pub mod ui;
pub mod input;
pub mod collision;
pub mod step;
pub mod draw;
pub mod enums;
pub mod tasks;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use draw::draw_game;
use constants::{TILESET_PATH, SCREEN_WIDTH, SCREEN_HEIGHT, DEFAULT_IDLE_POINT_ATLAS_TILE, DEFAULT_IDLE_POINT};
use entities::{spawn_hauler, Sprite};
use hecs::World;
use input::read_inputs;
use raylib::{
    prelude::*, 
    math::Vector2,
};
use step::update_game;
use tilemap::{Tileset, generate_tilemap};

fn main() -> Result<(), String>{
    pretty_env_logger::init();

    let (mut raylib_handle, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hecs Test")
        .build();

    let mut world = World::new();

    let result = world_setup(&mut world, &mut raylib_handle, &thread);
    if let Err(err) = result {
        return Err(err)
    }

    let mut camera = Camera2D {
        offset: Vector2::zero(), 
        target: Vector2::zero(), 
        rotation: 0.0, 
        zoom: 2.0
    };

    raylib_handle.set_target_fps(60);

    while !raylib_handle.window_should_close() {
        read_inputs(&mut world, &mut raylib_handle, &mut camera);
        update_game(&mut world, &mut raylib_handle);
        draw_game(&mut world, &mut raylib_handle, &thread, &camera);
    }

    Ok(())
}

fn world_setup(world: &mut World, raylib_handle: &mut RaylibHandle, thread: &RaylibThread) -> Result<(), String> {
    if let Err(err) = load_tileset(world, raylib_handle, thread) {
        return Err(err)
    }

    generate_tilemap(world, 100, 100);

    let sprite = Sprite::new(DEFAULT_IDLE_POINT, DEFAULT_IDLE_POINT_ATLAS_TILE);
    world.spawn((sprite,));

    spawn_hauler(
        world, 
        Vector2 { x: 48.0, y: 48.0 }, 
        Vector2 { x: 6.0, y: 12.0 },
        None
    );

    spawn_hauler(
        world, 
        Vector2 { x: 28.0, y: 28.0 }, 
        Vector2 { x: 6.0, y: 12.0 },
        None
    );

    Ok(())
}

pub fn load_tileset(world: &mut World, raylib_handle: &mut RaylibHandle, thread: &RaylibThread) -> Result<(), String> {
    let tileset: Texture2D;
    let result = raylib_handle
            .load_texture(&thread, TILESET_PATH);
    match result {
        Ok(texture) => {
            tileset = texture;
            info!("Tileset loaded");
        },
        Err(..) => {
            error!("Failed to load tileset on path {:?}", TILESET_PATH);
            return Err("Crash".to_string());
        }
    }

    world.spawn((Tileset, tileset));

    Ok(())
}
