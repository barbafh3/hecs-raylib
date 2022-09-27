use raylib::prelude::*;

use crate::game_core::enums::*;

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

pub struct LoadingState {
    pub target_position: Vector2
}

pub struct CarryingState {
    pub target_position: Vector2
}

pub struct Backpack {
    pub item: Option<GameResource>
}

