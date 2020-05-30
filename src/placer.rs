use specs::{System, WriteStorage, ReadStorage, Join, Entities};
use components::order::Order;
use components::placement::Placement;

pub struct Placer;

impl<'a> System<'a> for Placer {
    type SystemData = (Entities<'a>, ReadStorage<'a, Order>, WriteStorage<'a, Placement>);

    fn run(&mut self, (entity_storage, order_storage, mut placement_storage): Self::SystemData) {
        for (entity, _order) in (&entity_storage, &order_storage).join() {
            match placement_storage.insert(entity, Placement {}) {
                Ok(None) => println!("Did it"),
                _ => {}
            }
        }
    }
}
