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
use constants::{TILESET_PATH, SCREEN_WIDTH, SCREEN_HEIGHT, DEFAULT_IDLE_POINT_ATLAS_TILE, DEFAULT_IDLE_POINT, UI_ATLAS_PATH, TILE_SIZE};
use entities::{spawn_hauler, Sprite};
use enums::CollisionType;
use hecs::World;
use input::{read_inputs, toggle_draw_collisions, toggle_debug_text};
use raylib::{
    prelude::*, 
    math::Vector2,
};
use step::update_game;
use tilemap::{Tileset, generate_tilemap};
use ui::{UIAtlas, spawn_button, CameraZoom, toggle_debug_ui};

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
        update_game(&mut world, &mut raylib_handle);
        let mut draw_handle = raylib_handle.begin_drawing(&thread);
        {
            draw_game(&mut world, &mut draw_handle, &camera);
        }
        read_inputs(&mut world, &mut draw_handle, &mut camera);
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
        CollisionType::Body,
        None
    );

    spawn_hauler(
        world, 
        Vector2 { x: 28.0, y: 28.0 }, 
        Vector2 { x: 6.0, y: 12.0 },
        CollisionType::Trigger,
        None
    );

    spawn_hauler(
        world, 
        Vector2 { x: 28.0, y: 28.0 }, 
        Vector2 { x: 6.0, y: 12.0 },
        CollisionType::All,
        None
    );

    spawn_button(
        world, 
        Vector2 { x: 10.0, y: (SCREEN_HEIGHT as f32) - 10.0 }, 
        Vector2 { x: 0.0, y: 0.0 },
        Some(toggle_draw_collisions),
        None,
    );

    spawn_button(
        world, 
        Vector2 { x: 50.0, y: (SCREEN_HEIGHT as f32) - 10.0 }, 
        Vector2 { x: 48.0, y: 0.0 },
        Some(toggle_debug_text),
        None,
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

    let ui_atlas: Texture2D;
    let result = raylib_handle
            .load_texture(&thread, UI_ATLAS_PATH);
    match result {
        Ok(texture) => {
            ui_atlas = texture;
            info!("UI atlas loaded");
        },
        Err(..) => {
            error!("Failed to load tileset on path {:?}", UI_ATLAS_PATH);
            return Err("Crash".to_string());
        }
    }

    world.spawn((Tileset, tileset));
    world.spawn((UIAtlas, ui_atlas));

    Ok(())
}
