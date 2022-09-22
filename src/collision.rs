use hecs::World;
use raylib::prelude::*;

// STRUCTS ------
pub struct CollisionBox(pub(crate) Rectangle);
pub struct BodyCollision;
pub struct DetectionCollision;

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
    let mut query = world.query::<(&BodyCollision, &CollisionBox)>();
    let mut query_2 = world.query::<(&BodyCollision, &CollisionBox)>();
    for (ety1, (_, CollisionBox(box1))) in query.into_iter() {
      for (ety2, (_, CollisionBox(box2))) in query_2.into_iter() {
        if ety1 != ety2 && are_boxes_colliding(box1, box2) {
          warn!("Collision!!");
        }
      }
    }
}
