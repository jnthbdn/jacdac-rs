use alloc::vec::Vec;

use super::{
    reports::{ActionReport, EventReport},
    serivce_error::ServiceError,
    service::Service,
};

#[derive(Debug, Default)]
pub struct Control {
    flags: u16,
    classes: Vec<u32>,
}

impl Control {
    pub fn classes(&self) -> &Vec<u32> {
        &self.classes
    }

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
    // fn id(pub static BUTTON_ID: u32 = 0x00;

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

        Ok(())
    }
}
