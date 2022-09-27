use hecs::World;
use rand::Rng;
use raylib::{RaylibHandle, prelude::*};

use crate::game_core::{datatypes::Sprite, collision::CollisionBox};

use super::datatypes::IdleState;

pub fn update_villagers(world: &mut World,raylib_handle: &mut RaylibHandle) {
    update_collision_box_position(world);
    update_idle_state(world, raylib_handle);
}

pub fn update_idle_state(world: &mut World, raylib_handle: &mut RaylibHandle) {
    let delta = raylib_handle.get_frame_time();

    let idle_query = world.query_mut::<(&mut IdleState, &mut Sprite)>();
    for (_, (idle_state, sprite)) in idle_query.into_iter() {
        idle_state_tick(idle_state, delta);
        if idle_state.idle_timer <= 0.0 {
            get_new_target(idle_state);
        }
        if (idle_state.target_position - sprite.position).length() > 1.0 {
            idle_move(idle_state, sprite, delta);
        }
    }
}

pub fn idle_state_tick(idle_state: &mut IdleState, delta: f32) {
    if idle_state.idle_timer > 0.0 {
        idle_state.idle_timer -= delta;
    }
}

pub fn get_new_target(idle_state: &mut IdleState) {
        let mut rng = rand::thread_rng();
        let rand_x: f32 = rng.gen_range(
            (idle_state.idle_point.x - idle_state.radius)..(idle_state.idle_point.x + idle_state.radius)
        );
        let rand_y: f32 = rng.gen_range((idle_state.idle_point.y - idle_state.radius)..(idle_state.idle_point.y + idle_state.radius));
        idle_state.target_position = Vector2 { x: rand_x, y: rand_y };
        idle_state.idle_timer = rng.gen_range(idle_state.timer_range.0..idle_state.timer_range.1);
}

pub fn idle_move(idle_state: &mut IdleState, sprite: &mut Sprite, delta: f32) {
    let vector = (idle_state.target_position - sprite.position).normalized();
    sprite.position += vector * 50.0 * delta;
}

pub fn update_collision_box_position(world: &mut World) {
    let query = world.query_mut::<(&Sprite, &mut CollisionBox)>();
    for (_, (sprite, collision_box)) in query.into_iter() {
        collision_box.rect.x = sprite.position.x;
        collision_box.rect.y = sprite.position.y;
    }
}

