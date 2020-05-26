use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Instrument {
    pub id: u64
}

impl Component for Instrument {
    type Storage = VecStorage<Self>;
}