use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Order {

}

impl Component for Order {
    type Storage = VecStorage<Self>;
}