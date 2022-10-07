use hecs::{World, Entity};
use raylib::{prelude::*, text::{Font, measure_text_ex}};

use crate::{
    game::{
        constants::SCREEN_WIDTH_F,
        tasks::{OpenTasks, HaulTask}
    }, 
    engine::ui::datatypes::{Label, UIElement}
};

pub fn update_idle_haul_task_count(world: &mut World, draw_handle: &mut RaylibDrawHandle, font: &Font) {
    let font_size = 10.0;

    {
        let mut query = world.query::<&OpenTasks>();
        query.into_iter().for_each(|(_, open_tasks)| {
            let haul_count = open_tasks.haul_list.len();

            let text = format!("Haul tasks awaiting hauler: {:?}", haul_count);
            let half_text_size = measure_text_ex(font, &text, font_size, 1.0);
            draw_handle.draw_text_ex(
                font, 
                &text,
                Vector2 {
                    x: SCREEN_WIDTH_F - half_text_size.x - 5.0,
                    y: 12.0
                },
                font_size, 
                1.0,
                Color::BLACK
            );
        });
    }
}

pub fn update_active_haul_task_count(world: &mut World, font: &Font) {
    let mut labels_to_update: Vec<Entity> = vec![];

    let mut label_query = world.query::<(&Label, &UIElement)>();
    label_query.into_iter().for_each(|(entity, (_, element))| {
        if element.visible {
            labels_to_update.push(entity);
        }
    });

    labels_to_update.into_iter().for_each(|entity| {
        let mut _haul_count: usize = 0;
        let mut query = world.query::<&HaulTask>();
        _haul_count = query.into_iter().len();
        let text = format!("Haul tasks running: {:?}", _haul_count);
        let mut result = world.get_mut::<(Label, UIElement)>(entity).unwrap();
        let half_text_size = measure_text_ex(font, &text, result.0.font_size, result.0.spacing);
        result.0.text = text;
        result.1.offset.x = SCREEN_WIDTH_F - half_text_size.x - 5.0;
    });
}
