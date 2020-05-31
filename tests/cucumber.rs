extern crate cucumber_rust as cucumber;
extern crate specs;
extern crate shrev;
extern crate shred;
extern crate fantastic_octo_enigma;


use cucumber::{cucumber, before, after};
use specs::{Dispatcher, World, WorldExt, ReaderId};
use fantastic_octo_enigma::oms::events::OmsEvent;
use fantastic_octo_enigma::components::placement::Placement;

mod market1;
mod market2;


pub struct CukeWorld<'a> {
    ecs: World,
    dispatcher: Option<Dispatcher<'a, 'a>>,
    events_reader_id: Option<ReaderId<OmsEvent>>,
    market1_reader_id: Option<ReaderId<Placement>>
}

impl <'a> cucumber::World for CukeWorld<'a> {}
impl <'a> std::default::Default for CukeWorld<'a>  {
    fn default() -> CukeWorld<'a>  {
        CukeWorld { 
            ecs: World::new(),
            dispatcher: Option::None,
            events_reader_id: Option::None,
            market1_reader_id: Option::None
        }
    }
}

mod requeue_steps {
    use std::str::FromStr;

    use cucumber::steps;
    use specs::{WorldExt, DispatcherBuilder};
    use shred::ResourceId;
    use shrev::EventChannel;

    use fantastic_octo_enigma::components::instrument::Instrument;
    use fantastic_octo_enigma::components::order::Order;
    use fantastic_octo_enigma::components::placement::Placement;
    use fantastic_octo_enigma::acceptor::Acceptor;
    use fantastic_octo_enigma::placer::Placer;
    use fantastic_octo_enigma::sequencer::Sequencer;
    use market1::Market1;
    use market2::Market2;
    use fantastic_octo_enigma::oms::commands::{ OmsCommand, OrderType, TimeInForce };
    use fantastic_octo_enigma::oms::events::{ OmsEvent };
    
    // Any type that implements cucumber::World + Default can be the world
    steps!(::CukeWorld<'static> => {
        given "an OMS with:" |world, step| {
            let table = step.table().unwrap().clone();
            let command_channel = EventChannel::<OmsCommand>::new();
            let notification_channel = EventChannel::<OmsEvent>::new();
            let market1_channel = ResourceId::new_with_dynamic_id::<EventChannel<Placement>>(1);
            let market2_channel = ResourceId::new_with_dynamic_id::<EventChannel<Placement>>(2);
            
            world.ecs.register::<Instrument>();
            world.ecs.register::<Order>();
            world.ecs.register::<Placement>();

            world.ecs.insert(command_channel);
            world.ecs.insert(notification_channel);
            world.ecs.insert_by_id(market1_channel, EventChannel::<Placement>::new());
            world.ecs.insert_by_id(market2_channel, EventChannel::<Placement>::new());

            let mut dispatcher = table.rows.iter()
                .map(|row| row[0].as_ref())
                .fold(DispatcherBuilder::new(), |builder, name| match name {
                    "Sequencer" => builder.with(Sequencer::default(), name, &[]),
                    "Acceptor" => builder.with(Acceptor, name, &["Sequencer"]),
                    "Placer" => builder.with(Placer, name, &["Acceptor"]),
                    "Market 1" => builder.with(Market1, name, &["Placer"]),
                    "Market 2" => builder.with(Market2, name, &[]),
                    _ => builder
            }).build();

            dispatcher.setup(&mut world.ecs);

            world.dispatcher = Option::Some(dispatcher);

            world.events_reader_id = Option::Some(world.ecs.fetch_mut::<EventChannel::<OmsEvent>>().register_reader());
            world.market1_reader_id = world.ecs.try_fetch_mut_by_id::<EventChannel::<Placement>>(ResourceId::new_with_dynamic_id::<EventChannel<Placement>>(1)).map(|mut c| c.register_reader());
        };

        given "the following instrument was received:" |world, step| {
            let table = step.table().unwrap().clone();
            let mut command_channel = world.ecs.fetch_mut::<EventChannel::<OmsCommand>>();
            table.rows.iter().for_each(|row| {
                command_channel.single_write(OmsCommand::DefineInstrument {id: u64::from_str(&row[0]).unwrap(), dml_id: row[1].clone() })
            });
        };

        when "an order is sent:" |world, step| {
            let table = step.table().unwrap().clone();
            let mut command_channel = world.ecs.fetch_mut::<EventChannel::<OmsCommand>>();
            table.rows.iter().for_each(|row| {
                let create_order = OmsCommand::CreateOrder {
                    order_id: row[0].clone(),
                    quantity: u64::from_str(&row[2]).unwrap(),
                    order_type: OrderType::Market,
                    time_in_force: TimeInForce::Day,
                    instrument_id: u64::from_str(&row[1]).unwrap(),
                    market_id: row[4].clone()
                };
                command_channel.single_write(create_order)
            });
        };

        when "the OMS runs one iteration" |world, _step| {
            let dispatcher = world.dispatcher.as_mut().unwrap();
            dispatcher.dispatch(&mut world.ecs);
            world.ecs.maintain();
        };

        then "the OMS sends a Pending notification:" |world, step| {
            let table = step.table().unwrap().clone();
            let notification_channel = world.ecs.fetch_mut::<EventChannel::<OmsEvent>>();
            let mut count = 0;
            let row = &table.rows[0];
            for event in notification_channel.read(world.events_reader_id.as_mut().unwrap()) {
                count = count + 1;
                match event {
                    OmsEvent::Pending {
                        order_id,
                        quantity,
                        // order_type,
                        // time_in_force,
                        // instrument_id,
                        // market_id
                    } => {
                        assert_eq!(row[0], *order_id);
                    },
                    _ => { panic!("Unexpected event received.") }, 
                };

            }
            assert_eq!(1, count);
        };

        then "the OMS sends a Place event to Market 1" |world, step| {
            let market1_channel = ResourceId::new_with_dynamic_id::<EventChannel<Placement>>(1);
            let channel = world.ecs.try_fetch_mut_by_id::<EventChannel<Placement>>(market1_channel).unwrap();
            let mut count = 0;
            for _ in channel.read(world.market1_reader_id.as_mut().unwrap()) {
                count = count + 1;
            }
            assert_eq!(1, count, "Expected one placement");
        };

        then regex r"^we can (.*) rules with regex$" |world, matches, step| {
            // And access them as an array
            assert_eq!(matches[1], "implement");
        };

        then regex r"^we can also match (\d+) (.+) types$" (usize, String) |world, num, word, step| {
            // `num` will be of type usize, `word` of type String
            assert_eq!(num, 42);
            assert_eq!(word, "olika");
        };

        then "we can use data tables to provide more parameters" |world, step| {
            let table = step.table().unwrap().clone();

            assert_eq!(table.header, vec!["key", "value"]);

            let expected_keys = table.rows.iter().map(|row| row[0].to_owned()).collect::<Vec<_>>();
            let expected_values = table.rows.iter().map(|row| row[1].to_owned()).collect::<Vec<_>>();

            assert_eq!(expected_keys, vec!["a", "b"]);
            assert_eq!(expected_values, vec!["fizz", "buzz"]);
        };
    });
}

// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |scenario| {

});

// A setup function to be called before everything else
fn setup() {
    
}

cucumber! {
    features: "./features", // Path to our feature files
    world: ::CukeWorld<'static>, // The world needs to be the same for steps and the main cucumber call
    steps: &[
        requeue_steps::steps // the `steps!` macro creates a `steps` function in a module
    ],
    setup: setup, // Optional; called once before everything
    before: &[
        a_before_fn // Optional; called before each scenario
    ], 
    after: &[
        an_after_fn // Optional; called after each scenario
    ] 
}