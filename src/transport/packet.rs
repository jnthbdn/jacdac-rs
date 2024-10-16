use alloc::vec::Vec;

use super::{serice_index::ServiceIndex, service_command::ServiceCommand};

#[derive(Debug)]
pub struct Packet {
    pub size: u8,
    pub index: ServiceIndex,
    pub command: ServiceCommand,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn from_buffer(buffer: &[u8]) -> Option<Self> {
        if buffer.len() < 4 {
            return None;
        }

        let size = buffer[0];
        let index = ServiceIndex::from(buffer[1]);
        let command = ServiceCommand::from(u16::from_le_bytes(buffer[2..=3].try_into().unwrap()));

        if size as usize > buffer.len() - 4 {
            return None;
        }

        let payload = &buffer[4..(4 + size as usize)];

        Some(Self {
            size,
            index,
            command,
            payload: payload.into(),
        })
    }
}
