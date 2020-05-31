
#[derive(Default)]
pub struct Stamper {
    sequence: u32,
}

impl Stamper {
    pub fn stamp<T>(&mut self, input: T) -> (u32, T) {
        self.sequence += 1;

        (self.sequence, input)
    }
}

#[test]
fn stamp_increases_the_sequence_number() {
    use oms::commands::OmsCommand;

    let in_bound = OmsCommand::DefineInstrument { id: 1, dml_id: "Test".to_string()};
    let mut tested_stamper = Stamper::default();
    for i in 1..9 {
        tested_stamper.stamp(in_bound.clone());
        assert_eq!(i, tested_stamper.sequence);
    }
}

#[test]
fn stamp_sets_the_sequence_number_on_the_event() {
    use oms::commands::OmsCommand;

    let in_bound = OmsCommand::DefineInstrument { id: 1, dml_id: "Test".to_string()};
    let mut tested_stamper = Stamper::default();
    for i in 1..9 {
        assert_eq!(i, tested_stamper.stamp(in_bound.clone()).0);
    }
}