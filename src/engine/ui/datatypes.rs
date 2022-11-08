use hecs::World;
use raylib::prelude::*;

use crate::engine::enums::ButtonState;

// TAGS ------
pub struct MouseSelection;
pub struct DebugUI;
pub struct UIAtlas;
pub struct CameraZoom(pub(crate) f32);

// STRUCTS ------
pub struct Fonts {
    pub base_font: Font
}

pub struct UIElement {
    pub position: Vector2,
    pub offset: Vector2,
    pub layer: i32,
    pub visible: bool
}

pub struct Label {
    pub text: String,
    pub font_size: f32,
    pub spacing: f32,
    pub color: Color
}

pub struct Button {
    pub rect: Rectangle,
    pub state: ButtonState,
    pub action: Option<fn(&mut World) -> Result<(), String>>,
    pub handle_action: Option<fn(&mut World, &mut RaylibHandle) -> Result<(), String>>,
}

impl Button {
    pub fn new(
        atlas_tile: Vector2, 
        tile_size: f32,
        action: Option<fn(&mut World) -> Result<(), String>>,
        handle_action: Option<fn(&mut World, &mut RaylibHandle) -> Result<(), String>>,
    ) -> Button {
        Button {
            state: ButtonState::Normal,
            action,
            handle_action,
            rect: Rectangle {
                x: atlas_tile.x * tile_size,
                y: atlas_tile.y * tile_size,
                width: tile_size,
                height: tile_size
            }
        }
    }
}

pub struct ToggleButton {
    pub rect: Rectangle,
    pub state: ButtonState,
    pub action: Option<fn(&mut World) -> Result<(), String>>,
    pub handle_action: Option<fn(&mut World, &mut RaylibHandle) -> Result<(), String>>,
}

impl ToggleButton {
    pub fn new(
        atlas_tile: Vector2, 
        tile_size: f32,
        action: Option<fn(&mut World) -> Result<(), String>>,
        handle_action: Option<fn(&mut World, &mut RaylibHandle) -> Result<(), String>>,
    ) -> ToggleButton {
        ToggleButton {
            state: ButtonState::Normal,
            action,
            handle_action,
            rect: Rectangle {
                x: atlas_tile.x * tile_size,
                y: atlas_tile.y * tile_size,
                width: tile_size,
                height: tile_size
            }
        }
    }
}
