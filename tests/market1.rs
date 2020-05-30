use specs::{System, ReadStorage, World, Join};
use shrev::EventChannel;
use shred::{ Write, ResourceId, SystemData };

use fantastic_octo_enigma::components::placement::Placement;

#[derive(Default)]
pub struct Market1;

pub struct Market1SystemData<'a> {
    queue: Option<Write<'a, EventChannel<Placement>>>
}

impl <'a> SystemData<'a> for Market1SystemData<'a> {
    fn setup(world: &mut World) {
    }

    fn fetch(world: &'a World) -> Self {
        Self {
            queue: world.try_fetch_mut_by_id::<EventChannel<Placement>>(ResourceId::new_with_dynamic_id::<EventChannel<Placement>>(1)).map(Into::into),
        }
    }

    fn reads() -> Vec<ResourceId> {
        Vec::new()
    }
    
    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new_with_dynamic_id::<EventChannel<Placement>>(1)]
    }
}

impl<'a> System<'a> for Market1 {
    type SystemData = (ReadStorage<'a, Placement>, Market1SystemData<'a>);

    fn run(&mut self, (placements, market1): Self::SystemData) {
        match market1.queue {
            Some(mut queue) => {
                for placement in (&placements).join() {
                    queue.single_write(placement.clone());
                }
            },
            None => {}
        };
    }
}