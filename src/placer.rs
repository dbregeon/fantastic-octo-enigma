use specs::{System, WriteStorage, ReadStorage, Join, Entities};
use components::order::Order;
use components::placement::Placement;
use components::markers::PlacementPending;
use components::markers::{New, Pending};

pub struct Placer;

impl<'a> System<'a> for Placer {
    type SystemData = (Entities<'a>, ReadStorage<'a, Order>, WriteStorage<'a, New>, WriteStorage<'a, Pending>, WriteStorage<'a, Placement>, WriteStorage<'a, PlacementPending>);

    fn run(&mut self, (entity_storage, order_storage, mut new_storage, mut pending_storage, mut placement_storage, mut placement_pending_storage): Self::SystemData) {
        for (entity, _order, _) in (&entity_storage, &order_storage, &mut new_storage).join() {
            entity_storage.build_entity()
                .with(Placement { order_entity: entity.id()}, &mut placement_storage)
                .with(PlacementPending, &mut placement_pending_storage)
                .build();
            pending_storage.insert(entity, Pending);
        }
        (&entity_storage, &pending_storage).join().for_each(|(e,_)| {
            new_storage.remove(e);
        });
    } 
}
