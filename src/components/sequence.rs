use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Sequence {
    pub sequence: u32
}

impl Component for Sequence {
    type Storage = VecStorage<Self>;
}