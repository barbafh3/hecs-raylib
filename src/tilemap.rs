use hecs::World;
use rand::Rng;
use raylib::prelude::*;
use raylib::texture::Texture2D;

use crate::constants::TILE_SIZE;

// TAGS ------
pub struct Tileset;

// STRUCTS ------
#[derive(Clone)]
pub struct Tile {
    coord: Vector2,
    rect: Rectangle,
}

impl Tile {
    pub fn new(coord: Vector2, rect: Rectangle) -> Tile  {
        Tile {
            coord,
            rect
        }
    }

    pub fn get_coord(&self) -> Vector2 {
        self.coord
    }

    pub fn get_sprite_coord(&self) -> Rectangle {
        self.rect
    }
}

pub struct Tilemap {
    tiles: Vec<Tile>,
}

impl Tilemap {
    pub fn new(tiles: Vec<Tile>) -> Tilemap {
        Tilemap { tiles }
    }

    pub fn get_tiles(&self) -> Vec<Tile> {
        self.tiles.clone()
    }

    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }
}

// FUNCTIONS ------
pub fn generate_tilemap(world: &mut World, width: i32, height: i32) {
    let mut rng = rand::thread_rng();

    let mut tilemap = Tilemap::new(
        vec![],
    );

    for y in 0..height-1{
        for x in 0..width-1{
            let rng_x: i32 = rng.gen_range(0..4);
            let rect = Rectangle {
                x: (rng_x as f32) * TILE_SIZE,
                y: 0.0,
                width: TILE_SIZE,
                height: TILE_SIZE
            };
            let tile = Tile::new(Vector2 {x: x as f32, y: y as f32}, rect);
            tilemap.add_tile(tile);
        }
    }

    world.spawn((tilemap,));
}

pub fn draw_tilemap(world: &mut World, draw_handle: &mut RaylibMode2D<RaylibDrawHandle>) {
    let mut query = world.query::<&Tilemap>();
    let mut tileset_query = world.query::<(&Tileset, &Texture2D)>();
    for (_, tilemap) in query.into_iter() {
        tilemap.get_tiles().into_iter().for_each(|tile| {
            for (_, (_, tileset)) in tileset_query.into_iter() {
                draw_tile(draw_handle, &tileset, &tile);
            }
        });
    }
}

pub fn draw_tile(
    draw_handle: &mut RaylibMode2D<RaylibDrawHandle>,
    tileset: &Texture2D,
    tile: &Tile,
) {
    // TODO: Draw singleitem tile
    let rect = Rectangle {
        x: tile.rect.x,
        y: tile.rect.y,
        height: TILE_SIZE,
        width: TILE_SIZE,
    };
    draw_handle.draw_texture_rec(
        tileset,
        rect,
        tile.coord * TILE_SIZE,
        Color::WHITE
    );
}
