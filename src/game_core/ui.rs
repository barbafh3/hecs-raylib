use hecs::{World, Entity};
use raylib::prelude::RaylibDraw;
use raylib::prelude::*;

use crate::{TILESET, UI_ATLAS};
use crate::game_core::constants::TILE_SIZE;
use crate::game_core::enums::ButtonState;

// TAGS ------
pub struct MouseSelection;
pub struct DebugUI;
pub struct UIAtlas;
pub struct CameraZoom(pub(crate) f32);

// STRUCTS ------
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
pub fn draw_ui(world: &mut World, draw_handle: &mut RaylibDrawHandle) {
    // draw_handle.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

    {
        let mut debug_query = world.query::<&DebugUI>();
        for (_, _) in debug_query.into_iter() {
            draw_handle.draw_fps(10, 10);
        }
    }

    draw_ui_buttons(world, draw_handle);
    draw_ui_toggle_buttons(world, draw_handle);
}

pub fn draw_ui_buttons(world: &mut World, draw_handle: &mut RaylibDrawHandle) {
    let mut zoom_query = world.query::<&CameraZoom>();
    let (_, CameraZoom(zoom)) = zoom_query.into_iter().nth(0).unwrap();
    let mut query = world.query::<&Button>();
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
