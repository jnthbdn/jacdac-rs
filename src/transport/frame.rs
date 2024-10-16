use alloc::vec::Vec;

use crate::error::Error;

use super::{frame_flag::FrameFlags, packet::Packet};

#[derive(Debug)]
pub struct Frame {
    pub crc: u16,
    pub size: u8,
    pub flag: FrameFlags,
    pub device_id: u64,
    pub data: Vec<Packet>,
}

impl Frame {
    pub fn from_buffer(buffer: &[u8]) -> Result<Self, Error> {
        if buffer.len() < 16 {
            return Err(Error::NotEnougthBufferData);
        }

        let crc = u16::from_le_bytes(buffer[0..=1].try_into().unwrap());
        let size = buffer[2];
        let flag = FrameFlags::new(buffer[3]);
        let device_id = u64::from_le_bytes(buffer[4..=11].try_into().unwrap());

        if size == 0 {
            return Err(Error::ZeroSizeFrameNotSupported);
        }

        if size as usize > buffer.len() - 12 {
            return Err(Error::NotEnougthBufferData);
        }

        let mut frame = Frame {
            crc,
            size,
            flag,
            device_id,
            data: Vec::new(),
        };

        let mut start_pos: usize = 12;

        while start_pos < buffer.len() {
            match Packet::from_buffer(&buffer[start_pos..]) {
                Some(packet) => {
                    start_pos += 4 + packet.size as usize;
                    frame.data.push(packet);
                }
                None => break,
            };
        }

        let mut crc_state = crc16::State::<crc16::CCITT_FALSE>::new();
        crc_state.update(&buffer[2..start_pos]);

        let compute_crc = crc_state.get();

        if compute_crc == frame.crc {
            Ok(frame)
        } else {
            Err(Error::InvalidCRC(frame.crc, compute_crc))
        }
    }
}
