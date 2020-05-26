use std::convert::TryFrom;

pub enum OmsEventType {
    Pending,
    Accepted,
    Replaced,
    Cancelled
}

impl TryFrom<u16> for OmsEventType {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == OmsEventType::Pending as u16 => Ok(OmsEventType::Pending),
            x if x == OmsEventType::Accepted as u16 => Ok(OmsEventType::Accepted),
            x if x == OmsEventType::Replaced as u16 => Ok(OmsEventType::Replaced),
            x if x == OmsEventType::Cancelled as u16 => Ok(OmsEventType::Cancelled),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum OmsEvent {
    Pending { order_id: String, quantity: u64 },
    Accepted { order_id: String },
    Replaced { order_id: String },
    Cancelled { order_id: String }
}
