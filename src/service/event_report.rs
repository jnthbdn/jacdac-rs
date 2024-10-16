use alloc::vec::Vec;

#[derive(Debug)]
pub struct EventReport {
    pub counter: u8,
    pub code: u8,
    pub payload: Option<Vec<u8>>,
}
