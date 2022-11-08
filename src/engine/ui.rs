pub mod draw;
pub mod datatypes;
pub mod step;

use hecs::{World, Entity};
use raylib::prelude::*;

use self::datatypes::{MouseSelection, DebugUI, Label, UIElement};
use self::datatypes::ToggleButton;
use self::datatypes::Button;

// FUNCTIONS ------
pub fn spawn_button(
    world: &mut World, 
    position: Vector2, 
    offset: Vector2, 
    layer: i32,
    atlas_tile: Vector2, 
    tile_size: f32,
    action: Option<fn(&mut World) -> Result<(), String>>,
    handle_action: Option<fn(&mut World, &mut RaylibHandle) -> Result<(), String>>,
) -> Entity {
    let ui_element = UIElement {
        position,
        offset,
        layer,
        visible: true
    };
    let button = Button::new(
        atlas_tile,
        tile_size,
        action,
        handle_action,
    );

    return world.spawn((button, ui_element));
}

pub fn spawn_toggle_button(
    world: &mut World, 
    position: Vector2, 
    offset: Vector2, 
    layer: i32,
    atlas_tile: Vector2, 
    tile_size: f32,
    basic_action: Option<fn(&mut World) -> Result<(), String>>,
    handle_action: Option<fn(&mut World, &mut RaylibHandle) -> Result<(), String>>,
) -> Entity {
    let ui_element = UIElement {
        position,
        offset,
        layer,
        visible: true
    };
    let button = ToggleButton::new(
        atlas_tile,
        tile_size,
        basic_action,
        handle_action,
    );

    return world.spawn((button, ui_element));
}

pub fn spawn_label(
    world: &mut World, 
    position: Vector2, 
    offset: Vector2, 
    layer: i32,
    font_size: f32,
    spacing: f32,
    color: Color
) -> Entity {
    let ui_element = UIElement {
        position,
        offset,
        layer,
        visible: true
    };
    let label = Label {
        text: String::new(),
        font_size,
        spacing,
        color
    };

    return world.spawn((label, ui_element));
}

pub fn toggle_mouse_selection(world: &mut World) -> Result<(), String> {
    let mut entity_list: Vec<Entity> = vec![];
    {
        let mut selection_query = world.query::<&MouseSelection>();
        for (entity, _) in selection_query.into_iter() {
            {
                entity_list.push(entity);
            }
        }
    }
    if entity_list.len() <= 0 {
        world.spawn((MouseSelection,));
    } else {
        for entity in entity_list {
            world.despawn(entity).map_err(|_| "No such entity")?;
        }
    }

    Ok(())
}

pub fn toggle_debug_ui(world: &mut World) {
    let mut entity_list: Vec<Entity> = vec![];
    {
        let mut selection_query = world.query::<&DebugUI>();
        for (entity, _) in selection_query.into_iter() {
            {
                entity_list.push(entity);
            }
        }
    }
    if entity_list.len() <= 0 {
        world.spawn((DebugUI,));
    } else {
        for entity in entity_list {
            world.despawn(entity).unwrap();
        }
    }
}
