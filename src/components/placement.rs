use specs::{Component, VecStorage, world::Index};

#[derive(Debug, Clone)]
pub struct Placement {
    pub order_entity: Index
}

impl Component for Placement {
    type Storage = VecStorage<Self>;
}