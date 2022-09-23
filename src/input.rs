use hecs::{Entity, World};
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

use crate::collision::DrawCollisions;
use crate::constants::CAMERA_SPEED;
use crate::entities::Hauler;
use crate::enums::GameResource;
use crate::tasks::HaulTask;
use crate::ui::{DebugUI, toggle_mouse_selection};

// FUNCTIONS ------
pub fn read_inputs(world: &mut World, raylib_handle: &mut RaylibHandle, camera: &mut Camera2D) {
    camera.target = read_camera_input(&raylib_handle, camera.target);
    read_debug_toggle_input(world, &raylib_handle);
    read_draw_collisions_toggle_input(world, raylib_handle);
    give_haul_task(world, raylib_handle);

    if raylib_handle.is_key_released(KEY_F10) {
        toggle_mouse_selection(world);
    }
}

pub fn give_haul_task(world: &mut World, raylib_handle: &mut RaylibHandle) {
    if raylib_handle.is_key_released(KEY_F11) {
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

pub fn read_camera_input(raylib_handle: &RaylibHandle, target: Vector2) -> Vector2 {
    let mut new_target = target;
    if raylib_handle.is_key_down(KEY_RIGHT) {
        new_target.x = target.x + CAMERA_SPEED;
    } 
    if raylib_handle.is_key_down(KEY_LEFT) {
        new_target.x = target.x - CAMERA_SPEED;
    }
    if raylib_handle.is_key_down(KEY_UP) {
        new_target.y = target.y - CAMERA_SPEED;
    } 
    if raylib_handle.is_key_down(KEY_DOWN) {
        new_target.y = target.y + CAMERA_SPEED;
    }

    return new_target;
}

pub fn read_debug_toggle_input(world: &mut World, raylib_handle: &RaylibHandle) {
    if raylib_handle.is_key_released(KEY_F9) {
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
}

pub fn read_draw_collisions_toggle_input(world: &mut World, raylib_handle: &RaylibHandle) {
    if raylib_handle.is_key_released(KEY_F8) {
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
}
