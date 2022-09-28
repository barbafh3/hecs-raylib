use hecs::{Entity, World};
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;

use crate::game_core::collision::{DrawCollisions, is_point_inside_box};
use crate::game_core::constants::{CAMERA_SPEED, TILE_SIZE};
use crate::game_core::enums::ButtonState;
use crate::game_core::ui::CameraZoom;
use crate::game_core::ui::ToggleButton;
use crate::game_core::ui::{DebugUI, toggle_mouse_selection, Button};

// FUNCTIONS ------
pub fn game_input(
    world: &mut World, 
    raylib_handle: &mut RaylibHandle, 
    camera: &mut Camera2D
) {
    camera.target = read_camera_input(raylib_handle, camera.target);
    check_debug_button_click(world, raylib_handle);

    if raylib_handle.is_key_released(KEY_F10) {
        toggle_mouse_selection(world);
    }
}

pub fn check_debug_button_click(world: &mut World, raylib_handle: &mut RaylibHandle) {
    let zoom: f32;
    {
        let mut zoom_query = world.query::<&CameraZoom>();
        let (_, CameraZoom(z)) = zoom_query.into_iter().nth(0).unwrap();
        zoom = z.clone();
    }

    let mut functions: Vec<fn(&mut World) -> ()> = vec![];
    let mut handle_functions: Vec<fn(&mut World, &mut RaylibHandle) -> ()> = vec![];
    
    let button_query = world.query_mut::<&mut Button>().without::<ToggleButton>();
    button_query.into_iter().for_each(|(_, button)| {
        let mouse_pos = raylib_handle.get_mouse_position();
        let button_box = Rectangle {
            x: button.position.x,
            y: button.position.y - (TILE_SIZE * zoom),
            width: TILE_SIZE * zoom,
            height: TILE_SIZE * zoom
        };
        if is_point_inside_box(&mouse_pos, &button_box) {
            button.state = ButtonState::Hovered;
            if raylib_handle.is_mouse_button_down(MOUSE_LEFT_BUTTON) {
                button.state = ButtonState::Pressed;
            }

            if raylib_handle.is_mouse_button_released(MOUSE_LEFT_BUTTON) {
                if button.action.is_some() {
                    functions.push(button.action.unwrap());
                }
                if button.handle_action.is_some() {
                    handle_functions.push(button.handle_action.unwrap());
                }
            }
        } else {
            button.state = ButtonState::Normal;
        }
    });

    let toggle_button_query = world.query_mut::<&mut ToggleButton>().without::<Button>();

    toggle_button_query.into_iter().for_each(|(_, button)| {
        let mouse_pos = raylib_handle.get_mouse_position();
        let button_box = Rectangle {
            x: button.position.x,
            y: button.position.y - (TILE_SIZE * zoom),
            width: TILE_SIZE * zoom,
            height: TILE_SIZE * zoom
        };
        if is_point_inside_box(&mouse_pos, &button_box) {
            if raylib_handle.is_mouse_button_released(MOUSE_LEFT_BUTTON) {
                if button.state == ButtonState::Toggled {
                    button.state = ButtonState::Hovered;
                } else {
                    button.state = ButtonState::Toggled;
                }
                if button.action.is_some() {
                    functions.push(button.action.unwrap());
                }
                if button.handle_action.is_some() {
                    handle_functions.push(button.handle_action.unwrap());
                }
            } else if button.state != ButtonState::Toggled {
                button.state = ButtonState::Hovered;
            }
        } else {
            if button.state != ButtonState::Toggled {
                button.state = ButtonState::Normal;
            }
        }
    });

    functions.iter().for_each(|function| function(world));
    handle_functions.iter().for_each(|function| function(world, raylib_handle));
}

pub fn read_camera_input(raylib_handle: &mut RaylibHandle, target: Vector2) -> Vector2 {
    let mut new_target = target;
    if raylib_handle.is_key_down(KEY_D) {
        new_target.x = target.x + CAMERA_SPEED;
    } 
    if raylib_handle.is_key_down(KEY_A) {
        new_target.x = target.x - CAMERA_SPEED;
    }
    if raylib_handle.is_key_down(KEY_W) {
        new_target.y = target.y - CAMERA_SPEED;
    } 
    if raylib_handle.is_key_down(KEY_S) {
        new_target.y = target.y + CAMERA_SPEED;
    }

    return new_target;
}

pub fn toggle_debug_text(world: &mut World) {
    let mut entity_list: Vec<Entity> = vec![];
    {
        let mut selection_query = world.query::<&DebugUI>();
        selection_query.into_iter().for_each(|(entity, _)| {
            entity_list.push(entity);
        });
    }
    if entity_list.len() <= 0 {
        world.spawn((DebugUI,));
    } else {
        for entity in entity_list {
            world.despawn(entity).unwrap();
        }
    }
}

pub fn toggle_draw_collisions(world: &mut World) {
    let mut entity_list: Vec<Entity> = vec![];
    {
        let mut selection_query = world.query::<&DrawCollisions>();
        selection_query.into_iter().for_each(|(entity, _)| {
            entity_list.push(entity);
        });
    }
    if entity_list.len() <= 0 {
        world.spawn((DrawCollisions,));
    } else {
        for entity in entity_list {
            world.despawn(entity).unwrap();
        }
    }
}
