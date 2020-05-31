use specs::{Component, VecStorage};
use oms::commands::OmsCommand;
use oms::events::OmsEvent;

pub mod sequence;
pub mod placement;
pub mod order;
pub mod instrument;
pub mod markers;

impl Component for OmsCommand {
    type Storage = VecStorage<Self>;
}

impl Component for OmsEvent {
    type Storage = VecStorage<Self>;
}
