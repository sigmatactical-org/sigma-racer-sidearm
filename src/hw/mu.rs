//! MU1 mailbox doorbell for virtio RPMsg kicks (A53 ↔ M7).

use super::reg::{self, MU1_BASE};

/// MU transmit register for channel 0 (M7 → A53 kick).
const MU_TR0: usize = MU1_BASE + 0x20;
/// MU control/status register for channel 0.
const MU_CSR0: usize = MU1_BASE + 0x10;
/// Transmit empty flag in CSR.
const MU_CSR_TE: u32 = 1 << 23;

/// Notify the A53 that the TX virtio vring has a new buffer.
pub fn kick_host() {
    while reg::read(MU_CSR0) & MU_CSR_TE == 0 {}
    reg::write(MU_TR0, 0);
}
