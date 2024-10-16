#[derive(Debug)]
pub enum ServiceCommand {
    Action(u16),
    RegisterRead(u16),
    RegisterWrite(u16),
    Reserved(u16),
    Events(u16),
}

impl From<u16> for ServiceCommand {
    fn from(value: u16) -> Self {
        match value & 0xF000 {
            0x0000 => ServiceCommand::Action(value),
            0x1000 => ServiceCommand::RegisterRead(value),
            0x2000 => ServiceCommand::RegisterWrite(value),
            0x3000 | 0x4000 | 0x5000 | 0x6000 | 0x7000 => ServiceCommand::Reserved(value),
            _ => ServiceCommand::Events(value),
        }
    }
}
