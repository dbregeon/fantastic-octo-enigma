use specs::{System, Read, WriteStorage, Entities, World};
use shrev::{EventChannel, ReaderId};
use stamper::Stamper;
use oms::commands::OmsCommand;
use components::sequence::Sequence;

#[derive(Default)]
pub struct Sequencer {
    stamper : Stamper,
    reader_id: Option<ReaderId<OmsCommand>>
}

impl<'a> System<'a> for Sequencer {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (Read<'a, EventChannel<OmsCommand>>, Entities<'a>, WriteStorage<'a, Sequence>, WriteStorage<'a, OmsCommand>);

    fn run(&mut self, (incoming_commands, entities, mut sequences, mut commands): Self::SystemData) {
        for command in incoming_commands.read(self.reader_id.as_mut().unwrap()) {
            let (sequence, command) = self.stamper.stamp(command);
            entities.build_entity().with(Sequence { sequence }, &mut sequences).with(command.clone(), &mut commands).build();
        }
    }

    fn setup(&mut self, world: &mut World) {
        use specs::prelude::SystemData;
        Self::SystemData::setup(world);
        self.reader_id = Option::Some(world.fetch_mut::<EventChannel<OmsCommand>>().register_reader());
    }

}

#[test]
fn creates_an_entity_for_each_command() {
    use specs::{WorldExt, DispatcherBuilder, Join};

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().with(Sequencer::default(), "sequencer", &[]).build();
    let command_channel = EventChannel::<OmsCommand>::new();
    let inbound = OmsCommand::DefineInstrument { id: 1, dml_id: "Test".to_string()};

    world.insert(command_channel);

    dispatcher.setup(&mut world);

    for _ in 1..9 {
        world.fetch_mut::<EventChannel::<OmsCommand>>().single_write(inbound.clone());
    }

    dispatcher.dispatch(&mut world);
    world.maintain();

    let mut count = 0;
    let sequences = world.read_storage::<Sequence>();
    for entity in world.entities().join() {
        count = count + 1;
        assert_eq!(count, sequences.get(entity).unwrap().sequence);
    }
    assert_eq!(8, count);
}

#[test]
fn reads_a_command_and_creates_an_entity_with_sequence_and_itself() {
    use specs::{WorldExt, DispatcherBuilder, Join};

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().with(Sequencer::default(), "sequencer", &[]).build();
    let command_channel = EventChannel::<OmsCommand>::new();
    let inbound = OmsCommand::DefineInstrument { id: 1, dml_id: "Test".to_string()};

    world.insert(command_channel);

    dispatcher.setup(&mut world);

    world.fetch_mut::<EventChannel::<OmsCommand>>().single_write(inbound);

    dispatcher.dispatch(&mut world);
    world.maintain();

    let mut count = 0;
    let sequences = world.read_storage::<Sequence>();
    let commands = world.read_storage::<OmsCommand>();
    for entity in world.entities().join() {
        count = count + 1;
        assert_eq!(count, sequences.get(entity).unwrap().sequence);
        let command = commands.get(entity).unwrap();
        match command {
            OmsCommand::DefineInstrument { id, dml_id } => {
                assert_eq!(1, *id);
                assert_eq!("Test", dml_id);
            },
            _ => assert!(false)
        };
    }
    assert_eq!(1, count, "Too many entities created");
}