use specs::{System, ReadStorage};
use fantastic_octo_enigma::components::placement::Placement;

pub struct Market2;

impl<'a> System<'a> for Market2 {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (ReadStorage<'a, Placement>);

    fn run(&mut self, (_placement): Self::SystemData) {
        // The `.join()` combines multiple component storages,
        // so we get access to all entities which have
        // both a position and a velocity.
        // for (pos, vel) in (&mut pos, &vel).join() {
        //     pos.0 += vel.0;
        // }
    }
}