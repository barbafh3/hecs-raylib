use std::collections::HashMap;

use hecs::{World, Entity};
use raylib::prelude::*;

// TAGS ------
pub struct DrawCollisions;

// STRUCTS ------
#[derive(Default)]
pub struct BodyCollision {
  pub colliding: bool,
  pub other_body: Option<Entity>
}

pub struct TriggerCollision {
  pub colliding: bool,
  pub other_trigger: Option<Entity>,
}

impl TriggerCollision {
  pub fn new() -> TriggerCollision {
    TriggerCollision { colliding: false, other_trigger: None }
  }
}

pub struct CollisionBox {
  pub rect: Rectangle
}


// FUNCTIONS ------
pub fn is_point_inside_box(point: &Vector2, col_box: &Rectangle) -> bool {
  return point.x >= col_box.x &&
    point.x <= (col_box.x + col_box.width - 1.0) &&
    point.y >= col_box.y &&
    point.y <= (col_box.y + col_box.height - 1.0);
}

pub fn are_boxes_colliding(box1: &Rectangle, box2: &Rectangle) -> bool {
  if (box1.x + box1.width - 1.0) >= box2.x &&    // box1 right edge past box2 left
      box1.x <= box2.x + box2.width - 1.0 &&    // box1 left edge past box2 right
      box1.y + box1.height - 1.0 >= box2.y &&    // box1 top edge past box2 bottom
      box1.y <= box2.y + box2.height - 1.0  {    // box1 bottom edge past box2 top
        return true;
  }
  return false;
}

pub fn detect_body_collisions(world: &mut World) {
    let mut collided_entities: HashMap<Entity, Entity> = HashMap::new();

    {
      let mut query = world.query::<&CollisionBox>().with::<BodyCollision>();
      let mut query_2 = world.query::<&CollisionBox>().with::<BodyCollision>();
      for (ety1, box1) in query.into_iter() {
        for (ety2, box2) in query_2.into_iter() {
          if ety1 != ety2 && are_boxes_colliding(&box1.rect, &box2.rect) {
            collided_entities.insert(ety1, ety2);
            collided_entities.insert(ety2, ety1);
          }
        }
      }
    }

    let query = world.query_mut::<&mut BodyCollision>();
    for (ety, body_collision) in query.into_iter() {
      if collided_entities.contains_key(&ety) {
        body_collision.colliding = true;
        body_collision.other_body = Some(collided_entities[&ety]);
      } else {
        body_collision.colliding = false;
        body_collision.other_body = None;
      }
    }
}

pub fn detect_trigger_collisions(world: &mut World) {
    let mut collided_entities: HashMap<Entity, Entity> = HashMap::new();

    {
      let mut query = world.query::<&CollisionBox>().with::<TriggerCollision>();
      let mut query_2 = world.query::<&CollisionBox>().with::<TriggerCollision>();
      query.into_iter().for_each(|(ety1, box1)| {
        query_2.into_iter().for_each(|(ety2, box2)| {
          if ety1 != ety2 && are_boxes_colliding(&box1.rect, &box2.rect) {
            collided_entities.insert(ety1, ety2);
            collided_entities.insert(ety2, ety1);
          }
        });
      });
    }

    let query = world.query_mut::<&mut TriggerCollision>();
    query.into_iter().for_each(|(ety, trigger_collision)| {
      if collided_entities.contains_key(&ety) {
        trigger_collision.colliding = true;
        trigger_collision.other_trigger = Some(collided_entities[&ety]);
      } else {
        trigger_collision.colliding = false;
        trigger_collision.other_trigger = None;
      }
    });
}

pub fn draw_collisions(world: &mut World, draw_handle: &mut RaylibMode2D<RaylibDrawHandle>, tile_size: f32) {
  let mut draw_query = world.query::<&DrawCollisions>();
  let draw_collisions_enabled: bool;

  {
    draw_collisions_enabled = draw_query.into_iter().len() > 0;
  }

  if draw_collisions_enabled {
    let mut drawn_entities: Vec<Entity> = vec![];
    let query = &mut world.query::<(&BodyCollision, &CollisionBox)>();
    query.into_iter().for_each(|(ety, (col_body, col_box))| {
      let selected_color: Color;
      if col_body.colliding {
          selected_color = Color { r: 230, g: 41, b: 55, a: 170 };
      } else {
          selected_color = Color { r: 0, g: 121, b: 241, a: 170 };
      }

      drawn_entities.push(ety);

      draw_handle.draw_rectangle(
        col_box.rect.x as i32, 
        col_box.rect.y as i32, 
        tile_size as i32, 
        tile_size as i32, 
        selected_color);
    });
    let query = &mut world.query::<(&TriggerCollision, &CollisionBox)>();
    query.into_iter().for_each(|(ety, (col_trigger, col_box))| {
      let mut selected_color: Color = Color::WHITE;
      selected_color.a = 0;
      if col_trigger.colliding {
          selected_color = Color { r: 253, g: 249, b: 0, a: 170 };
      } else {
        if !drawn_entities.contains(&ety) {
          selected_color = Color { r: 0, g: 121, b: 241, a: 170 };
        }
      }

      draw_handle.draw_rectangle(
        col_box.rect.x as i32, 
        col_box.rect.y as i32, 
        tile_size as i32, 
        tile_size as i32, 
        selected_color);
    });
  }
}
