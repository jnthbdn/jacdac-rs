use alloc::vec::Vec;

use super::reports::{ActionReport, EventReport};

#[derive(Debug)]
pub enum PacketType {
    Command(CommandType),
    Report(ReportType),
}

#[derive(Debug)]
pub enum CommandType {
    Action(u16),
    ReadRegister(u16),
    WriteRegister(u16, Vec<u8>),
}

#[derive(Debug)]
pub enum ReportType {
    Register(u16, Vec<u8>),
    Actions(ActionReport),
    Events(EventReport),
}
