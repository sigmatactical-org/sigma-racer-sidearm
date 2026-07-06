//! CAN-FD **safety bus** endpoint to the ECU (`sigma-racer-efi`).
//!
//! The M7 owns this controller: it RX/TX-es the safety subset of the vehicle
//! bus with tight timing and hardware timestamping. **STUB** — the FlexCAN-FD
//! driver is not implemented yet; `poll` never yields and `transmit` is a no-op.

use sigma_racer_wingman_m7_can::MESSAGE_IDS;

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

/// Owns the M7's safety-bus CAN-FD controller.
#[derive(Default)]
pub struct SafetyBus {
    // TODO: hold the FlexCAN-FD peripheral (via a PAC) and its RX/TX mailboxes.
}

impl SafetyBus {
    pub fn new() -> Self {
        // TODO: configure CAN-FD (e.g. 1 Mbit/s nominal, higher data phase),
        // install acceptance filters for `MESSAGE_IDS`, enable timestamping.
        let _ = MESSAGE_IDS;
        Self::default()
    }

    /// Non-blocking receive: returns the next queued frame, if any.
    pub fn poll(&mut self) -> Option<Frame> {
        // TODO: pop from the RX FIFO / mailbox.
        None
    }

    /// Queue a frame for transmission on the safety bus.
    pub fn transmit(&mut self, _id: u32, _payload: &[u8]) {
        // TODO: write into a TX mailbox.
    }
}
