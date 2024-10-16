#[derive(Debug)]
pub enum TransportError {
    ZeroSizeFrameNotSupported,
    InvalidCRC(u16, u16),
    InvalidFrameSize,

    NotEnougthBufferData,

    InvalidPacketSize,
}
