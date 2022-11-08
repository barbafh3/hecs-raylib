use hecs::World;
use raylib::prelude::*;

use crate::engine::{
    collision::{draw_collisions, TriggerCollision},
    datatypes::Sprite,
    enums::VillagerState,
    ui::draw::draw_mouse_selection,
    TILESET,
};

use super::{
    buildings::datatypes::{ConstructionPlacement, ConstructionStorage, OngoingConstruction},
    constants::{CONSTRUCTION_RECT, SCREEN_WIDTH_F, TILE_SIZE},
    ui::datatypes::SelectedHauler,
    villagers::datatypes::{
        Backpack, CarryingState, GameItem, IdleState, LoadingState, WorkingState,
    }, 
    tilemap::{draw_tilemap, check_visible_tilemap_chunks},
};

pub fn draw_game(
    world: &mut World,
    mode2d: &mut RaylibMode2D<RaylibDrawHandle>,
    camera: &Camera2D,
) {
    check_visible_tilemap_chunks(world, mode2d, camera);
    draw_tilemap(world, mode2d);
    draw_sprites(world, mode2d);
    draw_construction_placement(world, mode2d);
    draw_construction(world, mode2d);
    draw_collisions(world, mode2d, TILE_SIZE);
    draw_mouse_selection(world, mode2d, &camera, TILE_SIZE);
}

pub fn draw_selected_hauler_state(
    world: &mut World,
    draw_handle: &mut RaylibDrawHandle,
    font: &Font,
) {
    let mut selected_hauler_query = world.query::<&SelectedHauler>();
    selected_hauler_query
        .into_iter()
        .for_each(|(_, selected_hauler)| {
            let mut m_state: Option<VillagerState> = None;
            let mut backpack_item: Option<GameItem> = None;

            let idle_query = world.get::<IdleState>(selected_hauler.hauler);
            if let Ok(_) = idle_query {
                m_state = Some(VillagerState::Idle);
            }
            let loading_query = world.get::<LoadingState>(selected_hauler.hauler);
            if let Ok(_) = loading_query {
                m_state = Some(VillagerState::Loading);
            }
            let carrying_query = world.get::<CarryingState>(selected_hauler.hauler);
            if let Ok(_) = carrying_query {
                m_state = Some(VillagerState::Carrying);
            }
            let working_query = world.get::<WorkingState>(selected_hauler.hauler);
            if let Ok(_) = working_query {
                m_state = Some(VillagerState::Working);
            }
            let backpack_query = world.get::<Backpack>(selected_hauler.hauler);
            if let Ok(bp) = backpack_query {
                backpack_item = bp.clone_item();
            }

            if let Some(state) = m_state {
                let font_size: f32 = 11.4;

                let text = String::from(format!("Hauler state: {:?}", state));
                let half_text_size = measure_text_ex(font, &text, font_size, 1.0) / 2.0;
                draw_handle.draw_text_ex(
                    font,
                    &text,
                    Vector2 {
                        x: ((SCREEN_WIDTH_F / 2.0) - half_text_size.x),
                        y: 12.0,
                    },
                    font_size,
                    1.0,
                    Color::BLACK,
                );
                let text: String;
                match backpack_item {
                    Some(item) => {
                        text = String::from(format!(
                            "Backpack Item: ({:?}, {:?})",
                            item.resource, item.amount
                        ))
                    }
                    None => text = String::from("Backpack Item: None"),
                }
                let half_text_size = measure_text_ex(font, &text, font_size, 1.0) / 2.0;
                draw_handle.draw_text_ex(
                    font,
                    &text,
                    Vector2 {
                        x: (SCREEN_WIDTH_F / 2.0) - half_text_size.x,
                        y: 32.0,
                    },
                    font_size,
                    1.0,
                    Color::BLACK,
                );
            }
        });
}

pub fn draw_construction_placement(world: &mut World, mode2d: &mut RaylibMode2D<RaylibDrawHandle>) {
    let mut query = world
        .query::<(&TriggerCollision, &Sprite)>()
        .with::<ConstructionStorage>()
        .with::<ConstructionPlacement>();
    query.into_iter().for_each(|(_, (trigger, sprite))| {
        let mut color = if trigger.colliding {
            Color::RED
        } else {
            Color::GREEN
        };
        color.a = 170;

        mode2d.draw_texture_rec(
            TILESET.get().unwrap(),
            CONSTRUCTION_RECT,
            sprite.position,
            color,
        );
    });
}

pub fn draw_construction(world: &mut World, mode2d: &mut RaylibMode2D<RaylibDrawHandle>) {
    let mut query = world
        .query::<&Sprite>()
        .with::<ConstructionStorage>()
        .without::<ConstructionPlacement>();
    query.into_iter().for_each(|(_, sprite)| {
        mode2d.draw_texture_rec(
            TILESET.get().unwrap(),
            CONSTRUCTION_RECT,
            sprite.position,
            Color::WHITE,
        );
    });

    let mut query = world
        .query::<&Sprite>()
        .with::<OngoingConstruction>()
        .without::<ConstructionStorage>()
        .without::<ConstructionPlacement>();
    query.into_iter().for_each(|(_, sprite)| {
        mode2d.draw_texture_rec(
            TILESET.get().unwrap(),
            CONSTRUCTION_RECT,
            sprite.position,
            Color::WHITE,
        );
    });
}

pub fn draw_sprites(world: &mut World, mode2d: &mut RaylibMode2D<RaylibDrawHandle>) {
    let mut query = world
        .query::<&Sprite>()
        .without::<ConstructionStorage>()
        .without::<OngoingConstruction>();
    query.into_iter().for_each(|(_, sprite)| {
        mode2d.draw_texture_rec(
            TILESET.get().unwrap(),
            sprite.rect,
            sprite.position,
            Color::WHITE,
        );
    });
}
