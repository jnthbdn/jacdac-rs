use crate::service::serivce_error::ServiceError;

#[derive(Debug)]
pub enum BrainError {
    /// The service index is not valid (e.g. there no ith service), the first parameter is the index_service, the second is the number of services
    InvalidServiceIndex(u8, usize),

    /// The service index is not supported the `str` help to contextualize the error.
    UnsupportedSericeIndex(&'static str),

    /// The service class is invalid (e.g. Control class, is not allow)
    InvalidServiceClass,

    /// The service class is not supported
    NotSupportedServiceClass,

    /// The command packet is not supported. (e.g A brain, or brain device, should not received a command packet.)
    WrongPacketType,

    /// A service return an error. The parameter is the service error.
    ServiceError(ServiceError),
}
