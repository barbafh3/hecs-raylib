use hecs::World;
use raylib::{prelude::*, RaylibHandle, RaylibThread};

use crate::{
    engine::{
        datatypes::Sprite,
        enums::CollisionType,
        ui::{spawn_button, spawn_label, spawn_toggle_button},
    },
    game::{
        buildings::{house::spawn_finished_house, warehouse::spawn_finished_warehouse},
        constants::*,
        input::{toggle_debug_text, toggle_draw_collisions},
        tasks::OpenTasks,
        tilemap::generate_tilemap,
        villagers::hauler::spawn_hauler,
    },
};

use super::{
    buildings::house::spawn_house_placement,
    constants::TILE_SIZE,
    ui::datatypes::{ActiveTaskCountLabel, GlobalStorageLabel, IdleTaskCountLabel, SelectedHauler},
};

pub fn game_setup(world: &mut World, _raylib_handle: &mut RaylibHandle, _thread: &RaylibThread) {
    generate_tilemap(world, 100, 100);

    world.spawn((OpenTasks::default(),));

    spawn_buildings(world);
    spawn_villagers(world);
    spawn_ui(world);
}

pub fn spawn_villagers(world: &mut World) {
    let selected_hauler = spawn_hauler(
        world,
        Vector2 { x: 48.0, y: 48.0 },
        Vector2 { x: 6.0, y: 12.0 },
        CollisionType::Trigger,
        None,
    );
    world.spawn((SelectedHauler {
        hauler: selected_hauler,
    },));
}

pub fn spawn_buildings(world: &mut World) {
    let sprite = Sprite::new(DEFAULT_IDLE_POINT, DEFAULT_IDLE_POINT_ATLAS_TILE, TILE_SIZE);
    world.spawn((sprite,));

    spawn_finished_warehouse(world, Vector2 { x: 304.0, y: 256.0 }, true);

    spawn_finished_house(world, Vector2 { x: 16.0, y: 192.0 });
}

pub fn spawn_ui(world: &mut World) {
    spawn_toggle_button(
        world,
        Vector2 {
            x: 10.0,
            y: (SCREEN_HEIGHT as f32) - 10.0,
        },
        Vector2::zero(),
        0,
        Vector2 { x: 0.0, y: 0.0 },
        TILE_SIZE,
        Some(toggle_draw_collisions),
        None,
    );

    spawn_toggle_button(
        world,
        Vector2 {
            x: 50.0,
            y: (SCREEN_HEIGHT as f32) - 10.0,
        },
        Vector2::zero(),
        0,
        Vector2 { x: 3.0, y: 0.0 },
        TILE_SIZE,
        Some(toggle_debug_text),
        None,
    );

    spawn_button(
        world,
        Vector2 {
            x: 90.0,
            y: (SCREEN_HEIGHT as f32) - 10.0,
        },
        Vector2::zero(),
        0,
        Vector2 { x: 3.0, y: 1.0 },
        TILE_SIZE,
        Some(|world| {
            spawn_house_placement(world, Vector2 { x: 0.0, y: 0.0 });
        }),
        None,
    );

    spawn_button(
        world,
        Vector2 {
            x: 130.0,
            y: (SCREEN_HEIGHT as f32) - 10.0,
        },
        Vector2::zero(),
        0,
        Vector2 { x: 0.0, y: 1.0 },
        TILE_SIZE,
        Some(|world| {
            spawn_finished_warehouse(world, Vector2 { x: 304.0, y: 48.0 }, false);
        }),
        None,
    );

    let label = spawn_label(
        world,
        Vector2 {
            x: SCREEN_WIDTH_F - 15.0,
            y: 12.0,
        },
        Vector2::zero(),
        0,
        10.0,
        1.0,
        Color::BLACK,
    );
    world.insert(label, (IdleTaskCountLabel,)).unwrap();

    let label = spawn_label(
        world,
        Vector2 {
            x: SCREEN_WIDTH_F - 15.0,
            y: 32.0,
        },
        Vector2::zero(),
        0,
        10.0,
        1.0,
        Color::BLACK,
    );
    world.insert(label, (ActiveTaskCountLabel,)).unwrap();

    let label = spawn_label(
        world,
        Vector2 {
            x: SCREEN_WIDTH_F / 2.0,
            y: 12.0,
        },
        Vector2::zero(),
        0,
        10.0,
        1.0,
        Color::BLACK,
    );
    world.insert(label, (GlobalStorageLabel,)).unwrap();
}
