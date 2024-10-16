#[derive(Debug)]
pub enum ServiceError {
    UnsupportedServiceCommand(&'static str),
    NotEnougthBufferData,

    InvalidPayloadSize(u32, u32),
    UnknownEventCode,
}
