use alloc::vec::Vec;
// use circular_buffer::CircularBuffer;

use crate::transport::frame::Frame;

use super::{brain_device::BrainDevice, brain_error::BrainError};

pub const BRAIN_EVENT_QUEUE_SIZE: usize = 16;

#[derive(Debug, Default)]
pub struct Brain {
    pub known_devices: Vec<BrainDevice>,
    // pub event_queue: CircularBuffer<BRAIN_EVENT_QUEUE_SIZE, EventReport>,
}

impl Brain {
    pub fn handle_frame(&mut self, frame: Frame) -> Result<(), BrainError> {
        match self.get_device_index_by_id(frame.device_id) {
            Some(idx) => {
                self.known_devices[idx].handle_frame(frame)?;
            }
            None => {
                let mut device = BrainDevice::new(frame.device_id);
                device.handle_frame(frame)?;
                self.known_devices.push(device);
            }
        };
        Ok(())
    }

    fn get_device_index_by_id(&self, id: u64) -> Option<usize> {
        self.known_devices.iter().position(|x| x.id() == id)
    }

    pub fn remove_inactive_devices(&mut self) {
        self.known_devices.retain(|e| e.is_active());
    }
}
