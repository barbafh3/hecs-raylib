use hecs::{World, Entity};
use raylib::prelude::*;

use crate::{
    game::{
        tasks::HaulTask, 
        constants::{HAULER_CAPACITY, TILE_SIZE, DEFAULT_IDLE_POINT}
    }, 
    engine::{
        enums::{VillagerState, CollisionType}, 
        datatypes::Sprite, 
        collision::{CollisionBox, BodyCollision, TriggerCollision},
    }
};

use super::{
    datatypes::{GameItem, Backpack, LoadingState, CarryingState, IdleInfo, IdleState, Hauler}, 
    step::move_villager
};


pub fn spawn_hauler(
    world: &mut World, 
    position: Vector2, 
    atlas_tile: Vector2, 
    collision_type: CollisionType,
    opt_idle_point: Option<Vector2>
) -> Entity {
    let sprite = Sprite::new(
        position,
        atlas_tile,
        TILE_SIZE
    );
    let rect = Rectangle {
        x: position.x,
        y: position.y,
        width: TILE_SIZE,
        height: TILE_SIZE
    };
    let idle_point: Vector2;

    match opt_idle_point {
        Some(point) => idle_point = point,
        None => idle_point = DEFAULT_IDLE_POINT
    }

    let hauler: Entity = world.spawn((
        Hauler, 
        IdleInfo::default(idle_point),
        IdleState,
        Backpack::default(),
        sprite, 
        CollisionBox {
            rect
    }));

    match collision_type {
        CollisionType::Body => {
            world.insert_one(hauler, BodyCollision::default()).unwrap();
        },
        CollisionType::Trigger => {
            world.insert_one(hauler, TriggerCollision::new()).unwrap();
        },
        CollisionType::All => {
            world.insert_one(
                hauler, 
                (
                    BodyCollision::default(), 
                    TriggerCollision::new()
                )
            ).unwrap();
        }
    }

    return hauler;
}


pub fn update_loading_state(world: &mut World, delta: f32) {
    let origin_missing_haulers: Vec<Entity> = vec![];

    let query = world.query_mut::<(&HaulTask, &mut Sprite)>().with::<LoadingState>();
    query.into_iter().for_each(|(_, (task, sprite))| {
        if let Some(target_position) = task.origin_position {
            if (target_position - sprite.position).length() > 1.0 {
                move_villager(target_position, sprite, delta);
            }
        }
    });

    origin_missing_haulers.into_iter().for_each(|hauler| {
        LoadingState::change_state_to(world, hauler, VillagerState::Idle);
    });
}

pub fn update_carrying_state(world: &mut World, delta: f32) {
    {
        let query = world.query_mut::<(&mut HaulTask, &mut Sprite)>().with::<CarryingState>();
        query.into_iter().for_each(|(_, (task, sprite))| {
            if let Some(target_position) = task.destination_position {
                if (target_position - sprite.position).length() > 1.0 {
                    move_villager(target_position, sprite, delta);
                }
            }
        });
    }
}

pub fn receive_resource(world: &mut World, hauler: Entity) -> Option<GameItem> {
    let mut m_item: Option<GameItem> = None;

    {
        let result = world.get::<HaulTask>(hauler);
        if let Ok(task) = result {
            m_item = Some(GameItem::new(task.resource, HAULER_CAPACITY));
        }
    }
    {
        let backpack = Backpack{ item: m_item.clone() };
        world.exchange_one::<Backpack, Backpack>(hauler, backpack).unwrap();
        LoadingState::change_state_to(world, hauler, VillagerState::Carrying);
    }

    return m_item;
}

pub fn deliver_resource(world: &mut World, hauler: Entity) -> Option<GameItem> {
    let mut m_item: Option<GameItem> = None;

    {
        let result = world.get::<HaulTask>(hauler);
        if let Ok(task) = result {
            m_item = Some(GameItem::new(task.resource, HAULER_CAPACITY));
        }
    }
    {
        let backpack = Backpack{ item: None };
        world.exchange_one::<Backpack, Backpack>(hauler, backpack).unwrap();
        world.remove_one::<HaulTask>(hauler).unwrap();
        CarryingState::change_state_to(world, hauler, VillagerState::Idle);
    }

    return m_item;
}
