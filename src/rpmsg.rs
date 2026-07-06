//! **RPMsg / OpenAMP** gateway to the A53 Linux side.
//!
//! Hands digested, timestamped vehicle state up to Linux so `vehicle-service`
//! consumes a clean stream instead of racing the raw bus. **STUB** — the
//! virtio/RPMsg endpoint is not implemented yet.

use sigma_racer_wingman_m7_can::M7Signals;

/// RPMsg endpoint publishing state to the Linux cluster.
#[derive(Default)]
pub struct RpmsgLink {
    // TODO: hold the RPMsg channel / virtio ring endpoint.
}

impl RpmsgLink {
    pub fn new() -> Self {
        // TODO: create the rpmsg endpoint (rpmsg-lite / OpenAMP) and announce it.
        Self::default()
    }

    /// Publish the latest vehicle state to Linux.
    pub fn publish(&mut self, _state: &M7Signals) {
        // TODO: serialize `state` and enqueue it on the rpmsg channel to
        // vehicle-service (which already supports a second, non-safety bus).
    }
}
