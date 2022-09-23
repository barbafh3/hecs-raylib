use hecs::{World, Entity};
use raylib::prelude::*;

use crate::constants::TILE_SIZE;

// TAGS ------
pub struct DrawCollisions;

// STRUCTS ------
pub struct BodyCollision {
  pub colliding: bool
}
pub struct TriggerCollision {
  pub colliding: bool
}
pub struct CollisionBox {
  pub rect: Rectangle
}

// FUNCTIONS ------
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
    let mut collided_entities: Vec<Entity> = vec![];

    {
      let mut query = world.query::<&CollisionBox>().with::<BodyCollision>();
      let mut query_2 = world.query::<&CollisionBox>().with::<BodyCollision>();
      for (ety1, box1) in query.into_iter() {
        for (ety2, box2) in query_2.into_iter() {
          if ety1 != ety2 && are_boxes_colliding(&box1.rect, &box2.rect) {
            warn!("Body Collision!!");
            collided_entities.push(ety1);
            collided_entities.push(ety2);
          }
        }
      }
    }

    let query = world.query_mut::<&mut BodyCollision>();
    for (ety, body_collision) in query.into_iter() {
      body_collision.colliding = collided_entities.contains(&ety);
    }
}

pub fn detect_trigger_collisions(world: &mut World) {
    let mut collided_entities: Vec<Entity> = vec![];

    {
      let mut query = world.query::<&CollisionBox>().with::<TriggerCollision>();
      let mut query_2 = world.query::<&CollisionBox>().with::<TriggerCollision>();
      for (ety1, box1) in query.into_iter() {
        for (ety2, box2) in query_2.into_iter() {
          if ety1 != ety2 && are_boxes_colliding(&box1.rect, &box2.rect) {
            collided_entities.push(ety1);
            collided_entities.push(ety2);
          }
        }
      }
    }

    let query = world.query_mut::<&mut TriggerCollision>();
    for (ety, trigger_collision) in query.into_iter() {
      trigger_collision.colliding = collided_entities.contains(&ety);
    }
}

pub fn draw_collisions(world: &mut World, draw_handle: &mut RaylibMode2D<RaylibDrawHandle>) {
  let mut draw_query = world.query::<&DrawCollisions>();
  let draw_collisions_enabled: bool;

  {
    draw_collisions_enabled = draw_query.into_iter().len() > 0;
  }

  if draw_collisions_enabled {
    for (_, (col_body, col_box)) in &mut world.query::<(&BodyCollision, &CollisionBox)>() {
        if col_body.colliding {
        draw_handle.draw_rectangle(
          col_box.rect.x as i32, 
          col_box.rect.y as i32, 
          TILE_SIZE as i32, 
          TILE_SIZE as i32, 
          Color { r: 230, g: 41, b: 55, a: 170 });
      }
    }
    for (_, (col_trigger, col_box)) in &mut world.query::<(&TriggerCollision, &CollisionBox)>() {
        if col_trigger.colliding {
        draw_handle.draw_rectangle(
          col_box.rect.x as i32, 
          col_box.rect.y as i32, 
          TILE_SIZE as i32, 
          TILE_SIZE as i32, 
          Color { r: 253, g: 249, b: 0, a: 170 });
      }
    }
  }
}
