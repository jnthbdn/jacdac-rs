use core::fmt::Debug;
use downcast_rs::{impl_downcast, Downcast};

use super::{
    reports::{ActionReport, EventReport},
    serivce_error::ServiceError,
};

pub trait Service: Debug + Downcast {
    fn handle_event_report(&mut self, event: EventReport) -> Result<(), ServiceError>;
    fn handle_action_report(&mut self, action: ActionReport) -> Result<(), ServiceError>;
}

impl_downcast!(Service);
