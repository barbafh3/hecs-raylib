use std::collections::HashMap;

use hecs::World;
use raylib::prelude::*;
use rand::prelude::*;

use crate::{tilemap::Tileset, collision::CollisionBox, constants::{TILE_SIZE, DEFAULT_IDLE_POINT}, enums::GameResource};


// STRUCTS
pub struct Player;
pub struct Hauler;

pub struct IdleState {
    pub idle_point: Vector2,
    pub idle_timer: f32,
    pub timer_range: (f32, f32),
    pub radius: f32,
    pub target_position: Vector2
}

impl IdleState {
    pub fn default(idle_point: Vector2) -> IdleState {
        IdleState {
            idle_point,
            idle_timer: 0.0,
            timer_range: (3.0, 5.0),
            radius: 20.0,
            target_position: Vector2::zero()
        }
    }
}

pub struct ColorBox(pub(crate) Rectangle);

#[derive(Clone)]
pub struct Sprite {
    pub rect: Rectangle,
    pub position: Vector2,
}

impl Sprite {
    pub fn new(position: Vector2, atlas_tile: Vector2) -> Sprite {
        Sprite {
            position,
            rect: Rectangle {
                x: atlas_tile.x * TILE_SIZE,
                y: atlas_tile.y * TILE_SIZE,
                width: TILE_SIZE,
                height: TILE_SIZE
            }
        }
    }
}

pub struct Backpack {
    pub item: Option<GameResource>
}

pub struct HaulTask {
    pub origin: i32,
    pub destination: i32,
    pub resource: GameResource,
    pub amount: i32
}

pub fn spawn_hauler(world: &mut World, position: Vector2, atlas_tile: Vector2, opt_idle_point: Option<Vector2>) {
    let sprite = Sprite::new(
        position,
        atlas_tile,
    );
    let rect = Rectangle {
        x: position.x,
        y: position.y,
        width: TILE_SIZE,
        height: TILE_SIZE
    };
    let idle_point: Vector2;

    match opt_idle_point {
        Some(point) => idle_point = point,
        None => idle_point = DEFAULT_IDLE_POINT
    }

    world.spawn((
        Hauler, 
        IdleState::default(idle_point),
        sprite, 
        CollisionBox(rect)
    ));
}

pub fn draw_sprites(world: &mut World,draw_handle: &mut RaylibMode2D<RaylibDrawHandle>) {
    let mut query = world.query::<&Sprite>();
    let mut tileset_query = world.query::<(&Tileset, &Texture2D)>();
    for (_, (_, tileset)) in tileset_query.into_iter() {
        for (_, sprite) in query.into_iter() {
            draw_handle.draw_texture_rec(
                tileset,
                sprite.rect,
                sprite.position,
                Color::WHITE);
        }
    }
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

