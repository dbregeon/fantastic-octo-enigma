use specs::{Component, VecStorage, NullStorage};

/// Marker Component for a Placement waiting for acceptance from a Venue.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct PlacementPending;

/// Marker Component for a Placement accepted by a Venue.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Accepted;

/// Marker Component for an Order that was placed to a Venue
#[derive(Debug, Clone)]
pub struct Placed {
}

impl Component for Placed {
    type Storage = VecStorage<Self>;
}

/// Marker Component for an Order which Placement was not accepted yet by a Venue
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Pending;

/// Marker Component for an Order which was just received
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct New;


