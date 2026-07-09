//! Fail-operational **supervisor**: watchdog and load-shed anchor.

use embassy_time::Instant;

static mut LOOP_COUNT: u32 = 0;
static mut LAST_KICK_MS: u32 = 0;

/// Periodic supervisor service, called once per control-loop iteration (5 ms).
pub fn service() {
    unsafe {
        LOOP_COUNT = LOOP_COUNT.wrapping_add(1);
        LAST_KICK_MS = Instant::now().as_millis() as u32;
    }
    // TODO: feed SoC WDOG3 once the kick register sequence is validated on silicon.
}

/// Milliseconds since the last successful supervisor kick.
pub fn millis_since_kick() -> u32 {
    unsafe {
        let now = Instant::now().as_millis() as u32;
        now.saturating_sub(LAST_KICK_MS)
    }
}

/// Control-loop iterations since boot (diagnostic).
pub fn loop_count() -> u32 {
    unsafe { LOOP_COUNT }
}

/// Enter a safe, non-returning fault state.
pub fn fault_loop() -> ! {
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}
