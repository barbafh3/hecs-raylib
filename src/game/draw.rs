use hecs::World;
use raylib::prelude::*;

use crate::engine::{
    draw::draw_sprites, 
    collision::draw_collisions, 
    ui::{draw::draw_mouse_selection, datatypes::SelectedHauler}, 
    enums::VillagerState, 
};

use super::{tilemap::draw_tilemap, villagers::datatypes::{GameItem, IdleState, LoadingState, CarryingState, WorkingState, Backpack}, constants::{TILE_SIZE, SCREEN_WIDTH_F}};


pub fn draw_game(world: &mut World, mode2d: &mut RaylibMode2D<RaylibDrawHandle>, camera: &Camera2D) {
    draw_tilemap(world, mode2d);
    draw_sprites(world, mode2d);
    draw_collisions(world, mode2d, TILE_SIZE);
    draw_mouse_selection(world, mode2d, &camera, TILE_SIZE);
}


pub fn draw_selected_hauler_state(world: &mut World, draw_handle: &mut RaylibDrawHandle, font: &Font) {
    let mut selected_hauler_query = world.query::<&SelectedHauler>();
    selected_hauler_query.into_iter().for_each(|(_, selected_hauler)| {
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
                    y: 12.0
                },
                font_size, 
                1.0,
                Color::BLACK
            );
            let text: String;
            match backpack_item {
                Some(item) => text = String::from(format!("Backpack Item: ({:?}, {:?})", item.resource, item.amount)),
                None => text = String::from("Backpack Item: None"),
            }
            let half_text_size = measure_text_ex(font, &text, font_size, 1.0) / 2.0;
            draw_handle.draw_text_ex(
                font, 
                &text,
                Vector2 {
                    x: (SCREEN_WIDTH_F / 2.0) - half_text_size.x,
                    y: 32.0
                },
                font_size, 
                1.0,
                Color::BLACK
            );
        }
    });
}
