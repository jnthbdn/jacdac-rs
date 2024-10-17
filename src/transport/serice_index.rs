#[derive(Debug)]
pub enum ServiceIndex {
    ControlService,
    RegularService(u8),
    PipePacket,
    AckPacket,
    Reserved,
}

impl From<u8> for ServiceIndex {
    fn from(value: u8) -> Self {
        if value == 0x00 {
            ServiceIndex::ControlService
        } else if 0x01 <= value && value <= 0x3A {
            ServiceIndex::RegularService(value)
        } else if value == 0x3E {
            ServiceIndex::PipePacket
        } else if value == 0x3F {
            ServiceIndex::AckPacket
        } else {
            ServiceIndex::Reserved
        }
    }
}
