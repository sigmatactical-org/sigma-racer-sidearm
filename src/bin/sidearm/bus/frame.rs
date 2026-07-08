//! A received safety-bus CAN frame.

/// A received classic/FD CAN frame (up to 8 bytes of the payload used here).
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub id: u32,
    data: [u8; 8],
    len: usize,
}

impl Frame {
    /// The valid payload bytes for this frame.
    pub fn payload(&self) -> &[u8] {
        &self.data[..self.len]
    }
}
