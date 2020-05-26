use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Placement {

}

impl Component for Placement {
    type Storage = VecStorage<Self>;
}