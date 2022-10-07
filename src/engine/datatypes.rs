use raylib::prelude::*;

#[derive(Clone)]
pub struct Sprite {
    pub rect: Rectangle,
    pub position: Vector2,
}

impl Sprite {
    pub fn new(position: Vector2, atlas_tile: Vector2, tile_size: f32) -> Sprite {
        Sprite {
            position,
            rect: Rectangle {
                x: atlas_tile.x * tile_size,
                y: atlas_tile.y * tile_size,
                width: tile_size,
                height: tile_size
            }
        }
    }
}
