use specs::{System, WriteStorage, ReadStorage, Join, Write, Entities};
use shrev::EventChannel;
use components::order::Order;
use components::sequence::Sequence;

use oms::commands::OmsCommand;
use oms::events::OmsEvent;

pub struct Acceptor;

impl<'a> System<'a> for Acceptor {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (Entities<'a>, Write<'a, EventChannel<OmsEvent>>, ReadStorage<'a, Sequence>, ReadStorage<'a, OmsCommand>, WriteStorage<'a, Order>);

    fn run(&mut self, (mut entities, mut notification_channel, sequence, command, mut order): Self::SystemData) {
        for (_sequence, command) in (&sequence, &command).join() {
            match command {
                OmsCommand::CreateOrder {order_id, quantity, order_type, time_in_force, instrument_id, market_id} =>  {
                    let entity = entities.create();
                    match order.insert(entity, Order {}) {
                        Ok(None) => {
                            let pending = OmsEvent::Pending { 
                                order_id: order_id.clone(),
                                quantity: *quantity
                            };
                            notification_channel.single_write(pending);
                        },
                        _ => {}
                    }
                    
                },
                _ => {}
            }
        }
    }
}

#[test]
fn sends_a_pending_event_on_order_pending() {
    use specs::{WorldExt, DispatcherBuilder, World, Builder};
    use oms::commands::{OmsCommand::CreateOrder, OrderType, TimeInForce};
    use components::instrument::Instrument;

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().with(Acceptor, "acceptor", &[]).build();
    let notification_channel = EventChannel::<OmsEvent>::new();

    world.register::<Sequence>();
    world.register::<OmsCommand>();
    world.register::<Instrument>();

    let create_order = CreateOrder {
        order_id: "Test".to_string(),
        quantity: 1,
        order_type: OrderType::Market,
        time_in_force: TimeInForce::Day,
        instrument_id: 1,
        market_id: "Mkt".to_string()
    };
    
    world.create_entity().with(Sequence{ sequence: 1}).with(create_order).build();
    world.create_entity().with(Instrument {id: 1}).build();
    world.insert(notification_channel);

    let reader_id = &mut world.fetch_mut::<EventChannel<OmsEvent>>().register_reader();

    dispatcher.setup(&mut world);

    dispatcher.dispatch(&mut world);
    world.maintain();

    let mut count = 0;
    for event in world.fetch_mut::<EventChannel::<OmsEvent>>().read(reader_id) {
        count = count + 1;
        match event {
            OmsEvent::Pending {order_id, quantity} => {
                assert_eq!("Test".to_string(), *order_id, "Incorrect order id");
                assert_eq!(1, *quantity, "Incorrect quantity");
            },
            _ => {},
        }
    }
    assert_eq!(1, count, "Too many entities created");
}

#[test]
fn creates_an_order_entity_on_order_pending() {
    use specs::{WorldExt, DispatcherBuilder, Join, World, Builder};
    use oms::commands::{OmsCommand::CreateOrder, OrderType, TimeInForce};

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().with(Acceptor, "acceptor", &[]).build();
    let notification_channel = EventChannel::<OmsEvent>::new();

    world.register::<Sequence>();
    world.register::<OmsCommand>();

    let create_order = CreateOrder {
        order_id: "Test".to_string(),
        quantity: 1,
        order_type: OrderType::Market,
        time_in_force: TimeInForce::Day,
        instrument_id: 1,
        market_id: "Mkt".to_string()
    };

    world.insert(notification_channel);
    world.create_entity().with(Sequence{ sequence: 1}).with(create_order).build();

    dispatcher.setup(&mut world);

    dispatcher.dispatch(&mut world);
    world.maintain();

    let mut count = 0;
    let orders = world.read_storage::<Order>();
    for _ in orders.join() {
        count = count + 1;
    }
    assert_eq!(1, count);
}

#[test]
fn does_not_create_an_order_entity_on_other_commands() {
    use specs::{WorldExt, DispatcherBuilder, Builder, World, Join};

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().with(Acceptor, "acceptor", &[]).build();
    let notification_channel = EventChannel::<OmsEvent>::new();

    world.register::<Sequence>();
    world.register::<OmsCommand>();
    world.register::<Order>();

    world.insert(notification_channel);
    world.create_entity().with(Sequence{ sequence: 1}).with(OmsCommand::DefineInstrument { id: 1, dml_id: "Test".to_string() }).build();

    dispatcher.setup(&mut world);

    dispatcher.dispatch(&mut world);
    world.maintain();

    let orders = world.read_storage::<Order>();
    assert_eq!(0, orders.join().collect::<Vec<&Order>>().len());
}

#[test]
fn does_not_send_an_event_on_other_commands() {
    use specs::{WorldExt, DispatcherBuilder, Builder, World};

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().with(Acceptor, "acceptor", &[]).build();
    let notification_channel = EventChannel::<OmsEvent>::new();

    world.register::<Sequence>();
    world.register::<OmsCommand>();

    world.insert(notification_channel);
    world.create_entity().with(Sequence{ sequence: 1}).with(OmsCommand::DefineInstrument { id: 1, dml_id: "Test".to_string() }).build();

    dispatcher.setup(&mut world);

    dispatcher.dispatch(&mut world);
    world.maintain();

    let reader_id = &mut world.fetch_mut::<EventChannel<OmsEvent>>().register_reader();
    assert_eq!(0, world.fetch_mut::<EventChannel::<OmsEvent>>().read(reader_id).len());
}
