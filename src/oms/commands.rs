use std::convert::TryFrom;

pub enum OmsCommandType {
    DefineInstrument = 10000,
    CreateOrder      = 11000,
    ReplaceOrder,
    CancelOrder
}

impl TryFrom<u16> for OmsCommandType {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == OmsCommandType::DefineInstrument as u16 => Ok(OmsCommandType::DefineInstrument),
            x if x == OmsCommandType::CreateOrder as u16 => Ok(OmsCommandType::CreateOrder),
            x if x == OmsCommandType::ReplaceOrder as u16 => Ok(OmsCommandType::ReplaceOrder),
            x if x == OmsCommandType::CancelOrder as u16 => Ok(OmsCommandType::CancelOrder),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum OrderType {
    Market,
    Limit(u32),
    Stop(u32),
    StopLimit(u32, u32)
}

#[derive(PartialEq, Copy, Clone)]
pub enum TimeInForce {
    Day,
    GoodTilCancel,
    GoodTilDate(u64)
}

#[derive(PartialEq, Clone)]
pub enum OmsCommand {
    DefineInstrument { id: u64, dml_id: String},
    CreateOrder { order_id: String, quantity: u64, order_type: OrderType, time_in_force: TimeInForce, instrument_id: u64, market_id: String },
    ReplaceOrder { order_id: String, quantity: u64, order_type: OrderType, time_in_force: TimeInForce },
    CancelOrder { order_id: String }
}

