use hecs::World;
use rand::Rng;
use raylib::prelude::*;
use raylib::texture::Texture2D;

use crate::engine::{TILESET, collision::are_boxes_colliding};

use super::constants::{TILE_SIZE, CHUNK_TILE_SIZE, CHUNK_RAW_SIZE};

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

#[derive(Default, Clone)]
pub struct Chunk {
    pub rect: Rectangle,
    pub tiles: Vec<Tile>,
    pub visible: bool
}

pub struct Tilemap {
    pub chunks: Vec<Chunk>,
}

impl Tilemap {
    pub fn new(chunks: Vec<Chunk>) -> Tilemap {
        Tilemap { chunks }
    }
}

// FUNCTIONS ------
pub fn generate_chunk(origin_x: i32, origin_y: i32) -> Chunk {
    let mut rng = rand::thread_rng();
    let mut chunk = Chunk::default();

    let world_start_x: i32 = origin_x * CHUNK_TILE_SIZE;
    let world_start_y: i32 = origin_y * CHUNK_TILE_SIZE;

    let world_end_x: i32 = world_start_x + CHUNK_TILE_SIZE;
    let world_end_y: i32 = world_start_y + CHUNK_TILE_SIZE;

    chunk.rect = Rectangle {
        x: world_start_x as f32,
        y: world_start_y as f32,
        width: CHUNK_RAW_SIZE as f32,
        height: CHUNK_RAW_SIZE as f32
    };

    for y in world_start_y..world_end_y {
        for x in world_start_x..world_end_x {
            let rng_x: i32 = rng.gen_range(0..4);
            let rect = Rectangle {
                x: (rng_x as f32) * TILE_SIZE,
                y: 0.0,
                width: TILE_SIZE,
                height: TILE_SIZE
            };
            let tile = Tile::new(Vector2 {x: x as f32, y: y as f32}, rect);
            chunk.tiles.push(tile);
        }
    }

    return chunk;
}

pub fn generate_tilemap(world: &mut World, width: i32, height: i32) {
    let mut tilemap = Tilemap::new(vec![]);

    let chunk_x: i32 = width / CHUNK_TILE_SIZE;
    let chunk_y: i32 = height / CHUNK_TILE_SIZE;

    for y in 0..chunk_y {
        for x in 0..chunk_x {
            let chunk = generate_chunk(x, y);
            tilemap.chunks.push(chunk);
        }
    }

    world.spawn((tilemap,));
}

pub fn draw_tilemap(world: &mut World, draw_handle: &mut RaylibMode2D<RaylibDrawHandle>) {
    let mut query = world.query::<&Tilemap>();
    let result = query.into_iter().nth(0);

    if let Some((_, tilemap)) = result {
        tilemap.chunks.iter().for_each(|chunk| {
            if chunk.visible {
                chunk.tiles.iter().for_each(|tile| {
                    draw_tile(draw_handle, &TILESET.get().unwrap(), &tile);
                });
            }
        });
    }
}

pub fn check_visible_tilemap_chunks(world: &mut World, draw_handle: &mut RaylibMode2D<RaylibDrawHandle>, camera: &Camera2D) {
    let query = world.query_mut::<&mut Tilemap>();
    let result = query.into_iter().nth(0);

    if let Some((_, tilemap)) = result {
        let cw = (draw_handle.get_screen_width() as f32) / camera.zoom;
        let ch = (draw_handle.get_screen_height() as f32) / camera.zoom;
        let camera_rect = Rectangle {
            x: camera.target.x,
            y: camera.target.y,
            width: cw,
            height: ch
        };

        tilemap.chunks.iter_mut().for_each(|mut chunk| {
            let chunk_rect = Rectangle {
                x: chunk.rect.x * TILE_SIZE,
                y: chunk.rect.y * TILE_SIZE,
                width: CHUNK_RAW_SIZE as f32,
                height: CHUNK_RAW_SIZE as f32,
            };

            if are_boxes_colliding(&camera_rect, &chunk_rect) {
                chunk.visible = true;
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
