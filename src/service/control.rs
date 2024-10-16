use alloc::vec::Vec;

use super::{
    reports::{ActionReport, EventReport},
    serivce_error::ServiceError,
    service::Service,
};

// #[derive(Debug)]
// pub struct ControlReport {
//     flags: u16,
//     packet_count: u8,
//     classes: Vec<u32>,
// }

// impl ControlReport {
//     pub fn from_buffer(buffer: &[u8]) -> Result<Self, ServiceError> {
//         if buffer.len() < 8 {
//             return Err(ServiceError::InvalidPayloadSize(buffer.len(), 8));
//         }

//         Ok(Self {
//             flags: u16::from_le_bytes(buffer[0..=1].try_into().unwrap()),
//             packet_count: buffer[2],
//             classes: Self::buffer_to_classes(&buffer[4..]),
//         })
//     }
// }

#[derive(Debug, Default)]
struct Control {
    flags: u16,
    classes: Vec<u32>,
    last_report: u64,
}

impl Control {
    fn buffer_to_classes(buffer: &[u8]) -> Vec<u32> {
        let mut result = Vec::with_capacity(buffer.len() / 4);

        for i in (0..(buffer.len())).step_by(4) {
            if i + 4 > buffer.len() {
                break;
            }

            result.push(u32::from_le_bytes(buffer[i..(i + 4)].try_into().unwrap()));
        }

        result
    }
}

impl Service for Control {
    fn id(&self) -> u64 {
        0x0
    }

    fn handle_event_report(&mut self, _event: EventReport) -> Result<(), ServiceError> {
        unimplemented!()
    }

    fn handle_action_report(&mut self, action: ActionReport) -> Result<(), ServiceError> {
        if action.code != 0 {
            return Err(ServiceError::UnknownEventCode);
        }

        if action.payload.len() < 8 {
            return Err(ServiceError::InvalidPayloadSize(action.payload.len(), 8));
        }

        self.flags = u16::from_le_bytes(action.payload[0..=1].try_into().unwrap());
        // packet_count: buffer[2],
        self.classes = Self::buffer_to_classes(&action.payload[4..]);

        // TODO: How to get current "time"
        // self.last_report = now;

        Ok(())
    }
}