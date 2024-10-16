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

// #[derive(Debug)]
// pub enum FrameFlags {
//     ReportPacket,
//     Command,
//     AckRequested,
//     DeviceIdAltMeaning,
//     Reserved,
// }

// impl From<u8> for FrameFlag {
//     fn from(value: u8) -> Self {
//         if value == 0 {
//             FrameFlag::ReportPacket
//         } else if value & 0x01 > 0 {
//             FrameFlag::Command
//         } else if value & 0x02 > 0 {
//             FrameFlag::AckRequested
//         } else if value & 0x04 > 0 {
//             FrameFlag::DeviceIdAltMeaning
//         } else {
//             FrameFlag::Reserved
//         }
//     }
// }
