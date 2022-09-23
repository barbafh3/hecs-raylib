use hecs::{Entity, World};
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;

use crate::collision::{DrawCollisions, is_point_inside_box};
use crate::constants::{CAMERA_SPEED, TILE_SIZE};
use crate::entities::Hauler;
use crate::enums::{GameResource, ButtonState};
use crate::tasks::HaulTask;
use crate::ui::CameraZoom;
use crate::ui::{DebugUI, toggle_mouse_selection, Button};

// FUNCTIONS ------
pub fn read_inputs(
    world: &mut World, 
    draw_handle: &mut RaylibDrawHandle, 
    camera: &mut Camera2D
) {
    camera.target = read_camera_input(draw_handle, camera.target);
    give_haul_task(world, draw_handle);
    check_button_click(world, draw_handle);

    if draw_handle.is_key_released(KEY_F10) {
        toggle_mouse_selection(world);
    }
}

pub fn check_button_click(world: &mut World, draw_handle: &mut RaylibDrawHandle) {
    let zoom: f32;
    {
        let mut zoom_query = world.query::<&CameraZoom>();
        let (_, CameraZoom(z)) = zoom_query.into_iter().nth(0).unwrap();
        zoom = z.clone();
    }

    let mut functions: Vec<fn(&mut World) -> ()> = vec![];
    let mut handle_functions: Vec<fn(&mut World, &mut RaylibDrawHandle) -> ()> = vec![];
    
    let button_query = world.query_mut::<&mut Button>();
    for (_, button) in button_query.into_iter() {
        let mouse_pos = draw_handle.get_mouse_position();
        let button_box = Rectangle {
            x: button.position.x,
            y: button.position.y - (TILE_SIZE * zoom),
            width: TILE_SIZE * zoom,
            height: TILE_SIZE * zoom
        };
        if is_point_inside_box(&mouse_pos, &button_box) {
            button.state = ButtonState::Hover;
            if draw_handle.is_mouse_button_down(MOUSE_LEFT_BUTTON) {
                button.state = ButtonState::Pressed;
            }

            if draw_handle.is_mouse_button_released(MOUSE_LEFT_BUTTON) {
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
    }

    functions.iter().for_each(|function| function(world));
    handle_functions.iter().for_each(|function| function(world, draw_handle));
}

pub fn give_haul_task(world: &mut World, draw_handle: &mut RaylibDrawHandle) {
    if draw_handle.is_key_released(KEY_F11) {
        let mut selected_hauler: Option<Entity> = None;

        {
            for (ety, _) in &mut world.query::<&Hauler>().without::<HaulTask>() {
                println!("Hauler: {:?}", ety);
                if selected_hauler.is_none() {
                    selected_hauler = Some(ety);
                }
            }
        }


        if let Some(hauler) = selected_hauler {
            let haul_task = HaulTask {
                origin: 0,
                destination: 0,
                resource: GameResource::Wood,
                resource_amount: 100,
                delivered_amount: 0
            };
            world.insert(hauler, (haul_task,)).unwrap();
        }
    }
}

pub fn read_camera_input(draw_handle: &mut RaylibDrawHandle, target: Vector2) -> Vector2 {
    let mut new_target = target;
    if draw_handle.is_key_down(KEY_RIGHT) {
        new_target.x = target.x + CAMERA_SPEED;
    } 
    if draw_handle.is_key_down(KEY_LEFT) {
        new_target.x = target.x - CAMERA_SPEED;
    }
    if draw_handle.is_key_down(KEY_UP) {
        new_target.y = target.y - CAMERA_SPEED;
    } 
    if draw_handle.is_key_down(KEY_DOWN) {
        new_target.y = target.y + CAMERA_SPEED;
    }

    return new_target;
}

pub fn toggle_debug_text(world: &mut World) {
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

pub fn toggle_draw_collisions(world: &mut World) {
    let mut entity_list: Vec<Entity> = vec![];
    {
        let mut selection_query = world.query::<&DrawCollisions>();
        for (entity, _) in selection_query.into_iter() {
            {
                entity_list.push(entity);
            }
        }
    }
    if entity_list.len() <= 0 {
        world.spawn((DrawCollisions,));
    } else {
        for entity in entity_list {
            world.despawn(entity).unwrap();
        }
    }
}
