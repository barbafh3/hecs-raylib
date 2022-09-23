use hecs::{World, Entity};

use crate::{enums::GameResource, entities::Hauler};

pub struct HaulTask {
    pub origin: i32,
    pub destination: i32,
    pub resource: GameResource,
    pub resource_amount: i32,
    pub delivered_amount: i32
}

pub fn check_finished_haul_tasks(world: &mut World) {
    let query = world.query_mut::<(&Hauler, &mut HaulTask)>();
    let mut removal_list: Vec<Entity> = vec![];
    for (ety, (_, task)) in query.into_iter() {
        if task.delivered_amount < task.resource_amount {
            task.delivered_amount += 1;
            println!(
                "Task has delivered {:?} of {:?} units. Delireving 1 unit.", 
                task.delivered_amount, 
                task.resource_amount);
        } else {
            println!("Finishing delivery...");
            if !removal_list.contains(&ety) {
                removal_list.push(ety);
            }
        }
    }
    for entity in removal_list {
        world.remove_one::<HaulTask>(entity).unwrap();
    }
}
