use raylib::prelude::Vector2;

pub const SCREEN_WIDTH: i32 = 1280;
pub const SCREEN_HEIGHT: i32 = 720;

pub const TILESET_PATH: &str = "assets/tileset.png";
pub const UI_ATLAS_PATH: &str = "assets/ui.png";

pub const CAMERA_SPEED: f32 = 10.0;

pub const TILE_SIZE: f32 = 16.0;

pub const DEFAULT_IDLE_POINT: Vector2 = Vector2 { x: 192.0, y: 192.0 };
pub const DEFAULT_IDLE_POINT_ATLAS_TILE: Vector2 = Vector2 { x: 2.0, y: 6.0 };
pub const PLAYER_SPEED: f32 = 2.0;
