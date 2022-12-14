pub mod collision;
pub mod datatypes;
pub mod draw;
pub mod enums;
pub mod startup;
pub mod ui;
pub mod utils;

use once_cell::sync::OnceCell;
use raylib::texture::Texture2D;

// GLOBAL TEXTURES
pub static TILESET: OnceCell<Texture2D> = OnceCell::new();
pub static UI_ATLAS: OnceCell<Texture2D> = OnceCell::new();
