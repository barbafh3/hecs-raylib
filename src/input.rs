use hecs::{Entity, World};
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

use crate::collision::CollisionBox;
use crate::constants::{CAMERA_SPEED, PLAYER_SPEED};
use crate::entities::{ColorBox, Player};
use crate::ui::{DebugUI, toggle_mouse_selection};

// FUNCTIONS ------
pub fn read_inputs(world: &mut World, raylib_handle: &mut RaylibHandle, camera: &mut Camera2D) {
    camera.target = read_camera_input(&raylib_handle, camera.target);
    read_debug_toggle_input(world, &raylib_handle);
    read_player_input(world, &raylib_handle);

    if raylib_handle.is_key_released(KEY_F10) {
        toggle_mouse_selection(world);
    }
}

pub fn read_player_input(world: &mut World, raylib_handle: &RaylibHandle) {
    let mut query = world.query::<(&Player, &mut ColorBox, &mut CollisionBox)>();
    for (_, (_, ColorBox(color_rect), CollisionBox(collision_rect))) in query.into_iter() {
        if raylib_handle.is_key_down(KEY_D) {
            color_rect.x += PLAYER_SPEED;
            collision_rect.x += PLAYER_SPEED;
        } 
        if raylib_handle.is_key_down(KEY_A) {
            color_rect.x -= PLAYER_SPEED;
            collision_rect.x -= PLAYER_SPEED;
        } 
        if raylib_handle.is_key_down(KEY_W) {
            color_rect.y -= PLAYER_SPEED;
            collision_rect.y -= PLAYER_SPEED;
        } 
        if raylib_handle.is_key_down(KEY_S) {
            color_rect.y += PLAYER_SPEED;
            collision_rect.y += PLAYER_SPEED;
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
