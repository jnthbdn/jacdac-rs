use super::{
    reports::{ActionReport, EventReport},
    serivce_error::ServiceError,
    service::Service,
};

#[derive(Debug, Default, Clone, Copy)]
pub enum ButtonState {
    #[default]
    Release,
    Press,
}

#[derive(Debug, Default)]
pub struct Button {
    state: ButtonState,
    last_press_time: u32,
}

impl Button {
    pub fn state(&self) -> ButtonState {
        self.state
    }

    pub fn last_press_time(&self) -> u32 {
        self.last_press_time
    }
}

impl Service for Button {
    fn handle_event_report(&mut self, event: EventReport) -> Result<(), ServiceError> {
        match event.code {
            0x01 => self.state = ButtonState::Press,

            0x02 => match event.payload {
                Some(payload) => {
                    if payload.len() < 4 {
                        return Err(ServiceError::InvalidPayloadSize(payload.len(), 4));
                    } else {
                        self.last_press_time =
                            u32::from_le_bytes(payload[0..4].try_into().unwrap());
                        self.state = ButtonState::Release;
                    }
                }
                None => return Err(ServiceError::InvalidPayloadSize(0, 4)),
            },

            0x81 => match event.payload {
                Some(payload) => {
                    if payload.len() < 4 {
                        return Err(ServiceError::InvalidPayloadSize(payload.len(), 4));
                    } else {
                        let _time = u32::from_le_bytes(payload[0..4].try_into().unwrap());
                        // self.state = ButtonState::Release;
                    }
                }
                None => return Err(ServiceError::InvalidPayloadSize(0, 4)),
            },

            _ => return Err(ServiceError::UnknownEventCode),
        };
        Ok(())
    }

    fn handle_action_report(&mut self, _action: ActionReport) -> Result<(), ServiceError> {
        unimplemented!()
    }
}
