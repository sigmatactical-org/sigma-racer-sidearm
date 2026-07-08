//! Fail-operational **supervisor**: watchdog and load-shed anchor.
//!
//! The M7 is the priority anchor of the platform — it keeps the safety bus and
//! ECU heartbeat alive and defers/sheds non-essential A53 loads under low
//! voltage. **STUB** — watchdog and load management are not implemented yet.

/// Periodic supervisor service, called once per control-loop iteration.
pub fn service() {
    // TODO: feed the M7 watchdog (IWDG-equivalent) and evaluate low-voltage
    // load-shedding priorities.
}

/// Enter a safe, non-returning fault state.
///
/// Used for unrecoverable boot errors (e.g. a corrupt embedded dictionary).
/// Drives outputs to a known-safe state, then idles the core.
pub fn fault_loop() -> ! {
    // TODO: force any safety-relevant outputs to their safe state first.
    loop {
        cortex_m::asm::wfi();
    }
}
