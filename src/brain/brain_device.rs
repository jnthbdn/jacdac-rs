use core::fmt::Debug;

use alloc::{boxed::Box, vec::Vec};

use super::brain_error::BrainError;
use crate::{
    service::{self, button::Button, control::Control, packet::Packet, service::Service},
    transport::frame::Frame,
};

#[derive(Debug)]
pub struct BrainDevice {
    id: u64,
    services: Vec<Box<dyn Service>>,
    last_control_report: u64,
}

impl BrainDevice {
    pub fn new(id: u64) -> Self {
        let mut v: Vec<Box<dyn Service>> = Vec::with_capacity(1);
        v.push(Box::new(Control::default()));

        Self {
            id,
            services: v,
            last_control_report: 0,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn is_active(&self) -> bool {
        self.last_control_report < 2000
    }

    pub fn get_all_services<T: Service>(&self) -> Vec<&T> {
        self.services
            .iter()
            .filter(|x| x.downcast_ref::<T>().is_some())
            .map(|x| x.downcast_ref::<T>().unwrap())
            .collect()
    }

    pub fn get_first_service<T: Service>(&self) -> Option<&T> {
        self.services
            .iter()
            .find(|x| x.downcast_ref::<T>().is_some())
            .map(|e| e.downcast_ref().unwrap())
    }

    pub fn handle_frame(
        &mut self,
        frame: Frame,
        get_current_millis: &fn() -> u64,
    ) -> Result<(), BrainError> {
        for packet in frame.data {
            let service_packet = Packet::from_transport(packet, &frame.flag)
                .map_err(|e| BrainError::ServiceError(e))?;

            let report_type = match service_packet.packet_type {
                crate::service::packet_type::PacketType::Command(_) => {
                    Err(BrainError::WrongPacketType)
                }
                crate::service::packet_type::PacketType::Report(report_type) => Ok(report_type),
            }?;

            self.last_control_report = get_current_millis();

            match service_packet.service_index {
                crate::transport::serice_index::ServiceIndex::ControlService => match report_type {
                    crate::service::packet_type::ReportType::Register(_, _vec) => {
                        Err(BrainError::WrongPacketType)
                    }
                    crate::service::packet_type::ReportType::Events(_) => {
                        Err(BrainError::WrongPacketType)
                    }
                    crate::service::packet_type::ReportType::Actions(report) => {
                        self.services[0]
                            .handle_action_report(report)
                            .map_err(|e| BrainError::ServiceError(e))?;
                        self.update_services()?;
                        Ok(())
                    }
                },
                crate::transport::serice_index::ServiceIndex::RegularService(index) => {
                    if index as usize >= self.services.len() {
                        return Err(BrainError::InvalidServiceIndex(index, self.services.len()));
                    }
                    match report_type {
                        crate::service::packet_type::ReportType::Register(_, _vec) => {
                            Err(BrainError::WrongPacketType)
                        }
                        crate::service::packet_type::ReportType::Events(report) => self.services
                            [index as usize]
                            .handle_event_report(report)
                            .map_err(|e| BrainError::ServiceError(e)),
                        crate::service::packet_type::ReportType::Actions(report) => self.services
                            [index as usize]
                            .handle_action_report(report)
                            .map_err(|e| BrainError::ServiceError(e)),
                    }
                }
                crate::transport::serice_index::ServiceIndex::PipePacket => Err(
                    BrainError::UnsupportedSericeIndex("Pipe are not implemented"),
                ),
                crate::transport::serice_index::ServiceIndex::AckPacket => Err(
                    BrainError::UnsupportedSericeIndex("Ack are not implemented"),
                ),
                crate::transport::serice_index::ServiceIndex::Reserved => {
                    Err(BrainError::UnsupportedSericeIndex("Reserved service"))
                }
            }?;
        }

        Ok(())
    }

    fn update_services(&mut self) -> Result<(), BrainError> {
        let control: &Control = self.services[0].downcast_ref().unwrap();

        let mut tmp: Vec<Box<dyn Service>> = Vec::with_capacity(control.classes().len());

        if self.services.len() == 1 {
            for class in control.classes() {
                match class {
                    &service::CONTROL_ID => return Err(BrainError::InvalidServiceClass),
                    &service::BUTTON_ID => tmp.push(Box::new(Button::default())),
                    _ => return Err(BrainError::NotSupportedServiceClass),
                }
            }
        }

        self.services.append(&mut tmp);
        Ok(())
    }
}
