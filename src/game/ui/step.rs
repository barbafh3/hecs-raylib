use std::collections::HashMap;

use hecs::{Entity, World};
use raylib::{
    prelude::Vector2,
    text::{measure_text_ex, Font},
};

use crate::{
    engine::{
        enums::GameResource,
        ui::datatypes::{Label, UIElement},
    },
    game::{
        buildings::datatypes::StorageSpace,
        tasks::{HaulTask, OpenTasks},
    },
};

use super::datatypes::{
    ActiveTaskCountLabel, GlobalStorage, GlobalStorageLabel, IdleTaskCountLabel,
};

pub fn update_ui(world: &mut World, font: &Font) {
    update_active_haul_task_count(world, font);
    update_idle_haul_task_count(world, font);
    update_global_storage(world, font);
}

// FUNCTIONS
pub fn update_global_storage(world: &mut World, font: &Font) {
    let mut current_storage: HashMap<GameResource, i32> = HashMap::new();
    {
        let mut query = world.query::<&StorageSpace>();
        query.into_iter().for_each(|(_, storage)| {
            storage
                .item_list
                .clone()
                .into_iter()
                .for_each(|(resource, amount)| {
                    if current_storage.contains_key(&resource) {
                        *current_storage.get_mut(&resource).unwrap() += amount;
                    } else {
                        current_storage.insert(resource, amount);
                    }
                });
        });
    }

    {
        let m_global_storage = world.query_mut::<&mut GlobalStorage>().into_iter().nth(0);
        if let Some((_, mut global_storage)) = m_global_storage {
            global_storage.resource_list = current_storage.clone();
        }
    }

    let mut labels_to_update: Vec<Entity> = vec![];

    {
        let mut label_query = world
            .query::<(&Label, &UIElement)>()
            .with::<GlobalStorageLabel>();
        label_query.into_iter().for_each(|(entity, (_, element))| {
            if element.visible {
                labels_to_update.push(entity);
            }
        });
    }

    labels_to_update.into_iter().for_each(|entity| {
        let mut _half_text_size: Vector2 = Vector2::zero();
        let text = format!("{:?}", current_storage);
        {
            let mut label = world.get_mut::<Label>(entity).unwrap();
            _half_text_size =
                measure_text_ex(font, &text, label.font_size.clone(), label.spacing.clone()) / 2.0;
            label.text = text;
        }
        {
            let mut element = world.get_mut::<UIElement>(entity).unwrap();
            element.offset.x = -_half_text_size.x;
        }
    });
}

pub fn update_idle_haul_task_count(world: &mut World, font: &Font) {
    let mut labels_to_update: Vec<Entity> = vec![];

    {
        let mut label_query = world
            .query::<(&Label, &UIElement)>()
            .with::<IdleTaskCountLabel>();
        label_query.into_iter().for_each(|(entity, (_, element))| {
            if element.visible {
                labels_to_update.push(entity);
            }
        });
    }

    labels_to_update.into_iter().for_each(|entity| {
        let mut _haul_count: usize = 0;
        {
            let mut query = world.query::<&OpenTasks>();
            query.into_iter().for_each(|(_, open_tasks)| {
                _haul_count = open_tasks.haul_list.len();
            });
        }
        let mut _half_text_size: Vector2 = Vector2::zero();
        let text = format!("Haul tasks awaiting hauler: {:?}", _haul_count);
        {
            let mut label = world.get_mut::<Label>(entity).unwrap();
            _half_text_size =
                measure_text_ex(font, &text, label.font_size.clone(), label.spacing.clone());
            label.text = text;
        }
        {
            let mut element = world.get_mut::<UIElement>(entity).unwrap();
            element.offset.x = -_half_text_size.x;
        }
    });
}

pub fn update_active_haul_task_count(world: &mut World, font: &Font) {
    let mut labels_to_update: Vec<Entity> = vec![];

    {
        let mut label_query = world
            .query::<(&Label, &UIElement)>()
            .with::<ActiveTaskCountLabel>();
        label_query.into_iter().for_each(|(entity, (_, element))| {
            if element.visible {
                labels_to_update.push(entity);
            }
        });
    }

    labels_to_update.into_iter().for_each(|entity| {
        let mut _haul_count: usize = 0;
        {
            let mut query = world.query::<&HaulTask>();
            _haul_count = query.into_iter().len();
        }
        let mut _half_text_size: Vector2 = Vector2::zero();
        let text = format!("Haul tasks running: {:?}", _haul_count);
        {
            let mut label = world.get_mut::<Label>(entity).unwrap();
            _half_text_size =
                measure_text_ex(font, &text, label.font_size.clone(), label.spacing.clone());
            label.text = text;
        }
        {
            let mut element = world.get_mut::<UIElement>(entity).unwrap();
            element.offset.x = -_half_text_size.x;
        }
    });
}
