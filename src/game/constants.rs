use raylib::prelude::{Vector2, Rectangle};

pub const SCREEN_WIDTH: i32 = 1280;
pub const SCREEN_WIDTH_F: f32 = 1280.0;
pub const SCREEN_HEIGHT: i32 = 720;
pub const SCREEN_HEIGHT_F: f32 = 720.0;

pub const TILESET_PATH: &str = "assets/tileset.png";
pub const UI_ATLAS_PATH: &str = "assets/ui.png";
pub const FONT_PATH: &str = "assets/prstartk.ttf";

pub const CAMERA_SPEED: f32 = 10.0;

pub const TILE_SIZE: f32 = 16.0;
pub const CHUNK_TILE_SIZE: i32 = 16;
/// The size in pixels of any given chunk
pub const CHUNK_RAW_SIZE: i32 = CHUNK_TILE_SIZE * TILE_SIZE as i32;

pub const DEFAULT_IDLE_POINT: Vector2 = Vector2 { x: 192.0, y: 192.0 };
pub const DEFAULT_IDLE_POINT_ATLAS_TILE: Vector2 = Vector2 { x: 2.0, y: 6.0 };
pub const PLAYER_SPEED: f32 = 2.0;

pub const HAULER_CAPACITY: i32 = 10;

pub const CONSTRUCTION_RECT: Rectangle = Rectangle { 
    x: 0.0 * TILE_SIZE, 
    y: 5.0 * TILE_SIZE, 
    width: TILE_SIZE, 
    height: TILE_SIZE 
};
