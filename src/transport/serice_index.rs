#[derive(Debug)]
pub enum ServiceIndex {
    ControlService,
    RegularService,
    PipePacket,
    AckPacket,
    Reserved,
}

impl From<u8> for ServiceIndex {
    fn from(value: u8) -> Self {
        if value == 0x00 {
            ServiceIndex::ControlService
        } else if 0x01 <= value && value <= 0x3A {
            ServiceIndex::RegularService
        } else if value == 0x3E {
            ServiceIndex::PipePacket
        } else if value == 0x3F {
            ServiceIndex::AckPacket
        } else {
            ServiceIndex::Reserved
        }
    }
}
