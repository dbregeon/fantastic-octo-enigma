use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct Placement {

}

impl Component for Placement {
    type Storage = VecStorage<Self>;
}