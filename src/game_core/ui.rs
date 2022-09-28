use hecs::{World, Entity};
use raylib::prelude::RaylibDraw;
use raylib::prelude::*;

use crate::villagers::datatypes::{IdleState, LoadingState, CarryingState, WorkingState, Backpack, GameItem};
use crate::{TILESET, UI_ATLAS};
use crate::game_core::constants::TILE_SIZE;
use crate::game_core::enums::ButtonState;

use super::constants::SCREEN_WIDTH_F;
use super::enums::VillagerState;

// TAGS ------
pub struct MouseSelection;
pub struct DebugUI;
pub struct UIAtlas;
pub struct CameraZoom(pub(crate) f32);

// STRUCTS ------
pub struct SelectedHauler {
    pub hauler: Entity
}

pub struct Button {
    pub rect: Rectangle,
    pub state: ButtonState,
    pub position: Vector2,
    pub action: Option<fn(&mut World) -> ()>,
    pub handle_action: Option<fn(&mut World, &mut RaylibHandle) -> ()>,
}

impl Button {
    pub fn new(
        position: Vector2, 
        atlas_tile: Vector2, 
        action: Option<fn(&mut World) -> ()>,
        handle_action: Option<fn(&mut World, &mut RaylibHandle) -> ()>,
    ) -> Button {
        Button {
            position,
            state: ButtonState::Normal,
            action,
            handle_action,
            rect: Rectangle {
                x: atlas_tile.x * TILE_SIZE,
                y: atlas_tile.y * TILE_SIZE,
                width: TILE_SIZE,
                height: TILE_SIZE
            }
        }
    }
}

pub struct ToggleButton {
    pub rect: Rectangle,
    pub state: ButtonState,
    pub position: Vector2,
    pub action: Option<fn(&mut World) -> ()>,
    pub handle_action: Option<fn(&mut World, &mut RaylibHandle) -> ()>,
}

impl ToggleButton {
    pub fn new(
        position: Vector2, 
        atlas_tile: Vector2, 
        action: Option<fn(&mut World) -> ()>,
        handle_action: Option<fn(&mut World, &mut RaylibHandle) -> ()>,
    ) -> ToggleButton {
        ToggleButton {
            position,
            state: ButtonState::Normal,
            action,
            handle_action,
            rect: Rectangle {
                x: atlas_tile.x * TILE_SIZE,
                y: atlas_tile.y * TILE_SIZE,
                width: TILE_SIZE,
                height: TILE_SIZE
            }
        }
    }
}

// FUNCTIONS ------
pub fn draw_ui(world: &mut World, draw_handle: &mut RaylibDrawHandle, font: &Font) {
    // draw_handle.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

    {
        let mut debug_query = world.query::<&DebugUI>();
        debug_query.into_iter().for_each(|_| {
            draw_handle.draw_fps(10, 10);
        });
    }

    draw_selected_hauler_state(world, draw_handle, font);
    draw_ui_buttons(world, draw_handle);
    draw_ui_toggle_buttons(world, draw_handle);
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

pub fn draw_ui_buttons(world: &mut World, draw_handle: &mut RaylibDrawHandle) {
    let mut zoom_query = world.query::<&CameraZoom>();
    let (_, CameraZoom(zoom)) = zoom_query.into_iter().nth(0).unwrap();
    let mut query = world.query::<&Button>();
    query.into_iter().for_each(|(_, button)| {
        let mut src = button.rect.clone();
        match button.state {
            ButtonState::Hovered => {
                src.x = src.x + 16.0;
                src.y = 0.0;
            },
            ButtonState::Pressed => {
                src.x = src.x + 32.0;
                src.y = 0.0;
            },
            _ => {}
        }
        let dest = Rectangle { 
            x: button.position.x,
            y: button.position.y - (TILE_SIZE * zoom),
            width: TILE_SIZE * zoom,
            height: TILE_SIZE * zoom,
        };
        draw_handle.draw_texture_pro(
            UI_ATLAS.get().unwrap(),
            src,
            dest,
            Vector2::zero(),
            0.0,
            Color::WHITE);
    });
}

pub fn draw_ui_toggle_buttons(world: &mut World, draw_handle: &mut RaylibDrawHandle) {
    let mut zoom_query = world.query::<&CameraZoom>();
    let (_, CameraZoom(zoom)) = zoom_query.into_iter().nth(0).unwrap();
    let mut query = world.query::<&ToggleButton>();
    for (_, button) in query.into_iter() {
        let mut src = button.rect.clone();
        match button.state {
            ButtonState::Hovered => {
                src.x = src.x + 16.0;
                src.y = 0.0;
            },
            ButtonState::Pressed => {
                src.x = src.x + 32.0;
                src.y = 0.0;
            },
            ButtonState::Toggled => {
                src.x = src.x + 32.0;
                src.y = 0.0;
            },
            _ => {}
        }
        let dest = Rectangle { 
            x: button.position.x,
            y: button.position.y - (TILE_SIZE * zoom),
            width: TILE_SIZE * zoom,
            height: TILE_SIZE * zoom,
        };
        draw_handle.draw_texture_pro(
            UI_ATLAS.get().unwrap(),
            src,
            dest,
            Vector2::zero(),
            0.0,
            Color::WHITE);
    }
}

pub fn draw_mouse_selection(world: &mut World, draw_handle: &mut RaylibMode2D<RaylibDrawHandle>, mouse_pos: Vector2) {
    let mut selection_query = world.query::<&MouseSelection>();
    for (_, _) in selection_query.into_iter() {
        let mut current_tile_x = (mouse_pos.x / TILE_SIZE) as i32;
        let mut current_tile_y = (mouse_pos.y / TILE_SIZE) as i32;

        if mouse_pos.x < 0.0 {
            current_tile_x -= 1;
        }
        if mouse_pos.y < 0.0 {
            current_tile_y -= 1;
        }

        let tile_position = Vector2 { 
            x: (current_tile_x) as f32 * TILE_SIZE, 
            y: (current_tile_y) as f32 * TILE_SIZE 
        };

        let rect = Rectangle {
            x: 96.0,
            y: 672.0,
            height: TILE_SIZE,
            width: TILE_SIZE,
        };
        draw_handle.draw_texture_rec(
            TILESET.get().unwrap(),
            rect,
            tile_position,
            Color::WHITE
        );
    }
}

pub fn spawn_button(
    world: &mut World, 
    position: Vector2, 
    atlas_tile: Vector2, 
    action: Option<fn(&mut World) -> ()>,
    handle_action: Option<fn(&mut World, &mut RaylibHandle) -> ()>,
) -> Entity {
    let button = Button::new(
        position,
        atlas_tile,
        action,
        handle_action,
    );

    return world.spawn((button,));
}

pub fn spawn_toggle_button(
    world: &mut World, 
    position: Vector2, 
    atlas_tile: Vector2, 
    action: Option<fn(&mut World) -> ()>,
    handle_action: Option<fn(&mut World, &mut RaylibHandle) -> ()>,
) -> Entity {
    let button = ToggleButton::new(
        position,
        atlas_tile,
        action,
        handle_action,
    );

    return world.spawn((button,));
}

pub fn toggle_mouse_selection(world: &mut World) {
    let mut entity_list: Vec<Entity> = vec![];
    {
        let mut selection_query = world.query::<&MouseSelection>();
        for (entity, _) in selection_query.into_iter() {
            {
                entity_list.push(entity);
            }
        }
    }
    if entity_list.len() <= 0 {
        world.spawn((MouseSelection,));
    } else {
        for entity in entity_list {
            world.despawn(entity).unwrap();
        }
    }
}

pub fn toggle_debug_ui(world: &mut World) {
    let mut entity_list: Vec<Entity> = vec![];
    {
        let mut selection_query = world.query::<&DebugUI>();
        for (entity, _) in selection_query.into_iter() {
            {
                entity_list.push(entity);
            }
        }
    }
    if entity_list.len() <= 0 {
        world.spawn((DebugUI,));
    } else {
        for entity in entity_list {
            world.despawn(entity).unwrap();
        }
    }
}
