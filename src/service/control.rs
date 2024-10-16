use alloc::vec::Vec;

use super::serivce_error::ServiceError;

#[derive(Debug)]
pub struct ControlReport {
    flags: u16,
    packet_count: u8,
    classes: Vec<u32>,
}

impl ControlReport {
    pub fn from_buffer(buffer: &[u8]) -> Result<Self, ServiceError> {
        if buffer.len() < 8 {
            return Err(ServiceError::NotEnougthBufferData);
        }

        Ok(Self {
            flags: u16::from_le_bytes(buffer[0..=1].try_into().unwrap()),
            packet_count: buffer[2],
            classes: Self::buffer_to_classes(&buffer[4..]),
        })
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
