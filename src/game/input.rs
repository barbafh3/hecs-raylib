use hecs::{Entity, World};
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
use raylib::prelude::*;

use crate::engine::{
    collision::{is_point_inside_box, DrawCollisions},
    enums::ButtonState,
    ui::{
        datatypes::{Button, CameraZoom, DebugUI, ToggleButton, UIElement},
        toggle_mouse_selection,
    },
};

use super::buildings::step::place_hovering_building;
use super::buildings::step::update_construction_hover;
use super::constants::{CAMERA_SPEED, TILE_SIZE};

// FUNCTIONS ------
pub fn handle_input(world: &mut World, raylib_handle: &mut RaylibHandle, camera: &mut Camera2D) -> Result<(), String> {
    camera.target = read_camera_input(raylib_handle, camera.target);
    place_hovering_building(world, raylib_handle)?;
    check_debug_button_click(world, raylib_handle)?;
    update_construction_hover(world, raylib_handle, camera);

    if raylib_handle.is_key_released(KEY_F10) {
        toggle_mouse_selection(world)?;
    }

    Ok(())
}

pub fn check_debug_button_click(world: &mut World, raylib_handle: &mut RaylibHandle) -> Result<(), String> {
    let zoom: f32;
    {
        let mut zoom_query = world.query::<&CameraZoom>();
        let (_, CameraZoom(z)) = zoom_query.into_iter().nth(0).ok_or("Camera zoom missing")?;
        zoom = z.clone();
    }

    let mut functions: Vec<fn(&mut World) -> Result<(), String>> = vec![];
    let mut handle_functions: Vec<fn(&mut World, &mut RaylibHandle) -> Result<(), String>> = vec![];

    let button_query = world
        .query_mut::<(&mut Button, &UIElement)>()
        .without::<ToggleButton>();
    for (_, (button, element)) in button_query.into_iter() {
        let mouse_pos = raylib_handle.get_mouse_position();
        let button_box = Rectangle {
            x: element.position.x,
            y: element.position.y - (TILE_SIZE * zoom),
            width: TILE_SIZE * zoom,
            height: TILE_SIZE * zoom,
        };
        if is_point_inside_box(&mouse_pos, &button_box) {
            button.state = ButtonState::Hovered;
            if raylib_handle.is_mouse_button_down(MOUSE_LEFT_BUTTON) {
                button.state = ButtonState::Pressed;
            }

            if raylib_handle.is_mouse_button_released(MOUSE_LEFT_BUTTON) {
                if button.action.is_some() {
                    functions.push(button.action.ok_or("No button action")?);
                }
                if button.handle_action.is_some() {
                    handle_functions.push(button.handle_action.ok_or("No button handle action")?);
                }
            }
        } else {
            button.state = ButtonState::Normal;
        }
    }

    let toggle_button_query = world
        .query_mut::<(&mut ToggleButton, &UIElement)>()
        .without::<Button>();

    for (_, (button, element)) in toggle_button_query.into_iter() {
        let mouse_pos = raylib_handle.get_mouse_position();
        let button_box = Rectangle {
            x: element.position.x,
            y: element.position.y - (TILE_SIZE * zoom),
            width: TILE_SIZE * zoom,
            height: TILE_SIZE * zoom,
        };
        if is_point_inside_box(&mouse_pos, &button_box) {
            if raylib_handle.is_mouse_button_released(MOUSE_LEFT_BUTTON) {
                if button.state == ButtonState::Toggled {
                    button.state = ButtonState::Hovered;
                } else {
                    button.state = ButtonState::Toggled;
                }
                if button.action.is_some() {
                    functions.push(button.action.ok_or("No button action")?);
                }
                if button.handle_action.is_some() {
                    handle_functions.push(button.handle_action.ok_or("No button handle action")?);
                }
            } else if button.state != ButtonState::Toggled {
                button.state = ButtonState::Hovered;
            }
        } else {
            if button.state != ButtonState::Toggled {
                button.state = ButtonState::Normal;
            }
        }
    }

    for function in functions.iter() {
        function(world)?;
    }

    for handle_function in handle_functions.iter() {
        handle_function(world, raylib_handle)?;
    }

    Ok(())
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

pub fn toggle_debug_text(world: &mut World) -> Result<(), String> {
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
            world.despawn(entity).map_err(|_| "No such entity")?;
        }
    }

    Ok(())
}

pub fn toggle_draw_collisions(world: &mut World) -> Result<(), String> {
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
            world.despawn(entity).map_err(|_| "No such entity")?;
        }
    }

    Ok(())
}
