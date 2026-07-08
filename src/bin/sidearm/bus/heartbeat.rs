//! Fail-operational **heartbeat** to the ECU.
//!
//! The M7 emits a periodic liveness frame so the ECU knows the safety core is
//! alive even while the A53/Linux side is rebooting, updating (RAUC A/B swap),
//! or crashed. **STUB** — timing and the real frame layout are not wired yet.

use super::safety_bus::SafetyBus;

/// Periodic ECU heartbeat emitter.
#[derive(Default)]
pub struct Heartbeat {
    sequence: u8,
}

impl Heartbeat {
    pub fn new() -> Self {
        Self::default()
    }

    /// Emit the heartbeat frame for this cycle.
    pub fn tick(&mut self, bus: &mut SafetyBus) {
        self.sequence = self.sequence.wrapping_add(1);
        // TODO: rate-limit to the agreed heartbeat period (timer-driven) and use
        // the real heartbeat message ID/layout from the shared dictionary.
        bus.transmit(0x000, &[self.sequence]);
    }
}
