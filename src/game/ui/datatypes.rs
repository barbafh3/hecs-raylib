use std::collections::HashMap;

use hecs::Entity;

use crate::engine::enums::GameResource;

// TRAITS
pub trait ComponentTag {}

// TAGS
pub struct ActiveTaskCountLabel;
pub struct IdleTaskCountLabel;
pub struct GlobalStorageLabel;

impl ComponentTag for ActiveTaskCountLabel {}

// STRUCTS
pub struct SelectedHauler {
    pub hauler: Entity
}

pub struct GlobalStorage {
    pub resource_list: HashMap<GameResource, i32>
}
