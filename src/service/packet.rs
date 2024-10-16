use crate::{
    error::Error,
    transport::{self, frame_flag::FrameFlags, serice_index::ServiceIndex},
};

use super::{control_report::ControlReport, event_report::EventReport, packet_type::{CommandType, PacketType, ReportType}};

#[derive(Debug)]
pub struct Packet {
    pub service_index: ServiceIndex,
    pub packet_type: PacketType,
}

impl Packet {
    pub fn from_transport(
        packet: transport::packet::Packet,
        flags: FrameFlags,
    ) -> Result<Self, Error> {
        let packet_type: PacketType;

        if flags.is_command() {
            let command_type = match packet.command {
                transport::service_command::ServiceCommand::Action(value) => {
                    Ok(CommandType::Action(value & 0x0FFF))
                }
                transport::service_command::ServiceCommand::RegisterRead(value) => {
                    Ok(CommandType::ReadRegister(value & 0x0FFF))
                }
                transport::service_command::ServiceCommand::RegisterWrite(value) => {
                    Ok(CommandType::WriteRegister(value & 0x0FFF, packet.payload))
                }
                transport::service_command::ServiceCommand::Reserved(_) => Err(
                    Error::UnsupportedServiceCommand("Reserved service command is not supported"),
                ),
                transport::service_command::ServiceCommand::Events(_) => {
                    Err(Error::UnsupportedServiceCommand(
                        "Event service command (when command flag present) is not supported",
                    ))
                }
            }?;
            packet_type = PacketType::Command(command_type);
        } else {
            let report_type = match packet.command {
                transport::service_command::ServiceCommand::Action(value) => {
                    match packet.index {
                        ServiceIndex::ControlService => Ok(ReportType::Control(ControlReport::from_buffer(&packet.payload)?)),
                        _ => Ok(ReportType::Actions(value & 0x0FFF, packet.payload))
                    }
                    
                }
                transport::service_command::ServiceCommand::RegisterRead(value) => {
                    Ok(ReportType::Register(value & 0x0FFF, packet.payload))
                }
                transport::service_command::ServiceCommand::Events(value) => Ok(
                    ReportType::Events(EventReport{
                        counter: ((value & 0x7F00) >> 8) as u8,
                        code: (value & 0x00FF) as u8,
                        payload: if packet.payload.is_empty() { None } else { Some(packet.payload) }
                    }
                )),
                transport::service_command::ServiceCommand::RegisterWrite(_) => Err(
                    Error::UnsupportedServiceCommand("Register Write service commmand (when command flag not present) is not supported"),
                ),
                transport::service_command::ServiceCommand::Reserved(_) => Err(
                    Error::UnsupportedServiceCommand("Reserved service command is not supported"),
                ),
            }?;
            packet_type = PacketType::Report(report_type);
        }

        Ok(Self {
            service_index: packet.index,
            packet_type,
        })
    }
}
