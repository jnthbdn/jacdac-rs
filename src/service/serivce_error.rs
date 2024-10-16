#[derive(Debug)]
pub enum ServiceError {
    /// The service_command is not supported. The `str` help to contextualize error
    UnsupportedServiceCommand(&'static str),

    /// A payload is needed with this request
    NoPayloadInRequest,

    /// The payload doesn't contains enought data. The first parameter is the payload size, the second paramter is the expected size.
    InvalidPayloadSize(usize, usize),

    /// The event code is unknown to the service
    UnknownEventCode,
}
