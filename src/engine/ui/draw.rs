use hecs::{Entity, World};
use raylib::{prelude::*, text::Font};

use crate::engine::{enums::ButtonState, TILESET, UI_ATLAS};

use super::{
    datatypes::{CameraZoom, Label, UIElement},
    Button, DebugUI, MouseSelection, ToggleButton,
};

pub fn draw_ui(world: &mut World, draw_handle: &mut RaylibDrawHandle, font: &Font, tile_size: f32) {
    {
        let mut debug_query = world.query::<&DebugUI>();
        debug_query.into_iter().for_each(|_| {
            draw_handle.draw_fps(10, 10);
        });
    }

    for layer in 0..101 {
        draw_ui_buttons(world, draw_handle, layer, tile_size);
        draw_ui_toggle_buttons(world, draw_handle, layer, tile_size);
        draw_labels(world, draw_handle, font, layer);
    }
}

pub fn draw_ui_buttons(
    world: &mut World,
    draw_handle: &mut RaylibDrawHandle,
    layer: i32,
    tile_size: f32,
) {
    let mut zoom_query = world.query::<&CameraZoom>();
    let (_, CameraZoom(zoom)): (Entity, &CameraZoom) = zoom_query.into_iter().nth(0).unwrap();
    let mut query = world.query::<(&Button, &UIElement)>();
    query.into_iter().for_each(|(_, (button, element))| {
        if element.layer == layer && element.visible {
            let mut src = button.rect.clone();
            println!("Button atlas coord: {:?}", src);
            match button.state {
                ButtonState::Hovered => {
                    src.x = src.x + 16.0;
                }
                ButtonState::Pressed => {
                    src.x = src.x + 32.0;
                }
                _ => {}
            }
            let dest = Rectangle {
                x: element.position.x,
                y: element.position.y - (tile_size * zoom),
                width: tile_size * zoom,
                height: tile_size * zoom,
            };
            draw_handle.draw_texture_pro(
                UI_ATLAS.get().unwrap(),
                src,
                dest,
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }
    });
}

pub fn draw_ui_toggle_buttons(
    world: &mut World,
    draw_handle: &mut RaylibDrawHandle,
    layer: i32,
    tile_size: f32,
) {
    let mut zoom_query = world.query::<&CameraZoom>();
    let (_, CameraZoom(zoom)) = zoom_query.into_iter().nth(0).unwrap();
    let mut query = world.query::<(&ToggleButton, &UIElement)>();
    query.into_iter().for_each(|(_, (button, element))| {
        if element.layer == layer && element.visible {
            let mut src = button.rect.clone();
            match button.state {
                ButtonState::Hovered => {
                    src.x = src.x + 16.0;
                }
                ButtonState::Pressed => {
                    src.x = src.x + 32.0;
                }
                ButtonState::Toggled => {
                    src.x = src.x + 32.0;
                }
                _ => {}
            }
            let dest = Rectangle {
                x: element.position.x,
                y: element.position.y - (tile_size * zoom),
                width: tile_size * zoom,
                height: tile_size * zoom,
            };
            draw_handle.draw_texture_pro(
                UI_ATLAS.get().unwrap(),
                src,
                dest,
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }
    });
}

pub fn draw_mouse_selection(
    world: &mut World,
    draw_handle: &mut RaylibMode2D<RaylibDrawHandle>,
    camera: &Camera2D,
    tile_size: f32,
) {
    let mouse_pos = draw_handle.get_screen_to_world2D(draw_handle.get_mouse_position(), camera);
    let mut selection_query = world.query::<&MouseSelection>();
    selection_query.into_iter().for_each(|(_, _)| {
        let mut current_tile_x = (mouse_pos.x / tile_size) as i32;
        let mut current_tile_y = (mouse_pos.y / tile_size) as i32;

        if mouse_pos.x < 0.0 {
            current_tile_x -= 1;
        }
        if mouse_pos.y < 0.0 {
            current_tile_y -= 1;
        }

        let tile_position = Vector2 {
            x: (current_tile_x) as f32 * tile_size,
            y: (current_tile_y) as f32 * tile_size,
        };

        let rect = Rectangle {
            x: 96.0,
            y: 672.0,
            height: tile_size,
            width: tile_size,
        };
        draw_handle.draw_texture_rec(TILESET.get().unwrap(), rect, tile_position, Color::WHITE);
    });
}

pub fn draw_labels(world: &mut World, draw_handle: &mut RaylibDrawHandle, font: &Font, layer: i32) {
    let mut query = world.query::<(&Label, &UIElement)>();
    query.into_iter().for_each(|(_, (label, element))| {
        if element.layer == layer && element.visible {
            draw_handle.draw_text_ex(
                font,
                &label.text,
                element.position + element.offset,
                label.font_size,
                label.spacing,
                label.color,
            );
        }
    });
}
