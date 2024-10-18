#[derive(Debug)]
pub struct FrameFlags {
    raw: u8,
}

impl FrameFlags {
    pub fn new(raw: u8) -> Self {
        Self { raw }
    }

    pub fn is_command(&self) -> bool {
        self.raw & 0x01 > 0
    }

    pub fn is_report(&self) -> bool {
        self.raw & 0x01 == 0
    }

    pub fn is_ack_requested(&self) -> bool {
        self.raw & 0x02 > 0
    }
}
