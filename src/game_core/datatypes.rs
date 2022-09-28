use raylib::prelude::*;

use super::constants::TILE_SIZE;

pub struct Fonts {
    pub base_font: Font
}

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

