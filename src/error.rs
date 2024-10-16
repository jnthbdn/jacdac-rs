#[derive(Debug)]
pub enum Error {
    ZeroSizeFrameNotSupported,
    InvalidCRC(u16, u16),
    InvalidFrameSize,

    NotEnougthBufferData,

    InvalidPacketSize,

    UnsupportedServiceCommand(&'static str),
}
