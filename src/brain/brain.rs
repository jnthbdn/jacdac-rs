use alloc::vec::Vec;

use crate::transport::frame::Frame;

use super::{brain_device::BrainDevice, brain_error::BrainError};

pub const BRAIN_EVENT_QUEUE_SIZE: usize = 16;

#[derive(Debug)]
pub struct Brain {
    get_current_millis: fn() -> u64,
    known_devices: Vec<BrainDevice>,
}

impl Brain {
    pub fn new(get_current_millis: fn() -> u64) -> Self {
        Self {
            get_current_millis,
            known_devices: Vec::new(),
        }
    }

    pub fn handle_frame(&mut self, frame: Frame) -> Result<(), BrainError> {
        match self.get_device_index_by_id(frame.device_id) {
            Some(idx) => {
                self.known_devices[idx].handle_frame(frame, &self.get_current_millis)?;
            }
            None => {
                let mut device = BrainDevice::new(frame.device_id);
                device.handle_frame(frame, &self.get_current_millis)?;
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

    pub fn get_devices(&self) -> &[BrainDevice] {
        &self.known_devices
    }

    pub fn get_device_by_id(&self, id: u64) -> Option<&BrainDevice> {
        self.known_devices.iter().find(|x| x.id() == id)
    }
}
